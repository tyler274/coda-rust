//! Command receipts: the idempotency record of every accepted physician
//! command, kept in `physician_runtime_commands`.
//!
//! A receipt holds the full command for [`RECEIPT_FULL_SECONDS`]. The same
//! input already lives in the conversation, or in the reply draft and its
//! archived revisions, so after that the receipt is only a duplicate copy:
//! [`Store::compact_receipts`] replaces it with `{"salt", "sha256"}`. The salt
//! is random per row and stored beside the digest, so a retried command still
//! compares equal and a different command under the same key still collides.
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set,
    TransactionTrait, Unchanged,
    sea_query::{Expr, ExprTrait, LockBehavior, LockType, OnConflict, extension::postgres::PgExpr},
};
use serde_json::Value;
use uuid::Uuid;

use super::entities::command;
use super::{Store, error, now};

/// Full receipts are kept this long (90 days), then reduced to a digest.
pub const RECEIPT_FULL_SECONDS: i64 = 90 * 24 * 60 * 60;

/// Whether a receipt insert found an earlier command under the same key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Receipt {
    /// Newly recorded in this transaction.
    New,
    /// The same command was already accepted.
    Same,
    /// A different command already holds this key.
    Different,
}

/// Whether a command key has already been recorded prior to executing side effects.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ExistingReceipt {
    /// Key has not been used yet for `(practice, physician)`.
    Absent,
    /// Key already holds this exact command.
    Same,
    /// Key already holds a different command.
    Different,
}

/// Object keys sorted at every level, so the digest never depends on the
/// order JSONB or a `serde_json` feature happens to keep them in.
fn canonical(value: &Value, out: &mut String) {
    match value {
        Value::Object(map) => {
            let mut entries: Vec<_> = map.iter().collect();
            entries.sort_by(|a, b| a.0.cmp(b.0));
            out.push('{');
            for (index, (key, value)) in entries.into_iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                out.push_str(&Value::String(key.clone()).to_string());
                out.push(':');
                canonical(value, out);
            }
            out.push('}');
        }
        Value::Array(items) => {
            out.push('[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                canonical(item, out);
            }
            out.push(']');
        }
        scalar => out.push_str(&scalar.to_string()),
    }
}

fn digest(salt: &str, command: &Value) -> String {
    let mut text = String::from(salt);
    canonical(command, &mut text);
    crate::sha256_hex(text.as_bytes())
}

/// A compacted receipt is exactly `{"salt": string, "sha256": string}`.
/// Accepted commands are objects keyed by `scope` and their kind, never this.
fn compacted(stored: &Value) -> Option<(&str, &str)> {
    let map = stored.as_object().filter(|map| map.len() == 2)?;
    Some((map.get("salt")?.as_str()?, map.get("sha256")?.as_str()?))
}

/// Whether `command` is the one a stored receipt recorded, full or compacted.
pub(super) fn matches(stored: &Value, command: &Value) -> bool {
    match compacted(stored) {
        Some((salt, sha256)) => digest(salt, command) == sha256,
        None => stored == command,
    }
}

/// Records `command` under its key in `tx`, or compares it with the receipt
/// already there.
pub(super) async fn record(
    tx: &sea_orm::DatabaseTransaction,
    practice: &str,
    physician: &str,
    key: &str,
    command: &Value,
) -> Result<Receipt, DbErr> {
    let accepted_at = now(tx).await?;
    let inserted = command::Entity::insert(command::ActiveModel {
        practice: Set(practice.to_owned()),
        physician: Set(physician.to_owned()),
        key: Set(key.to_owned()),
        command: Set(command.clone()),
        accepted_at: Set(accepted_at),
    })
    .on_conflict(OnConflict::new().do_nothing().to_owned())
    .exec_without_returning(tx)
    .await?;
    if inserted == 1 {
        return Ok(Receipt::New);
    }
    let previous =
        command::Entity::find_by_id((practice.to_owned(), physician.to_owned(), key.to_owned()))
            .one(tx)
            .await?
            .ok_or_else(|| error("missing command receipt"))?
            .command;
    Ok(if matches(&previous, command) {
        Receipt::Same
    } else {
        Receipt::Different
    })
}

impl Store {
    /// Checks whether `(practice, physician, key)` already holds a command receipt.
    pub(crate) async fn inspect_command_receipt(
        &self,
        practice: &str,
        physician: &str,
        key: &str,
        command: &Value,
    ) -> Result<ExistingReceipt, DbErr> {
        let existing = command::Entity::find_by_id((
            practice.to_owned(),
            physician.to_owned(),
            key.to_owned(),
        ))
        .one(&self.db)
        .await?;
        let Some(row) = existing else {
            return Ok(ExistingReceipt::Absent);
        };
        Ok(if matches(&row.command, command) {
            ExistingReceipt::Same
        } else {
            ExistingReceipt::Different
        })
    }

    /// Durably records `command` under `(practice, physician, key)` in `physician_runtime_commands`.
    pub(crate) async fn record_command_receipt(
        &self,
        practice: &str,
        physician: &str,
        key: &str,
        command: &Value,
    ) -> Result<Receipt, DbErr> {
        let tx = self.db.begin().await?;
        let outcome = record(&tx, practice, physician, key, command).await?;
        tx.commit().await?;
        Ok(outcome)
    }

    /// Reduces up to `limit` full receipts accepted at least
    /// `max_age_seconds` ago to a salted digest, oldest first, and returns
    /// how many it reduced. Rows another sweep holds are skipped, not waited
    /// on. Nothing is deleted: the key and the digest keep idempotency.
    pub async fn compact_receipts(&self, max_age_seconds: i64, limit: u64) -> Result<u64, DbErr> {
        use command::Column as C;
        if max_age_seconds < 0 || !(1..=1000).contains(&limit) {
            return Err(error("invalid receipt compaction bounds"));
        }
        let tx = self.db.begin().await?;
        let cutoff = now(&tx).await? - max_age_seconds;
        let rows = command::Entity::find()
            .filter(C::AcceptedAt.lte(cutoff))
            .filter(Expr::col(C::Command).get_json_field("sha256").is_null())
            .order_by_asc(C::AcceptedAt)
            .limit(limit)
            .lock_with_behavior(LockType::Update, LockBehavior::SkipLocked)
            .all(&tx)
            .await?;
        let mut reduced = 0;
        for row in rows {
            if compacted(&row.command).is_some() {
                continue;
            }
            let salt = Uuid::new_v4().simple().to_string();
            let sha256 = digest(&salt, &row.command);
            command::ActiveModel {
                practice: Unchanged(row.practice),
                physician: Unchanged(row.physician),
                key: Unchanged(row.key),
                command: Set(serde_json::json!({"salt": salt, "sha256": sha256})),
                accepted_at: Unchanged(row.accepted_at),
            }
            .update(&tx)
            .await?;
            reduced += 1;
        }
        tx.commit().await?;
        Ok(reduced)
    }
}

#[cfg(test)]
mod unit {
    use super::*;
    use serde_json::json;

    #[test]
    fn digests_ignore_key_order_and_compare_only_the_same_command() {
        let command =
            json!({"scope": {"chat": "c", "practice": "p"}, "input": {"key": "k", "text": "t"}});
        let mut reordered = serde_json::Map::new();
        reordered.insert("input".into(), json!({"text": "t", "key": "k"}));
        reordered.insert("scope".into(), json!({"practice": "p", "chat": "c"}));
        let reordered = Value::Object(reordered);
        let stored = json!({"salt": "s", "sha256": digest("s", &command)});
        assert!(matches(&stored, &command));
        assert!(matches(&stored, &reordered));
        assert!(!matches(
            &stored,
            &json!({"scope": {"chat": "c", "practice": "p"}, "input": {"key": "k", "text": "u"}})
        ));
        assert_ne!(digest("s", &command), digest("t", &command));
        assert!(matches(&command, &reordered));
        assert!(!matches(&command, &stored));
    }
}
