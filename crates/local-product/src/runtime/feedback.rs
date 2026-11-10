//! Physician feedback on Coda's responses in hosted chats.
//!
//! A response is one of Coda's saved replies in a chat, named by the run that
//! published it ([`RunId`]), as [`Conversation::responses`] reads it. One row
//! per rated response holds the rating and, once saved, the details. Details
//! never exist without a rating: a details save updates an existing row only.
//! A save holds the auth owner's locks for the physician's session in its own
//! transaction, so a revoked session or a patient taken off the panel writes
//! nothing. Feedback is clinical working text and is never deleted
//! automatically.
use std::collections::{BTreeMap, BTreeSet};

use sea_orm::{
    ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
    sea_query::{Expr, OnConflict},
};
use serde::{Deserialize, Serialize};

use super::entities::{conversation, response_feedback};
use super::{Authority, Conversation, RunId, Scope, Store, error, keys, now};

/// The longest details text a save accepts, in characters.
pub const MAX_DETAILS_CHARS: usize = 4000;

/// A physician's rating of one response.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Rating {
    /// A good response.
    Up,
    /// A poor response.
    Down,
}

impl Rating {
    fn column(self) -> &'static str {
        match self {
            Self::Up => "up",
            Self::Down => "down",
        }
    }

    fn parse(column: &str) -> Result<Self, DbErr> {
        match column {
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err(error("corrupt feedback rating")),
        }
    }
}

/// A reason the physician picked in the details.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Reason {
    /// The response says something that isn't so.
    IncorrectInformation,
    /// The response leaves out something it needed.
    MissingContext,
    /// The response is hard to follow.
    HardToFollow,
    /// The response doesn't help.
    NotUseful,
}

/// The optional details on a rating: reasons and the physician's words.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Details {
    reasons: BTreeSet<Reason>,
    text: String,
}

impl Details {
    /// Details with these reasons and words, or `None` when the words are
    /// longer than [`MAX_DETAILS_CHARS`].
    pub fn new(reasons: BTreeSet<Reason>, text: String) -> Option<Self> {
        (text.chars().count() <= MAX_DETAILS_CHARS).then_some(Self { reasons, text })
    }

    /// The reasons picked.
    pub fn reasons(&self) -> &BTreeSet<Reason> {
        &self.reasons
    }

    /// The physician's words.
    pub fn text(&self) -> &str {
        &self.text
    }
}

/// Saved feedback on one response.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Feedback {
    /// The latest rating.
    pub rating: Rating,
    /// Database-clock seconds of the latest rating.
    pub rated_at: i64,
    /// The latest saved details, if any were saved.
    pub details: Option<SavedDetails>,
}

/// Details as saved, with when.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SavedDetails {
    /// The details.
    pub details: Details,
    /// Database-clock seconds of the save.
    pub saved_at: i64,
}

/// Outcome of a rating save.
#[derive(Debug, PartialEq, Eq)]
pub enum RatingSaved {
    /// Saved; the feedback as it now stands.
    Saved(Feedback),
    /// That run published no response in this chat; nothing was written.
    NotAResponse,
    /// The auth owner refused the session or panel; nothing was written.
    Refused,
}

/// Outcome of a details save.
#[derive(Debug, PartialEq, Eq)]
pub enum DetailsSaved {
    /// Saved; the feedback as it now stands.
    Saved(Feedback),
    /// The response has no rating to add details to; nothing was written.
    NotRated,
    /// The auth owner refused the session or panel; nothing was written.
    Refused,
}

/// One feedback row with what an evaluator needs beside it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Evaluated {
    /// The chat, and the physician who wrote the feedback.
    pub scope: Scope,
    /// The run that published the rated response.
    pub run: RunId,
    /// The feedback.
    pub feedback: Feedback,
    /// The rated response's text; `None` if the chat no longer holds it.
    pub response_text: Option<String>,
}

fn nonempty(scope: &Scope) -> Result<(), DbErr> {
    if [
        &scope.practice,
        &scope.physician,
        &scope.workspace,
        &scope.chat,
    ]
    .iter()
    .any(|s| s.trim().is_empty())
    {
        return Err(error("empty physician scope"));
    }
    Ok(())
}

fn feedback(row: &response_feedback::Model) -> Result<Feedback, DbErr> {
    let details = match (&row.details, row.details_saved_at) {
        (None, None) => None,
        (Some(stored), Some(saved_at)) => Some(SavedDetails {
            details: serde_json::from_value(stored.clone())
                .map_err(|cause| unconvertible("stored feedback details", "Details", cause))?,
            saved_at,
        }),
        _ => return Err(error("feedback details without their save time")),
    };
    Ok(Feedback {
        rating: Rating::parse(&row.rating)?,
        rated_at: row.rated_at,
        details,
    })
}

/// A value that would not convert between its stored and typed forms,
/// keeping the conversion's own error as the source.
fn unconvertible(
    from: &'static str,
    into: &'static str,
    cause: impl std::error::Error + Send + Sync + 'static,
) -> DbErr {
    DbErr::TryIntoErr {
        from,
        into,
        source: std::sync::Arc::new(cause),
    }
}

fn response_key(scope: &Scope, run: &RunId) -> (String, String, String, String, String) {
    let (practice, physician, workspace, chat) = keys(scope);
    (
        practice,
        physician,
        workspace,
        chat,
        run.as_str().to_owned(),
    )
}

impl Store {
    /// This physician's feedback on one chat's responses, by run.
    pub async fn chat_feedback(&self, scope: &Scope) -> Result<BTreeMap<RunId, Feedback>, DbErr> {
        use response_feedback::Column as C;
        nonempty(scope)?;
        response_feedback::Entity::find()
            .filter(C::Practice.eq(scope.practice.as_str()))
            .filter(C::Physician.eq(scope.physician.as_str()))
            .filter(C::Workspace.eq(scope.workspace.as_str()))
            .filter(C::Chat.eq(scope.chat.as_str()))
            .all(&self.db)
            .await?
            .iter()
            .map(|row| Ok((RunId::from(row.run.clone()), feedback(row)?)))
            .collect()
    }

    /// Rates the response `run` published in this physician's chat. A repeat
    /// converges; the other rating replaces it; details stay. Answers
    /// [`RatingSaved::NotAResponse`] unless `run` published one of the chat's
    /// saved responses. The auth owner's locks for `session_hash` are held in
    /// the same transaction.
    pub async fn rate_response(
        &self,
        scope: &Scope,
        run: &RunId,
        rating: Rating,
        session_hash: &[u8],
        authority: &mut dyn Authority,
    ) -> Result<RatingSaved, DbErr> {
        use response_feedback::Column as C;
        nonempty(scope)?;
        if session_hash.len() != 32 {
            return Err(error("invalid session reference"));
        }
        let tx = self.db.begin().await?;
        if authority
            .lock_publication(&tx, scope, Some(session_hash))
            .await
            .is_err()
        {
            tx.rollback().await?;
            return Ok(RatingSaved::Refused);
        }
        // A saved response and the run that published it never change, so
        // this unlocked read cannot see the answer withdrawn before the commit.
        let conversation: Option<Conversation> = conversation::Entity::find_by_id(keys(scope))
            .one(&tx)
            .await?
            .map(|row| serde_json::from_value(row.state))
            .transpose()
            .map_err(|cause| unconvertible("stored conversation", "Conversation", cause))?;
        if conversation
            .as_ref()
            .and_then(|c| c.response(run))
            .is_none()
        {
            return Ok(RatingSaved::NotAResponse);
        }
        let now = now(&tx).await?;
        response_feedback::Entity::insert(response_feedback::ActiveModel {
            practice: Set(scope.practice.clone()),
            physician: Set(scope.physician.clone()),
            workspace: Set(scope.workspace.clone()),
            chat: Set(scope.chat.clone()),
            run: Set(run.as_str().to_owned()),
            rating: Set(rating.column().into()),
            rated_at: Set(now),
            details: Set(None),
            details_saved_at: Set(None),
        })
        .on_conflict(
            OnConflict::columns([C::Practice, C::Physician, C::Workspace, C::Chat, C::Run])
                .update_columns([C::Rating, C::RatedAt])
                .to_owned(),
        )
        .exec_without_returning(&tx)
        .await?;
        let row = response_feedback::Entity::find_by_id(response_key(scope, run))
            .one(&tx)
            .await?
            .ok_or_else(|| error("missing feedback"))?;
        let saved = feedback(&row)?;
        tx.commit().await?;
        Ok(RatingSaved::Saved(saved))
    }

    /// Replaces the details on the response `run` published. Answers
    /// [`DetailsSaved::NotRated`] when that response has no rating. The auth
    /// owner's locks for `session_hash` are held in the same transaction.
    pub async fn save_feedback_details(
        &self,
        scope: &Scope,
        run: &RunId,
        details: &Details,
        session_hash: &[u8],
        authority: &mut dyn Authority,
    ) -> Result<DetailsSaved, DbErr> {
        use response_feedback::Column as C;
        nonempty(scope)?;
        if session_hash.len() != 32 {
            return Err(error("invalid session reference"));
        }
        let stored = serde_json::to_value(details)
            .map_err(|cause| unconvertible("Details", "stored feedback details", cause))?;
        let tx = self.db.begin().await?;
        if authority
            .lock_publication(&tx, scope, Some(session_hash))
            .await
            .is_err()
        {
            tx.rollback().await?;
            return Ok(DetailsSaved::Refused);
        }
        let now = now(&tx).await?;
        let written = response_feedback::Entity::update_many()
            .col_expr(C::Details, Expr::val(stored))
            .col_expr(C::DetailsSavedAt, Expr::val(now))
            .filter(C::Practice.eq(scope.practice.as_str()))
            .filter(C::Physician.eq(scope.physician.as_str()))
            .filter(C::Workspace.eq(scope.workspace.as_str()))
            .filter(C::Chat.eq(scope.chat.as_str()))
            .filter(C::Run.eq(run.as_str()))
            .exec(&tx)
            .await?
            .rows_affected;
        if written == 0 {
            return Ok(DetailsSaved::NotRated);
        }
        let row = response_feedback::Entity::find_by_id(response_key(scope, run))
            .one(&tx)
            .await?
            .ok_or_else(|| error("missing feedback"))?;
        let saved = feedback(&row)?;
        tx.commit().await?;
        Ok(DetailsSaved::Saved(saved))
    }

    /// Every feedback row, each with its response's text, for evaluation by
    /// Coda staff. Ordered by practice, physician, workspace and chat, then by
    /// each response's place in its chat; a row whose chat no longer holds its
    /// response comes last in that chat.
    pub async fn feedback_for_evaluation(&self) -> Result<Vec<Evaluated>, DbErr> {
        use response_feedback::Column as C;
        let rows = response_feedback::Entity::find()
            .order_by_asc(C::Practice)
            .order_by_asc(C::Physician)
            .order_by_asc(C::Workspace)
            .order_by_asc(C::Chat)
            .order_by_asc(C::Run)
            .all(&self.db)
            .await?;
        // Rows arrive grouped by chat, so each conversation is read once.
        let mut chat: Option<(Scope, Option<Conversation>)> = None;
        let mut evaluated = Vec::with_capacity(rows.len());
        for row in rows {
            let scope = Scope {
                practice: row.practice.clone(),
                physician: row.physician.clone(),
                workspace: row.workspace.clone(),
                chat: row.chat.clone(),
            };
            if chat.as_ref().is_none_or(|(read, _)| *read != scope) {
                chat = Some((scope.clone(), self.read(&scope).await?));
            }
            let run = RunId::from(row.run.clone());
            let response = chat
                .as_ref()
                .and_then(|(_, conversation)| conversation.as_ref())
                .and_then(|c| c.response(&run));
            let place = response.as_ref().map_or(usize::MAX, |r| r.place);
            let response_text = response.map(|r| r.text.to_owned());
            evaluated.push((
                place,
                Evaluated {
                    scope,
                    run,
                    feedback: feedback(&row)?,
                    response_text,
                },
            ));
        }
        // Chats keep the database's order; within each, responses sort by place.
        for one_chat in evaluated.chunk_by_mut(|(_, a), (_, b)| a.scope == b.scope) {
            one_chat.sort_by_key(|(place, _)| *place);
        }
        Ok(evaluated.into_iter().map(|(_, row)| row).collect())
    }
}
