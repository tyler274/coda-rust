//! Linear tickets, their parents and their sub-tickets, through Linear's GraphQL API.
//!
//! The key comes from `LINEAR_API_KEY`, a personal API key. Linear also names
//! each ticket's Git branch and its GitHub pull request attachments, which is how
//! worktrees and pull requests find their ticket before falling back to the key
//! written in a branch name.

use std::collections::BTreeMap;

use serde::Deserialize;
use thiserror::Error;

use crate::event::{StatusCategory, TicketStatus, TimelineEvent};
use crate::key::TicketKey;

const ENDPOINT: &str = "https://api.linear.app/graphql";

const QUERY: &str = r"
query Ticket($id: String!) {
  issue(id: $id) {
    ...Fields
    parent { ...Fields }
    children(first: 100) { nodes { ...Fields } }
  }
}
fragment Fields on Issue {
  identifier
  title
  url
  branchName
  state { name type }
  assignee { name }
  attachments(first: 50) { nodes { url } }
}
";

/// Why Linear could not be read.
#[derive(Debug, Error)]
pub enum LinearError {
    #[error("LINEAR_API_KEY is not set; create a personal API key in Linear's settings")]
    NoKey,
    #[error("asking Linear for {ticket}")]
    Request {
        ticket: String,
        #[source]
        source: reqwest::Error,
    },
    #[error("Linear answered {ticket} with errors: {messages}")]
    Answer { ticket: String, messages: String },
    #[error("Linear has no ticket {0}")]
    Missing(String),
}

/// A ticket as the timeline needs it, with what joins it to branches and pull requests.
#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub key: TicketKey,
    pub parent: Option<TicketKey>,
    pub title: String,
    pub status: TicketStatus,
    pub assignee: Option<String>,
    pub url: String,
    pub branch: Option<String>,
    pub pull_requests: Vec<String>,
}

impl Ticket {
    /// The timeline item for this ticket.
    pub fn event(&self) -> TimelineEvent {
        TimelineEvent::Ticket {
            key: self.key.clone(),
            parent: self.parent.clone(),
            title: self.title.clone(),
            status: self.status.clone(),
            assignee: self.assignee.clone(),
            url: self.url.clone(),
        }
    }
}

#[derive(Deserialize)]
struct Response {
    data: Option<Data>,
    #[serde(default)]
    errors: Vec<GraphError>,
}

#[derive(Deserialize)]
struct GraphError {
    message: String,
}

#[derive(Deserialize)]
struct Data {
    issue: Option<Issue>,
}

#[derive(Deserialize)]
struct Issue {
    #[serde(flatten)]
    fields: Fields,
    parent: Option<Fields>,
    children: Option<Nodes<Fields>>,
}

#[derive(Deserialize)]
struct Nodes<T> {
    nodes: Vec<T>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Fields {
    identifier: String,
    title: String,
    url: String,
    branch_name: Option<String>,
    state: State,
    assignee: Option<Person>,
    attachments: Option<Nodes<Attachment>>,
}

#[derive(Deserialize)]
struct State {
    name: String,
    #[serde(rename = "type")]
    kind: String,
}

#[derive(Deserialize)]
struct Person {
    name: String,
}

#[derive(Deserialize)]
struct Attachment {
    url: String,
}

fn category(kind: &str) -> StatusCategory {
    match kind {
        "triage" => StatusCategory::Triage,
        "backlog" => StatusCategory::Backlog,
        "started" => StatusCategory::Started,
        "completed" => StatusCategory::Completed,
        "canceled" | "duplicate" => StatusCategory::Canceled,
        _ => StatusCategory::Unstarted,
    }
}

fn ticket(fields: Fields, parent: Option<TicketKey>) -> Option<Ticket> {
    Some(Ticket {
        key: TicketKey::parse(&fields.identifier)?,
        parent,
        title: fields.title,
        status: TicketStatus {
            name: fields.state.name,
            category: category(&fields.state.kind),
        },
        assignee: fields.assignee.map(|person| person.name),
        url: fields.url,
        branch: fields.branch_name,
        pull_requests: fields
            .attachments
            .map(|nodes| {
                nodes
                    .nodes
                    .into_iter()
                    .map(|attachment| attachment.url)
                    .filter(|url| url.contains("/pull/"))
                    .collect()
            })
            .unwrap_or_default(),
    })
}

/// Reads one Linear answer: the ticket, its parent and its sub-tickets. The parent
/// comes back without its own parent, which the timeline does not need.
pub fn parse_answer(requested: &str, body: &str) -> Result<Vec<Ticket>, LinearError> {
    let response: Response = serde_json::from_str(body).map_err(|error| LinearError::Answer {
        ticket: requested.to_owned(),
        messages: error.to_string(),
    })?;
    if !response.errors.is_empty() {
        return Err(LinearError::Answer {
            ticket: requested.to_owned(),
            messages: response
                .errors
                .into_iter()
                .map(|error| error.message)
                .collect::<Vec<_>>()
                .join("; "),
        });
    }
    let issue = response
        .data
        .and_then(|data| data.issue)
        .ok_or_else(|| LinearError::Missing(requested.to_owned()))?;
    let parent = issue.parent.and_then(|fields| ticket(fields, None));
    let parent_key = parent.as_ref().map(|parent| parent.key.clone());
    let main = ticket(issue.fields, parent_key)
        .ok_or_else(|| LinearError::Missing(requested.to_owned()))?;
    let main_key = main.key.clone();
    let mut found: Vec<Ticket> = parent.into_iter().collect();
    found.push(main);
    found.extend(
        issue
            .children
            .map(|children| children.nodes)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|fields| ticket(fields, Some(main_key.clone()))),
    );
    Ok(found)
}

/// Asks Linear for each requested ticket. Tickets that came back are kept even when
/// others failed; each failure is returned beside them.
pub fn fetch(requested: &[TicketKey]) -> (Vec<Ticket>, Vec<LinearError>) {
    let Some(key) = std::env::var("LINEAR_API_KEY")
        .ok()
        .filter(|key| !key.is_empty())
    else {
        return (Vec::new(), vec![LinearError::NoKey]);
    };
    let client = reqwest::blocking::Client::new();
    let mut tickets: BTreeMap<TicketKey, Ticket> = BTreeMap::new();
    let mut errors = Vec::new();
    for wanted in requested {
        let id = wanted.to_string();
        let answer = client
            .post(ENDPOINT)
            .header("Authorization", &key)
            .json(&serde_json::json!({ "query": QUERY, "variables": { "id": id } }))
            .send()
            .and_then(reqwest::blocking::Response::error_for_status)
            .and_then(reqwest::blocking::Response::text);
        let body = match answer {
            Ok(body) => body,
            Err(source) => {
                errors.push(LinearError::Request { ticket: id, source });
                continue;
            }
        };
        match parse_answer(&id, &body) {
            Ok(found) => {
                for ticket in found {
                    // A ticket's own answer knows its parent; an answer where it
                    // appears as a parent does not, so never overwrite a parent with none.
                    match tickets.get(&ticket.key) {
                        Some(known) if known.parent.is_some() && ticket.parent.is_none() => {}
                        _ => {
                            tickets.insert(ticket.key.clone(), ticket);
                        }
                    }
                }
            }
            Err(error) => errors.push(error),
        }
    }
    (tickets.into_values().collect(), errors)
}
