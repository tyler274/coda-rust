//! Reserved integration boundary: intentionally no create/import HTTP endpoint.
//! The follow-up producer is a conversation tool, never client supplied accepted
//! instruction authority. Direct patient controls belong in the authenticated
//! router only AFTER privacy Restriction variants and outbox replay are wired.
//!
//! Integration checklist (coordinator-owned files):
//! - lib.rs: `pub mod follow_up;` (this file need not be declared).
//! - database.rs: register 0011_follow_up_producer AFTER 0010, and grant runtime
//!   SELECT,INSERT,UPDATE on follow_ups,duet_follow_up_occurrences,
//!   duet_follow_up_settings; SELECT,INSERT only on duet_follow_up_origins;
//!   SELECT on duet_due_follow_up_accounts. No DELETE/TRUNCATE/schema privileges.
//! - ConversationOwner::effect, after its existing fence/dedup check: construct
//!   AgentAuthority::from_owner(self.context.clone(),self.id,self.lease,self.claim).
//!   Dispatch read_follow_up -> read; create_follow_up -> create; edit_follow_up
//!   -> edit; pause/resume/cancel_follow_up -> agent_control. Keep the returned
//!   result and mutation inside the existing duet_tool_effects/event transaction.
//!   Use caller-preserved UUID for Definition.id and expected revision for edits.
//!   The actor never comes from call.arguments. No second tool-effect ledger.
//! - Add corresponding tools to accepted Request.tools. read_follow_up takes id;
//!   create takes Definition; edit takes {revision,definition}; controls take
//!   {id,revision}. Unknown fields should be rejected by the tool argument DTOs.
//! - Privacy Restriction adds FollowUpPause/FollowUpCancel/ProactiveWithdraw.
//!   Append the existing content-free outbox event in the SAME control transaction
//!   (resource UUID, account incarnation, revision, original event ID/time only).
//!   Confirm independent journal bytes AFTER commit; uncertain retries reuse ID.
//!   Replay calls replay_restriction/replay_settings_withdrawal after verification.
//!   Never publish a successful restriction receipt based only on this module.
//! - Person endpoints authenticate AccountContext; accept ONLY revision/event ID,
//!   call person_control in the privacy transaction; they cannot author definitions.
//! - Exact source read tool is existing read_message with command_id. It delivers
//!   the full completed command pair. Current user message ID is current command;
//!   assistant IDs use the existing high-bit transform. Prior IDs must have an
//!   exact delivered read receipt; a foreign UUID/snippet cannot grant authority.
//! - Scheduled admission must use reserve_due + insert assistant-only command in
//!   one transaction (deferred FK enforces this). Build Request from frozen returned
//!   accepted_instruction/operation/revision and current permitted context. Do NOT
//!   call ordinary accept with fabricated user input. Add explicit scheduled lane
//!   and assignment projection before enabling sweeps; existing interactive-only
//!   phone projection currently assumes every command has a patient message.
//! - claim/fence/complete call mark_running/fence_scheduled/complete_scheduled in
//!   their existing transactions. Completion records the existing assistant ID.
//!   Fixed reminders use frozen accepted_instruction before provider construction,
//!   but still pass all permission, lease, scheduled, reply and occurrence fences.
//! - Temporary newer-user/settings holds call hold_scheduled and suspend/release
//!   the SAME command (no terminal cancel), then existing recovery resumes it after
//!   the quiet window. Current duet_commands state/index needs explicit held work
//!   support before this can be enabled without blocking new interactive commands.
//! - Actual definition edits/pause/cancel permanently interrupt old assignments;
//!   restore never reopens terminal assignments. Cancellation must retain original
//!   command's transcript/effect receipts. Events notify action receipt changes;
//!   phone action projection still needs to join current definition by result.id.
//! - No fabricated source cards. Follow-ups are accepted work, not health evidence.
//! - No routine linking, push delivery, generic HTTP export/pagination, hosted job,
//!   or live scheduling runner is claimed by this isolated module.
