//! Hosted chat drafts: one physician's unsent composer text for one chat.
//!
//! One typed row per practice, physician, workspace and chat. A save names
//! the revision it replaces and holds the auth owner's locks for the
//! physician's session in the same transaction, so a revoked session or a
//! patient taken off the panel writes nothing. Drafts are clinical working
//! text and are never deleted automatically.
use sea_orm::{
    ColumnTrait, DbErr, EntityTrait, QueryFilter, Set, TransactionTrait,
    sea_query::{Expr, ExprTrait, OnConflict},
};

use super::entities::chat_draft;
use super::{Authority, Scope, Store, error, keys, now};

/// A saved chat draft. Revision 0 with empty text means none was saved.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChatDraft {
    /// Saved revision; every save increments it.
    pub revision: u64,
    /// The saved composer text.
    pub text: String,
}

/// Outcome of a chat-draft save.
#[derive(Debug, PartialEq, Eq)]
pub enum ChatDraftSaved {
    /// Saved at the returned revision.
    Saved(ChatDraft),
    /// The draft moved past the expected revision; nothing was written.
    Conflict(ChatDraft),
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

fn draft(row: Option<chat_draft::Model>) -> Result<ChatDraft, DbErr> {
    Ok(match row {
        Some(row) => ChatDraft {
            revision: u64::try_from(row.revision).map_err(|_| error("corrupt chat draft"))?,
            text: row.text,
        },
        None => ChatDraft::default(),
    })
}

impl Store {
    /// This physician's saved draft for one chat; `{0, ""}` when none exists.
    pub async fn chat_draft(&self, scope: &Scope) -> Result<ChatDraft, DbErr> {
        nonempty(scope)?;
        draft(
            chat_draft::Entity::find_by_id(keys(scope))
                .one(&self.db)
                .await?,
        )
    }

    /// Saves composer text under an exact revision fence. `expected` 0 opens
    /// the draft; any other value must equal the saved revision. The auth
    /// owner's locks for `session_hash` are held in the same transaction.
    pub async fn save_chat_draft(
        &self,
        scope: &Scope,
        expected: u64,
        text: &str,
        session_hash: &[u8],
        authority: &mut dyn Authority,
    ) -> Result<ChatDraftSaved, DbErr> {
        use chat_draft::Column as C;
        nonempty(scope)?;
        if session_hash.len() != 32 {
            return Err(error("invalid session reference"));
        }
        let expected_i64 = i64::try_from(expected).map_err(|_| error("invalid draft revision"))?;
        let tx = self.db.begin().await?;
        authority
            .lock_publication(&tx, scope, Some(session_hash))
            .await
            .map_err(|e| error(&e))?;
        let now = now(&tx).await?;
        let written = if expected == 0 {
            chat_draft::Entity::insert(chat_draft::ActiveModel {
                practice: Set(scope.practice.clone()),
                physician: Set(scope.physician.clone()),
                workspace: Set(scope.workspace.clone()),
                chat: Set(scope.chat.clone()),
                text: Set(text.into()),
                revision: Set(1),
                created_at: Set(now),
                updated_at: Set(now),
            })
            .on_conflict(OnConflict::new().do_nothing().to_owned())
            .exec_without_returning(&tx)
            .await?
        } else {
            // Postgres rechecks `revision` after taking the row lock, so of
            // two saves naming the same revision exactly one updates.
            chat_draft::Entity::update_many()
                .col_expr(C::Text, Expr::val(text))
                .col_expr(C::Revision, Expr::col(C::Revision).add(1))
                .col_expr(C::UpdatedAt, Expr::val(now))
                .filter(C::Practice.eq(scope.practice.as_str()))
                .filter(C::Physician.eq(scope.physician.as_str()))
                .filter(C::Workspace.eq(scope.workspace.as_str()))
                .filter(C::Chat.eq(scope.chat.as_str()))
                .filter(C::Revision.eq(expected_i64))
                .exec(&tx)
                .await?
                .rows_affected
        };
        let current = draft(chat_draft::Entity::find_by_id(keys(scope)).one(&tx).await?)?;
        tx.commit().await?;
        Ok(if written == 1 {
            ChatDraftSaved::Saved(current)
        } else {
            ChatDraftSaved::Conflict(current)
        })
    }
}
