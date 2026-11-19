//! Physician feedback on Coda's responses in hosted chats.
//!
//! A response is one of Coda's published replies in a chat, named by its
//! [`ResponseId`], as [`Conversation::responses`] reads it. One row per rated
//! response holds the rating and any details, and one save replaces both, so
//! details never exist without a rating. Every save increments the row's
//! revision, so a reader keeping the highest revision it has seen never shows
//! an older answer over a newer one. A save holds the auth owner's locks for
//! the physician's session in its own transaction, so a revoked session or a
//! patient taken off the panel writes nothing. Feedback is clinical working
//! text and is never deleted automatically.
use std::collections::{BTreeMap, BTreeSet};

use sea_orm::{
    ColumnTrait, DbErr, EntityTrait, ExprTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
    sea_query::{Expr, OnConflict},
};
use serde::{Deserialize, Serialize};

use super::control;
use super::entities::{conversation, reply_draft, reply_revision, response_feedback};
use super::reply::proposals::{CopiedReplySource, tables as reply_tables};
use super::{
    Authority, Conversation, ResponseId, Scope, Store, conversion_error, error, keys, now,
};

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
    /// The latest details, if the latest save had any.
    pub details: Option<Details>,
    /// Database-clock seconds of the latest save.
    pub saved_at: i64,
    /// 1 on the first save, one more on each later save.
    pub revision: u64,
}

/// Outcome of a feedback save.
#[derive(Debug, PartialEq, Eq)]
pub enum FeedbackSaved {
    /// Saved; the feedback as it now stands.
    Saved(Feedback),
    /// The chat holds no published response by that id; nothing was written.
    NotAResponse,
    /// The auth owner refused the session or panel; nothing was written.
    Refused,
}

/// One feedback row with what an evaluator needs beside it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Evaluated {
    /// The chat, and the physician who wrote the feedback.
    pub scope: Scope,
    /// The rated response.
    pub response: ResponseId,
    /// The feedback.
    pub feedback: Feedback,
    /// The rated response's text; `None` if the chat no longer holds it.
    pub response_text: Option<String>,
}

fn feedback(row: &response_feedback::Model) -> Result<Feedback, DbErr> {
    let details = row
        .details
        .clone()
        .map(serde_json::from_value)
        .transpose()
        .map_err(|cause| unconvertible("stored feedback details", "Details", cause))?;
    Ok(Feedback {
        rating: Rating::parse(&row.rating)?,
        details,
        saved_at: row.saved_at,
        revision: u64::try_from(row.revision)
            .ok()
            .filter(|revision| *revision > 0)
            .ok_or_else(|| error("corrupt feedback revision"))?,
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

fn response_key(scope: &Scope, response: &ResponseId) -> (String, String, String, String, String) {
    let (practice, physician, workspace, chat) = keys(scope);
    (
        practice,
        physician,
        workspace,
        chat,
        response.as_str().to_owned(),
    )
}

impl Store {
    /// This physician's feedback on one chat's responses, by response.
    pub async fn chat_feedback(
        &self,
        scope: &Scope,
    ) -> Result<BTreeMap<ResponseId, Feedback>, DbErr> {
        use response_feedback::Column as C;
        scope.validate()?;
        response_feedback::Entity::find()
            .filter(C::Practice.eq(scope.practice.as_str()))
            .filter(C::Physician.eq(scope.physician.as_str()))
            .filter(C::Workspace.eq(scope.workspace.as_str()))
            .filter(C::Chat.eq(scope.chat.as_str()))
            .all(&self.db)
            .await?
            .iter()
            .map(|row| Ok((ResponseId::from(row.response.clone()), feedback(row)?)))
            .collect()
    }

    /// Saves the physician's feedback on the response named `response` in
    /// their chat: the rating and the details, replacing both, so a save
    /// without details clears them. Each save increments the revision.
    /// Answers [`FeedbackSaved::NotAResponse`] unless `response` names one of
    /// the chat's published responses. The auth owner's locks for `session_hash`
    /// are held in the same transaction.
    pub async fn save_feedback(
        &self,
        scope: &Scope,
        response: &ResponseId,
        rating: Rating,
        details: Option<&Details>,
        session_hash: &[u8],
        authority: &mut dyn Authority,
    ) -> Result<FeedbackSaved, DbErr> {
        use response_feedback::Column as C;
        scope.validate()?;
        if session_hash.len() != 32 {
            return Err(error("invalid session reference"));
        }
        let stored = details
            .map(serde_json::to_value)
            .transpose()
            .map_err(|cause| unconvertible("Details", "stored feedback details", cause))?;
        let tx = self.db.begin().await?;
        if authority
            .lock_publication(&tx, scope, Some(session_hash))
            .await
            .is_err()
        {
            tx.rollback().await?;
            return Ok(FeedbackSaved::Refused);
        }
        // A published response and its id never change, so
        // this unlocked read cannot see the answer withdrawn before the commit.
        let Some(row) = conversation::Entity::find_by_id(keys(scope))
            .one(&tx)
            .await?
        else {
            return Ok(FeedbackSaved::NotAResponse);
        };
        let conversation = Self::conversation_control(&tx, scope, row).await?;
        if conversation.response(response).is_none() {
            return Ok(FeedbackSaved::NotAResponse);
        }
        let now = now(&tx).await?;
        response_feedback::Entity::insert(response_feedback::ActiveModel {
            practice: Set(scope.practice.clone()),
            physician: Set(scope.physician.clone()),
            workspace: Set(scope.workspace.clone()),
            chat: Set(scope.chat.clone()),
            response: Set(response.as_str().to_owned()),
            rating: Set(rating.column().into()),
            details: Set(stored),
            saved_at: Set(now),
            revision: Set(1),
        })
        .on_conflict(
            // The row lock taken by the conflicting update orders concurrent
            // saves, so each one reads the revision the previous one left.
            OnConflict::columns([
                C::Practice,
                C::Physician,
                C::Workspace,
                C::Chat,
                C::Response,
            ])
            .update_columns([C::Rating, C::Details, C::SavedAt])
            .value(
                C::Revision,
                Expr::col((response_feedback::Entity, C::Revision)).add(1),
            )
            .to_owned(),
        )
        .exec_without_returning(&tx)
        .await?;
        let row = response_feedback::Entity::find_by_id(response_key(scope, response))
            .one(&tx)
            .await?
            .ok_or_else(|| error("missing feedback"))?;
        let saved = feedback(&row)?;
        tx.commit().await?;
        Ok(FeedbackSaved::Saved(saved))
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
            .order_by_asc(C::Response)
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
            let response_id = ResponseId::from(row.response.clone());
            let response = chat
                .as_ref()
                .and_then(|(_, conversation)| conversation.as_ref())
                .and_then(|c| c.response(&response_id));
            let place = response.as_ref().map_or(usize::MAX, |r| r.place);
            let response_text = response.map(|r| r.text.to_owned());
            evaluated.push((
                place,
                Evaluated {
                    scope,
                    response: response_id,
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

    /// Standalone physician reply draft feedback for evaluation by Coda staff
    /// (`K1.3`): `"Tell Coda what to change"` instructions (archived and
    /// current), direct physician edits saved on reply drafts, and copied
    /// replies compared against Coda's proposals.
    pub async fn reply_draft_feedback_for_evaluation(
        &self,
    ) -> Result<Vec<EvaluatedReplyFeedback>, DbErr> {
        let mut out = Vec::new();

        // 1. Archived "Tell Coda what to change" instructions.
        {
            use reply_revision::Column as R;
            let rows = reply_revision::Entity::find()
                .order_by_asc(R::Practice)
                .order_by_asc(R::Physician)
                .order_by_asc(R::Workspace)
                .order_by_asc(R::Reply)
                .order_by_asc(R::Ordinal)
                .all(&self.db)
                .await?;
            for row in rows {
                let practice = row.practice.clone();
                let physician = row.physician.clone();
                let workspace = row.workspace.clone();
                let reply = row.reply.clone();
                let earlier = control::earlier(row, Vec::new())?;
                out.push(EvaluatedReplyFeedback {
                    practice,
                    physician,
                    workspace,
                    reply,
                    detail: ReplyFeedbackDetail::TellCodaWhatToChange {
                        revision_key: earlier.accepted.input.key,
                        instruction: earlier.accepted.input.instruction,
                        base_text: earlier.base_text,
                        current_text: None,
                    },
                });
            }
        }

        // 2. Current reply drafts: active/latest revision instructions and
        // physician-saved text edits.
        {
            use reply_draft::Column as D;
            let rows = reply_draft::Entity::find()
                .order_by_asc(D::Practice)
                .order_by_asc(D::Physician)
                .order_by_asc(D::Workspace)
                .order_by_asc(D::Reply)
                .all(&self.db)
                .await?;
            for row in rows {
                let practice = row.practice.clone();
                let physician = row.physician.clone();
                let workspace = row.workspace.clone();
                let reply = row.reply.clone();
                let draft_revision = row.revision;
                let draft = control::draft(row, Vec::new())?;
                if let Some(accepted) = draft.accepted {
                    out.push(EvaluatedReplyFeedback {
                        practice: practice.clone(),
                        physician: physician.clone(),
                        workspace: workspace.clone(),
                        reply: reply.clone(),
                        detail: ReplyFeedbackDetail::TellCodaWhatToChange {
                            revision_key: accepted.input.key,
                            instruction: accepted.input.instruction,
                            base_text: draft.base_text,
                            current_text: Some(draft.text.clone()),
                        },
                    });
                }
                if draft_revision > 1 || draft.restore_text.is_some() {
                    out.push(EvaluatedReplyFeedback {
                        practice,
                        physician,
                        workspace,
                        reply,
                        detail: ReplyFeedbackDetail::PhysicianDraftEdit {
                            draft_revision,
                            edited_text: draft.text,
                            restore_text: draft.restore_text,
                        },
                    });
                }
            }
        }

        // 3. Copied replies paired with Coda's proposal text.
        {
            use reply_tables::reply_copy::{self, Column as C};
            use reply_tables::reply_proposal;
            let copies = reply_copy::Entity::find()
                .order_by_asc(C::Practice)
                .order_by_asc(C::Physician)
                .order_by_asc(C::Workspace)
                .order_by_asc(C::ConversationId)
                .order_by_asc(C::CopiedAt)
                .order_by_asc(C::CopyId)
                .all(&self.db)
                .await?;
            if !copies.is_empty() {
                let proposals = reply_proposal::Entity::find()
                    .order_by_asc(reply_proposal::Column::Version)
                    .all(&self.db)
                    .await?;
                let mut proposal_by_version: BTreeMap<(String, String, i64), String> =
                    BTreeMap::new();
                let mut latest_proposal_for_reply: BTreeMap<(String, String, i64), (u64, String)> =
                    BTreeMap::new();
                for proposal in proposals {
                    let version_u64 = u64::try_from(proposal.version)
                        .map_err(conversion_error("reply proposal version", "u64"))?;
                    latest_proposal_for_reply.insert(
                        (
                            proposal.practice.clone(),
                            proposal.conversation_id.clone(),
                            proposal.reply_id,
                        ),
                        (version_u64, proposal.text.clone()),
                    );
                    proposal_by_version.insert(
                        (
                            proposal.practice,
                            proposal.conversation_id,
                            proposal.version,
                        ),
                        proposal.text,
                    );
                }
                for copy in copies {
                    let source: CopiedReplySource = serde_json::from_value(copy.copied_source)
                        .map_err(conversion_error("reply copy source", "CopiedReplySource"))?;
                    let (proposal_version, proposal_text) = match source {
                        CopiedReplySource::CodaProposal { version, .. } => {
                            let text = i64::try_from(version).ok().and_then(|v| {
                                proposal_by_version
                                    .get(&(copy.practice.clone(), copy.conversation_id.clone(), v))
                                    .cloned()
                            });
                            (Some(version), text)
                        }
                        CopiedReplySource::PhysicianDraft { .. } => latest_proposal_for_reply
                            .get(&(
                                copy.practice.clone(),
                                copy.conversation_id.clone(),
                                copy.reply_id,
                            ))
                            .map(|(v, text)| (Some(*v), Some(text.clone())))
                            .unwrap_or((None, None)),
                    };
                    let edited_from_proposal = proposal_text
                        .as_deref()
                        .is_some_and(|proposal| proposal != copy.copied_text);
                    let reply_id = u64::try_from(copy.reply_id)
                        .map_err(conversion_error("reply copy reply_id", "u64"))?;
                    out.push(EvaluatedReplyFeedback {
                        practice: copy.practice,
                        physician: copy.physician,
                        workspace: copy.workspace,
                        reply: copy.conversation_id,
                        detail: ReplyFeedbackDetail::CopiedReply {
                            copy_id: copy.copy_id,
                            reply_id,
                            proposal_version,
                            proposal_text,
                            copied_text: copy.copied_text,
                            edited_from_proposal,
                            copied_at: copy.copied_at,
                        },
                    });
                }
            }
        }

        Ok(out)
    }
}

/// Specific physician reply feedback payload exported for evaluation (`K1.3`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "reply_feedback_kind", rename_all = "snake_case")]
pub enum ReplyFeedbackDetail {
    /// A physician's `"Tell Coda what to change"` revision request.
    TellCodaWhatToChange {
        /// The revision command key.
        revision_key: String,
        /// The physician's instruction to Coda.
        instruction: String,
        /// The draft text before the revision ran.
        base_text: String,
        /// Current text of the draft, when exported from the live draft row.
        current_text: Option<String>,
    },
    /// A physician's direct text edit saved on a reply draft.
    PhysicianDraftEdit {
        /// Saved revision counter on the draft.
        draft_revision: i64,
        /// Physician's edited text.
        edited_text: String,
        /// Prior text captured before a revision, if any.
        restore_text: Option<String>,
    },
    /// A physician's copied reply text compared against Coda's proposal.
    CopiedReply {
        /// Durable copy identifier.
        copy_id: String,
        /// Reply identity copied.
        reply_id: u64,
        /// Proposal version the copied text started from, if any.
        proposal_version: Option<u64>,
        /// Coda's proposal text for that version, when retained.
        proposal_text: Option<String>,
        /// Exact text the physician copied.
        copied_text: String,
        /// Whether the copied text differs from Coda's proposal.
        edited_from_proposal: bool,
        /// Database-clock Unix second when copied.
        copied_at: i64,
    },
}

/// One reply feedback row returned by [`Store::reply_draft_feedback_for_evaluation`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct EvaluatedReplyFeedback {
    /// Practice identifier.
    pub practice: String,
    /// Physician identifier.
    pub physician: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Reply or conversation identifier.
    pub reply: String,
    /// Specific reply feedback detail.
    #[serde(flatten)]
    pub detail: ReplyFeedbackDetail,
}
