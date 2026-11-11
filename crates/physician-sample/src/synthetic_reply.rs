//! An explicitly enabled synthetic reply source for authenticated Chords,
//! so the first reply draft and its revision can be
//! exercised without seeding the database directly.
//!
//! `PHYSICIAN_SYNTHETIC_REPLY=<patient workspace id>` enables it for that
//! patient; it is off by default. It only supplies one fixed-ready reply, with
//! a stable server-owned identifier and non-PHI wording, in the synthetic
//! practice. The draft opens at revision 1 while the auth owner's session, account and
//! panel locks are held in the same transaction. Opening again keeps the
//! saved draft as it is.
//!
//! This is not inbound-message ingestion, clinical reply generation, or
//! patient delivery: nothing here reads a patient message, writes a chart, or
//! sends anything. Closing, sending and other lifecycle actions remain
//! unsupported for this reply.
use crate::auth::{AuthContext, STAGING_PRACTICE_ID};
use crate::live::Live;
use serde_json::{Value, json};

/// The switch: the patient workspace the reply belongs to. Blank or unset
/// leaves the source off.
pub(crate) const SWITCH: &str = "PHYSICIAN_SYNTHETIC_REPLY";

/// Stable, server-owned reply work identity.
pub(crate) const REPLY_ID: &str = "synthetic-reply-medication-question";

/// Non-PHI wording of the first draft.
pub(crate) const FIRST_DRAFT: &str = "Thank you for your message. Please keep taking your medicines as prescribed, and we will go over any questions at your next appointment.";

const HEADLINE: &str = "Synthetic message: question about medicines";
const CREATED_AT: &str = "2026-08-27T12:05:00Z";

/// The patient `value` names.
pub(crate) fn patient(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
}

/// Whether this signed-in physician and workspace get the synthetic reply.
fn applies(ctx: &AuthContext, patient: Option<&str>, workspace: &str) -> bool {
    ctx.identity.practice_id == STAGING_PRACTICE_ID && patient == Some(workspace)
}

fn scope(ctx: &AuthContext, workspace: &str) -> local_product::runtime::Scope {
    local_product::runtime::Scope {
        practice: ctx.identity.practice_id.clone(),
        physician: ctx.identity.physician_id.clone(),
        workspace: workspace.into(),
        chat: REPLY_ID.into(),
    }
}

fn wire(
    workspace: &str,
    display_name: &str,
    text: &str,
    revision: u64,
    run: Option<physician_wire::ReplyRevisionRun>,
) -> Value {
    json!({
        "reply_work_id": REPLY_ID,
        "patient_workspace_id": workspace,
        "patient_display_name": display_name,
        "channel": "spruce",
        "headline": HEADLINE,
        "draft_text": text,
        "urgency": "routine",
        "created_at": CREATED_AT,
        "state": "ready",
        "draft_revision": revision,
        "revision_run": run,
    })
}

/// The synthetic reply for this physician and workspace, opened if needed,
/// with its authoritative saved text, exact revision and latest run.
/// `Ok(None)` when the source is off, does not apply, or authority refused.
pub(crate) async fn project(
    live: &Live,
    ctx: Option<&AuthContext>,
    workspace: &str,
    display_name: &str,
) -> Result<Option<Value>, ()> {
    let patient = live.synthetic_reply();
    let Some(ctx) = ctx.filter(|ctx| applies(ctx, patient, workspace)) else {
        return Ok(None);
    };
    let durable = &live.chat;
    let scope = scope(ctx, workspace);
    let draft = match durable.store.draft(&scope).await.map_err(|_| ())? {
        Some(draft) => Some(draft),
        None => {
            let hash = physician_auth::PostgresAuthStore::session_reference(&ctx.token);
            let mut authority = crate::durable::RuntimeAuthority;
            durable
                .store
                .open_draft_authorized(&scope, FIRST_DRAFT, &hash, &mut authority)
                .await
                .map_err(|_| ())?
        }
    };
    Ok(draft.map(|draft| {
        let run = crate::reply::revision_run(&draft, REPLY_ID);
        wire(workspace, display_name, &draft.text, draft.revision, run)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_switch_names_the_patient() {
        assert_eq!(patient(Some(" pw-1 ")).as_deref(), Some("pw-1"));
        for off in [None, Some(""), Some("  ")] {
            assert_eq!(patient(off), None, "{off:?}");
        }
    }

    #[test]
    fn the_first_draft_is_fixed_non_empty_wording() {
        assert!(!FIRST_DRAFT.trim().is_empty());
        assert!(!REPLY_ID.trim().is_empty());
        let reply = wire("pw-1", "Synthetic patient", FIRST_DRAFT, 1, None);
        let parsed: physician_wire::WorkspaceReply = serde_json::from_value(reply).unwrap();
        assert_eq!(parsed.draft_revision, Some(1));
        assert_eq!(parsed.state, "ready");
        assert!(parsed.revision_run.is_none());
    }
}
