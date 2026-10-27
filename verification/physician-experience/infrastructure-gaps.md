# Infrastructure gaps

The graph ([physician.mmd](physician.mmd)) describes the ideal product for every
user and leaves runtime mechanics out (decisions.md, "Ideal state, not
infrastructure"). This list is for whoever builds the runtime. Each entry says
what the graph assumes away and what the real system must still guarantee, so
that the product people use behaves as the graph says. How to meet each
guarantee is the runtime's call.

Failures caused by the outside world are not on this list. They are in the
graph: a records source that is down, a channel that refuses, a message Coda
can't read, a page that fails to load.

## Work capacity

**Assumed away.** Work for one patient runs side by side. Only real dependencies
order it, such as later work building on the initial understanding.

**Must guarantee.** If the runtime queues or serializes work, the physician sees
the same outcomes as side by side. Unrelated work never waits behind other work
in a way that changes what shows on Today, Updates or the patient page.

## Retries and recovery

**Assumed away.** The graph has no retry budget. Work either finishes, waits on
missing information, or fails into Couldn't finish with Try again. Work Coda
can't restart goes to Today with Done only if something is left unhandled.

**Must guarantee.** Recovery is silent; machinery never alerts the physician
(CT:95). A retry never repeats an effect: no second send, and no duplicate
concern, follow-up or update. Couldn't finish is shown only once recovery has
given up.

## Interruptions and restarts

**Assumed away.** Work is never interrupted. A steer joins the running answer
at once.

**Must guarantee.** A process restart, deploy or crash mid-work is invisible: the
work resumes or reruns from a fresh read. A steer sent during a restart reaches
the answer, or the answer restarts with it. Typed input is never lost (N3).

## News while working

**Assumed away.** How often the runtime re-reads records while work runs.

**Must guarantee.** Initial understanding, patient review and visit preparation
reflect every record that arrived before they finish. Answers and asked replies
that finish on older records say so and offer Update (decision 7).

## Records currency

**Assumed away.** Records stay current while the source is up. The graph shows a
failed check and a source that is down, but no "overdue" state.

**Must guarantee.** Each source is checked at least daily, with no model calls,
and a review follows only when something is new (CT). A failed check retries
without waiting a day (N11). A scheduled pull never silently lapses while the
source is up; the physician has no status for that case.

## Matching patient messages

**Assumed away.** How long Coda tries to match a message.

**Must guarantee.** Matching always ends, either in a suggestion or in "unsure"
with a person deciding. It never hangs.

## Channel sync

**Assumed away.** Catching up and temporary sync failures on Gmail or Spruce.

**Must guarantee.** These are never shown. Only a revoked grant or an
administrator's block surfaces Reconnect (PM-1275). Reconnecting catches up
without duplicates, and accepted, sent, delivered and read stay distinct
(PM:161).

**Proved so far (Gmail push wake).** Evidence kind: a controlled test on the
real Pub/Sub emulator, driving the real `gmail::push` router and
`gmail::history` reader against the `FakeGmail` stand-in
([`crates/gmail/tests/push_wake.rs`](../../crates/gmail/tests/push_wake.rs)).
It shows that a push is a wake only: state changes only through
`history.list` from the stored cursor, and an unknown mailbox or malformed
payload is acked without a read. With a test-owned cursor and seen-message set,
duplicate and reordered wakes apply each message once and never move the cursor
back. A refused push, a crash before the cursor commits, and a nacked pull are
redelivered without repeating effects in that stand-in. This is not evidence of
production durable deduplication or atomic cursor/effect commits.
A poison wake dead-letters after five attempts while healthy wakes complete.
One ordering key keeps publish order, and an expired cursor surfaces instead of
reading past it. Still unproven: push OIDC (the emulator has no IAM), real
Gmail, the durable cursor and intake that own these effects, and hosted
Pub/Sub.

## Command receipts

**Assumed away.** How a lost receipt is confirmed.

**Must guarantee.** A command changes its item only when accepted. A lost
receipt shows as pending, is confirmed without the physician's help where
possible, and never sends or applies twice. A pending command survives session
expiry and ends at sign-out (decision 8).

## Sending replies

**Assumed away.** Milestone stopgaps such as a Copy-only reply card (px.md:153).

**Must guarantee.** Coda sends exactly the approved words, shown as sent by Coda
for the physician (PM:160). Whether the app offers Send yet is coverage.md's row,
not a guarantee here.

## Reads and loading

**Assumed away.** Caching, prefetching and how reads are scheduled. The graph
keeps reads that load, fail with Retry, and keep what already loaded.

**Must guarantee.** A failed read never blanks content that already loaded, and a
small failure never closes a whole surface (decisions 4 and 5).
