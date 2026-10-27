//! An explicitly enabled synthetic reply source for the authenticated live
//! and hosted paths, so the first reply draft and its hosted revision can be
//! exercised without seeding the database directly.
//!
//! `PHYSICIAN_SYNTHETIC_REPLY=1` enables it; it is off by default. It only
//! supplies one fixed-ready reply, with a stable server-owned identifier and
//! non-PHI wording, to the synthetic practice's first recorded workspace. The
//! draft opens at revision 1 while the auth owner's session, account and
//! panel locks are held in the same transaction (hosted) or under the
//! publication fence (inline). Opening again keeps the saved draft as it is.
//!
//! This is not inbound-message ingestion, clinical reply generation, or
//! patient delivery: nothing here reads a patient message, writes a chart, or
//! sends anything. Closing, sending and other lifecycle actions remain
//! unsupported for this reply.
use crate::auth::{AuthContext, STAGING_PRACTICE_ID};
use crate::live::Live;
use local_product::execution::Status;
use serde_json::{Value, json};

/// The switch. Any other value, or none, leaves the source off.
pub(crate) const SWITCH: &str = "PHYSICIAN_SYNTHETIC_REPLY";

/// Stable, server-owned reply work identity.
pub(crate) const REPLY_ID: &str = "synthetic-reply-medication-question";

/// Non-PHI wording of the first draft.
pub(crate) const FIRST_DRAFT: &str = "Thank you for your message. Please keep taking your medicines as prescribed, and we will go over any questions at your next appointment.";

const HEADLINE: &str = "Synthetic message: question about medicines";
const CREATED_AT: &str = "2026-08-27T12:05:00Z";

/// Whether `value` turns the source on.
pub(crate) fn enabled(value: Option<&str>) -> bool {
    value.map(str::trim) == Some("1")
}

/// The one workspace the fixed scenario is associated with.
pub(crate) fn workspace() -> &'static str {
    crate::WORKSPACE_IDS[0]
}

/// Whether this signed-in physician and workspace get the synthetic reply.
fn applies(ctx: &AuthContext, workspace: &str) -> bool {
    ctx.identity.practice_id == STAGING_PRACTICE_ID && workspace == self::workspace()
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
    run: Option<Value>,
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

/// The inline owner's run status in wire words.
fn inline_status(status: &Status) -> &'static str {
    match status {
        Status::Active => "active",
        Status::Completed => "completed",
        // Interrupted and cancelled inline work never publishes.
        Status::Interrupted | Status::Cancelled | Status::Failed => "failed",
    }
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
    let Some(ctx) = ctx.filter(|ctx| live.synthetic_reply() && applies(ctx, workspace)) else {
        return Ok(None);
    };
    if let Some(durable) = &live.durable {
        let scope = scope(ctx, workspace);
        let draft = match durable.store.draft(&scope).await.map_err(|_| ())? {
            Some(draft) => Some(draft),
            None => {
                let hash = physician_auth::PostgresAuthStore::session_reference(&ctx.token);
                let mut authority = crate::durable::RuntimeAuthority {
                    store: durable.store.clone(),
                };
                durable
                    .store
                    .open_draft_authorized(&scope, FIRST_DRAFT, &hash, &mut authority)
                    .await
                    .map_err(|_| ())?
            }
        };
        return Ok(draft.map(|draft| {
            let run = draft
                .input
                .as_ref()
                .map(|input| json!({ "run_id": input.key, "status": draft.status }));
            wire(workspace, display_name, &draft.text, draft.revision, run)
        }));
    }
    let target = crate::reply::target(ctx, REPLY_ID);
    let existing = live
        .session
        .lock()
        .expect("physician session")
        .draft(workspace, &target)
        .is_some();
    if !existing {
        let mut open = || {
            live.session
                .lock()
                .expect("physician session")
                .open_reply_draft(workspace, &target, FIRST_DRAFT)
                .map(|_| ())
        };
        if crate::publication::publish(ctx, workspace, &mut open)
            .await
            .is_err()
        {
            return Ok(None);
        }
    }
    let session = live.session.lock().expect("physician session");
    let Some((revision, text, _)) = session.draft(workspace, &target) else {
        return Ok(None);
    };
    let run = session
        .reply_run(workspace, &target)
        .map(|(id, status)| json!({ "run_id": id, "status": inline_status(&status) }));
    Ok(Some(wire(workspace, display_name, &text, revision, run)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_exact_switch_value_enables_the_source() {
        assert!(enabled(Some("1")));
        assert!(enabled(Some(" 1 ")));
        for off in [None, Some(""), Some("0"), Some("true"), Some("yes")] {
            assert!(!enabled(off), "{off:?}");
        }
    }

    #[test]
    fn the_first_draft_is_fixed_non_empty_wording() {
        assert!(!FIRST_DRAFT.trim().is_empty());
        assert!(!REPLY_ID.trim().is_empty());
        let reply = wire(workspace(), "Synthetic patient", FIRST_DRAFT, 1, None);
        let parsed: physician_wire::WorkspaceReply = serde_json::from_value(reply).unwrap();
        assert_eq!(parsed.draft_revision, Some(1));
        assert_eq!(parsed.state, "ready");
        assert!(parsed.revision_run.is_none());
    }
}
