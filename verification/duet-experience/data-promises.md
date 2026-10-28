# Data promises

Promises about records that the graph's states can't hold: which words, which
version, which saved content. The graph ([duet.mmd](duet.mmd)) holds the states
around each one; the record itself is the Rust owner's, and that owner is where
each must be proved. None is proved yet.

## Your message keeps its words

**Promise.** The words you send are the words your account holds: Send moves
them into the outgoing message, an unconfirmed send keeps them, only a definite
refusal returns them, Edit gives them back, and signing out doesn't drop them
(decisions.md, D1; Dev 2026-10-25).

**Graph.** "Your latest message" and "Your draft".

**Rust owner.** `ConversationTab` (crates/duet-app/src/lib.rs) for the composer;
`ConversationStore::accept_submission` (crates/duet-api/src/conversation.rs) for
what the account accepts.

**Status.** Unproven.

## Supporting information shows the cited version

**Promise.** Opening a citation shows the exact version the reply cited, never a
newer one in its place; a correction adds a version and never rewrites one
(decisions.md, C6, E5).

**Graph.** "Open on Duet" and "The cited version".

**Rust owner.** `EvidenceStore::exact` and `exact_in_transaction`
(crates/duet-api/src/evidence.rs), which read the addressed revision.

**Status.** Unproven.

## For my doctor changes only as you accepted it

**Promise.** Nothing Duet proposes saves by itself. Your own words save word for
word; an update saves exactly as you reviewed it, onto the version you reviewed
it against; a proposal overtaken by a newer save waits for review again
(duet.mmd, For my doctor; Dev 2026-10-26).

**Graph.** "For my doctor" and "Duet's proposed update for your doctor".

**Rust owner.** `WorkspaceStore::save`, `stage_proposal`, `reject_proposal` and
`approved_read` (crates/duet-api/src/workspace.rs); the `check` route
(crates/duet-api/src/doctor_routes.rs) refuses a page or evidence that changed.

**Status.** Unproven.
