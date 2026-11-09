//! Send next: queued messages waiting behind a running answer.
//!
//! Coda keeps the queue in the conversation row, so it survives a closed tab
//! and shows on every device. Each queued message is named by the key of the
//! command that queued it. Edit and withdraw take effect once: their receipt
//! is saved with the change, so a retried command answers `Duplicate`. They
//! reach only a message still in the queue; once its turn has started, or
//! Stop returned it to the composer, the message is out of reach and the
//! command is refused without keeping its receipt.
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, QuerySelect, Set, TransactionTrait};

use super::entities::{chat_draft, conversation};
use super::{Conversation, QueuedMessage, Scope, Store, keys, now, receipt};

/// Outcome of a physician's edit or withdrawal of a queued message.
#[derive(Debug, PartialEq, Eq)]
pub enum QueueChanged {
    /// Saved with its receipt.
    Changed,
    /// The exact same command was already taken.
    Duplicate,
    /// The message is no longer queued (it started, was withdrawn, or Stop
    /// returned it), a different command holds the key, or a field is blank.
    Refused,
}

/// What a physician does to a queued message.
enum Change<'a> {
    Edit(&'a str),
    Withdraw,
}

impl Conversation {
    /// The queued message `id`, only while it has not started.
    fn queued_mut(&mut self, id: &str) -> Option<&mut QueuedMessage> {
        self.queued.iter_mut().find(|queued| queued.id == id)
    }
}

impl Store {
    /// Replaces the text of queued message `queued_id` before it starts.
    pub async fn edit_queued(
        &self,
        scope: &Scope,
        key: &str,
        queued_id: &str,
        text: &str,
    ) -> Result<QueueChanged, DbErr> {
        if text.trim().is_empty() {
            return Ok(QueueChanged::Refused);
        }
        self.change_queued(scope, key, queued_id, Change::Edit(text))
            .await
    }

    /// Removes queued message `queued_id` before it starts; it is never sent.
    pub async fn withdraw_queued(
        &self,
        scope: &Scope,
        key: &str,
        queued_id: &str,
    ) -> Result<QueueChanged, DbErr> {
        self.change_queued(scope, key, queued_id, Change::Withdraw)
            .await
    }

    async fn change_queued(
        &self,
        scope: &Scope,
        key: &str,
        queued_id: &str,
        change: Change<'_>,
    ) -> Result<QueueChanged, DbErr> {
        if [
            key,
            queued_id,
            &scope.practice,
            &scope.physician,
            &scope.workspace,
            &scope.chat,
        ]
        .iter()
        .any(|part| part.trim().is_empty())
        {
            return Ok(QueueChanged::Refused);
        }
        let command = match change {
            Change::Edit(text) => serde_json::json!({
                "scope": scope,
                "queue_edit": {"queued_id": queued_id, "text": text},
            }),
            Change::Withdraw => serde_json::json!({
                "scope": scope,
                "queue_withdraw": {"queued_id": queued_id},
            }),
        };
        let tx = self.db.begin().await?;
        match receipt::record(&tx, &scope.practice, &scope.physician, key, &command).await? {
            receipt::Receipt::New => {}
            receipt::Receipt::Same => {
                tx.commit().await?;
                return Ok(QueueChanged::Duplicate);
            }
            receipt::Receipt::Different => {
                tx.commit().await?;
                return Ok(QueueChanged::Refused);
            }
        }
        let Some(row) = conversation::Entity::find_by_id(keys(scope))
            .lock_exclusive()
            .one(&tx)
            .await?
        else {
            // Dropping the transaction rolls the receipt back.
            return Ok(QueueChanged::Refused);
        };
        let mut state: Conversation =
            serde_json::from_value(row.state).map_err(|cause| DbErr::TryIntoErr {
                from: "stored conversation",
                into: "Conversation",
                source: std::sync::Arc::new(cause),
            })?;
        match change {
            Change::Edit(text) => {
                let Some(queued) = state.queued_mut(queued_id) else {
                    return Ok(QueueChanged::Refused);
                };
                queued.text = text.to_owned();
            }
            Change::Withdraw => {
                let before = state.queued.len();
                state.queued.retain(|queued| queued.id != queued_id);
                if state.queued.len() == before {
                    return Ok(QueueChanged::Refused);
                }
            }
        }
        Self::save(tx, scope, &state).await?;
        Ok(QueueChanged::Changed)
    }
}

/// Stop never sends queued messages: their text goes back to the
/// physician's composer for this chat, after any text already there, in the
/// same transaction as the hold. The draft's revision moves, so an open page
/// that has the old text sees the returned text, or a conflict beside its
/// own unsaved typing, and nothing is lost.
pub(super) async fn return_to_draft(
    tx: &sea_orm::DatabaseTransaction,
    scope: &Scope,
    state: &mut Conversation,
) -> Result<(), DbErr> {
    let queued = std::mem::take(&mut state.queued);
    if queued.is_empty() {
        return Ok(());
    }
    let returned = queued
        .into_iter()
        .map(|queued| queued.text)
        .collect::<Vec<_>>()
        .join("\n\n");
    let now = now(tx).await?;
    let existing = chat_draft::Entity::find_by_id(keys(scope))
        .lock_exclusive()
        .one(tx)
        .await?;
    match existing {
        Some(row) => {
            // The draft can still hold a queued message whose Send clear has
            // not landed. Appending after it is deliberate: the sending tab
            // then reads a newer, different draft as a conflict and keeps
            // both. Collapsing the copy would hand that tab its own words,
            // and its pending clear would erase what Stop returned.
            let text = if row.text.trim().is_empty() {
                returned
            } else {
                format!("{}\n\n{returned}", row.text)
            };
            let revision = row.revision + 1;
            let mut row: chat_draft::ActiveModel = row.into();
            row.text = Set(text);
            row.revision = Set(revision);
            row.updated_at = Set(now);
            row.update(tx).await?;
        }
        None => {
            // A draft opened by another tab meanwhile fails the insert, and
            // with it the Stop, rather than dropping the returned text.
            chat_draft::Entity::insert(chat_draft::ActiveModel {
                practice: Set(scope.practice.clone()),
                physician: Set(scope.physician.clone()),
                workspace: Set(scope.workspace.clone()),
                chat: Set(scope.chat.clone()),
                text: Set(returned),
                revision: Set(1),
                created_at: Set(now),
                updated_at: Set(now),
            })
            .exec_without_returning(tx)
            .await?;
        }
    }
    Ok(())
}
