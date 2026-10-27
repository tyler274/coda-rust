# Data promises

Promises about records that the graph's states can't hold: which update, which
sources, which patient, which words. The graph ([physician.mmd](physician.mmd))
holds the states around each one; the record itself is the Rust owner's, and
that owner is where each must be proved. None is proved yet.

## A concern always comes with its explanation

**Promise.** Coda raises or revises a concern only in work that writes you an
update explaining it, and opening the work that raised the concern opens that
update; a quiet finish may only close concerns (decisions.md, "A concern always
comes with its explanation").

**Graph.** "Concern" and each work's two finishes.

**Rust owner.** None in this repository writes concerns; the physician app only
reads them (`TodayRows`, crates/physician-app/src/lib.rs).

**Status.** Unproven; no owner yet.

## A reply revision reads what arrived before it finishes

**Promise.** A reply revision takes in what arrives while it works before it
finishes, so nothing it publishes rests on an older reading (Dev 2026-10-26).

**Graph.** "reply revision: news while working".

**Rust owner.** `DurableChat::accept_revision` (crates/physician-sample/src/durable.rs)
and the `read_patient_sources` tool in physician-worker
(crates/physician-sample/src/live.rs). Coverage records that they don't keep it
yet: acceptance pins the sources, and the revision reads only those.

**Status.** Unproven; the owner doesn't meet it yet.

## A message's work belongs to its patient

**Promise.** Work relies on a patient message only while it is linked to that
patient. When a correction takes effect, whatever relied on the message is
withheld until the patient review decides it from the patient's own record (N2,
PM:640-711).

**Graph.** "Patient message: match" and "What relied on a moved message".

**Rust owner.** None yet. Coverage marks the move and mark-not-a-patient controls
missing and the withholding partial.

**Status.** Unproven; no owner yet.

## A reply goes out in the words you approved

**Promise.** Coda sends exactly the text in your reply box, on the conversation
linked to this patient, shown as sent by Coda for you; a reply the channel can't
send keeps your text (decisions.md, communication standard; PM:160).

**Graph.** "Patient message: reply".

**Rust owner.** None yet. Coverage marks Send to the patient missing; the app has
Copy reply (`ReplyReviewCard`, crates/physician-app/src/lib.rs).

**Status.** Unproven; no owner yet.
