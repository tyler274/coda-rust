//! Duet's model input for one claimed conversation attempt, read under the same
//! fence as every execution write. It projects only Duet-owned rows: the exact
//! settled transcript through the current message, its timezone and the
//! assignment. Owners that are not ported yet (private understanding, Today
//! observations, conversation continuation, saved actions) are reported as
//! absent rather than fabricated. Selection is a transport bound, not a
//! judgment of what matters: older messages remain stored and readable.
use super::{execution::Claim, *};
use serde_json::{Value, json};

/// Most recent settled messages supplied verbatim.
pub const RECENT_MESSAGES: i64 = 50;
/// Encoded byte budget for the verbatim window, excluding the current message.
pub const CONVERSATION_BYTES: usize = 65_536;
/// Exact note the product gives a retried attempt.
pub const EARLIER_ATTEMPT_NOTE: &str = "An earlier attempt at this same request stopped on a service fault before it finished. Its reply was discarded. Actions it confirmed are done and kept; do not repeat them.";

/// Assembled input. `payload` is the exact JSON the model receives.
#[derive(Clone, Debug, PartialEq)]
pub struct ConversationContext {
    /// The person's current message.
    pub current_message_id: Uuid,
    /// One-based attempt ordinal for this accepted run.
    pub attempt: i32,
    /// Last transcript sequence visible to this attempt.
    pub through_sequence: i64,
    /// Exact model-facing document.
    pub payload: Value,
}

impl PostgresFrontDoorStore {
    /// Assemble context for a live claim. Refuses (never degrades) when the
    /// claim, consent, session or operator switch no longer holds.
    pub async fn conversation_context(
        &self,
        claim: &Claim,
    ) -> Result<ConversationContext, AuthorityError> {
        let tx = self.db.begin().await?;
        let run = execution::fence(&tx, claim).await?;
        let current: Uuid = run.try_get("", "message_id")?;
        let timezone: String = run.try_get("", "timezone")?;
        let attempt: i32 = run.try_get("", "attempt_count")?;
        let head = tx.query_one_raw(sql("SELECT m.sequence, to_char(clock_timestamp() AT TIME ZONE $3,'YYYY-MM-DD\"T\"HH24:MI:SS') AS local_now, (SELECT min(sequence) FROM duet_conversation_messages WHERE account_id=$1) AS first_sequence, (SELECT id FROM duet_conversation_messages WHERE account_id=$1 ORDER BY sequence LIMIT 1) AS first_id FROM duet_conversation_messages m WHERE m.account_id=$1 AND m.id=$2",vec![claim.account.into(),current.into(),timezone.clone().into()])).await?.ok_or(AuthorityError::Refused)?;
        let through: i64 = head.try_get("", "sequence")?;
        // Settled messages only: unfinished or interrupted empty replies are not
        // things Duet said. Newest first, then trimmed to the byte budget.
        let rows = tx.query_all_raw(sql("SELECT id,sequence,role,text,state FROM duet_conversation_messages WHERE account_id=$1 AND sequence<$2 AND (state='complete' OR (role='assistant' AND state='interrupted' AND text<>'')) ORDER BY sequence DESC LIMIT $3",vec![claim.account.into(),through.into(),RECENT_MESSAGES.into()])).await?;
        let current_row = tx.query_one_raw(sql("SELECT id,sequence,role,text,state FROM duet_conversation_messages WHERE account_id=$1 AND id=$2",vec![claim.account.into(),current.into()])).await?.ok_or(AuthorityError::Refused)?;
        tx.commit().await?;
        let message = |r: &sea_orm::QueryResult| -> Result<Value, sea_orm::DbErr> {
            Ok(json!({
                "id": r.try_get::<Uuid>("", "id")?.to_string(),
                "role": r.try_get::<String>("", "role")?,
                "text": r.try_get::<String>("", "text")?,
                "state": r.try_get::<String>("", "state")?,
                "sequence": r.try_get::<i64>("", "sequence")?,
            }))
        };
        let mut recent = Vec::new();
        let mut used = 0usize;
        for r in &rows {
            let m = message(r)?;
            let size = serde_json::to_vec(&m)
                .map_err(|_| AuthorityError::Refused)?
                .len();
            if used + size > CONVERSATION_BYTES {
                break;
            }
            used += size;
            recent.push(m);
        }
        recent.reverse();
        let verbatim_from = recent
            .first()
            .and_then(|m| m["sequence"].as_i64())
            .unwrap_or(through);
        recent.push(message(&current_row)?);
        let first_id: Option<Uuid> = head.try_get("", "first_id")?;
        let mut payload = json!({
            "conversation_continuation": null,
            "conversation_history_boundary": {
                "first_message_id": first_id.map(|id| id.to_string()),
                "first_sequence": head.try_get::<Option<i64>>("", "first_sequence")?,
                "through_sequence": through,
                "verbatim_from_sequence": verbatim_from,
                "continuation_through_sequence": null,
            },
            "local_timezone": timezone,
            "current_time": head.try_get::<String>("", "local_now")?,
            "working_understanding": null,
            "published_today_observations": [],
            "understanding_updated_at": null,
            "changed_since_understanding_was_reviewed": null,
            "conversation": recent,
            "conversation_through_user_sequence": through,
            "conversation_through_sequence": through,
            "current_message_reply_target": null,
            "assignment": {"kind": "conversation", "selected_follow_up": null},
        });
        if attempt > 1 {
            payload["earlier_attempt"] = json!({
                "attempt": attempt,
                "note": EARLIER_ATTEMPT_NOTE,
                "confirmed_actions": [],
            });
        }
        Ok(ConversationContext {
            current_message_id: current,
            attempt,
            through_sequence: through,
            payload,
        })
    }
}
