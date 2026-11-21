//! Read-only Discord archive inspection. No credentials or network access are needed.
use crate::failure::Failure;
use serde::Deserialize;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

const USAGE: &str = "discord inspect --backup PATH\ndiscord search --backup PATH [--author ID] [--text TEXT]\nSearch explicitly prints matching archived messages; inspect prints counts only.";

#[derive(Debug, thiserror::Error)]
#[error("Discord ID must be a nonzero canonical unsigned integer string")]
struct InvalidId;

fn validate_id(value: String) -> Result<String, InvalidId> {
    match value.parse::<u64>() {
        Ok(number) if number != 0 && number.to_string() == value => Ok(value),
        _ => Err(InvalidId),
    }
}

macro_rules! discord_id {
    ($name:ident) => {
        #[derive(Deserialize, serde::Serialize, PartialEq, Eq)]
        #[serde(try_from = "String", into = "String")]
        #[derive(Clone)]
        struct $name(String);

        impl TryFrom<String> for $name {
            type Error = InvalidId;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                validate_id(value).map(Self)
            }
        }
        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}
discord_id!(AuthorId);
discord_id!(MessageId);
discord_id!(ChannelId);

#[derive(Deserialize)]
struct Author {
    id: AuthorId,
}

#[derive(Deserialize)]
struct Message {
    id: MessageId,
    channel_id: ChannelId,
    author: Author,
    timestamp: String,
    content: String,
}

pub fn from_args(args: &[String]) -> Result<(), Failure> {
    let Some((command, options)) = args.split_first() else {
        return Err(Failure::new(2, USAGE));
    };
    if command != "inspect" && command != "search" {
        return Err(Failure::new(2, USAGE));
    }
    let mut backup = None;
    let mut author = None;
    let mut text = None;
    let (pairs, remainder) = options.as_chunks::<2>();
    for pair in pairs {
        let slot = match pair[0].as_str() {
            "--backup" => &mut backup,
            "--author" if command == "search" => &mut author,
            "--text" if command == "search" => &mut text,
            _ => return Err(Failure::new(2, USAGE)),
        };
        if slot.replace(pair[1].as_str()).is_some() {
            return Err(Failure::new(2, "duplicate Discord option"));
        }
    }
    if !remainder.is_empty() {
        return Err(Failure::new(2, USAGE));
    }
    let backup = backup.ok_or_else(|| Failure::new(2, USAGE))?;
    if command == "search" && author.is_none() && text.is_none() {
        return Err(Failure::new(2, "search requires --author or --text"));
    }
    let author = author
        .map(|value| AuthorId::try_from(value.to_owned()))
        .transpose()
        .map_err(|error| Failure::with_source(2, "invalid author ID", error))?;
    let count = scan(Path::new(backup), author.as_ref(), text, |message| {
        if command == "search" {
            // JSON escapes control characters in untrusted archived text.
            println!(
                "{}",
                serde_json::json!({
                    "id": message.id,
                    "channel_id": message.channel_id,
                    "author_id": message.author.id,
                    "timestamp": message.timestamp,
                    "content": message.content,
                })
            );
        }
    })?;
    println!("records={count}");
    Ok(())
}

fn scan(
    path: &Path,
    author: Option<&AuthorId>,
    text: Option<&str>,
    mut matched: impl FnMut(&Message),
) -> Result<u64, Failure> {
    let file = File::open(path)
        .map_err(|error| Failure::with_source(1, "cannot open Discord archive", error))?;
    let mut count = 0;
    for (index, line) in BufReader::new(file).lines().enumerate() {
        let line =
            line.map_err(|error| Failure::with_source(1, "cannot read Discord archive", error))?;
        let message: Message = serde_json::from_str(&line).map_err(|error| {
            Failure::with_source(
                1,
                format!("invalid archive record at line {}", index + 1),
                error,
            )
        })?;
        if author.is_none_or(|id| &message.author.id == id)
            && text.is_none_or(|query| message.content.contains(query))
        {
            matched(&message);
            count += 1;
        }
    }
    Ok(count)
}
