# Infrastructure gaps

The graph ([duet.mmd](duet.mmd)) describes the ideal product for every person
and leaves runtime mechanics out (decisions.md, "Ideal state, not
infrastructure"). This list is context for whoever builds the runtime, not a
list of questions. Each entry says what the graph assumes away and what the real
system must still guarantee, so that the product people use behaves as the graph
says. How to meet each guarantee is the runtime's call.

Failures caused by the outside world are not on this list. They are in the
graph: a reply that can't finish, a message Duet refuses, Duet's own work that
can't finish, a deletion still finishing.

## One reply at a time, steered

**Assumed away.** How a message that arrives while Duet writes joins the reply.
The graph has one Send and no Stop; a new message steers the reply in progress,
which answers both (Dev 2026-10-25).

**Must guarantee.** A message sent while a reply runs is never refused or queued
behind it; it reaches that reply. A limit of one model run in flight per account
is compatible with one continuous conversation: the second message steers that
run instead of being turned away.

## A Check-in that keeps failing

**Assumed away.** How many failures in a row pause a Check-in. The graph keeps
only what you see: it couldn't finish, with Try again, and after failing
repeatedly it is paused, with Resume (Dev 2026-10-26).

**Must guarantee.** A Check-in that keeps failing stops on its own and says so
in the thread instead of failing silently at every scheduled time; three in a
row is the starting point Dev picked. Only you resume it.

## When Duet does its own work

**Assumed away.** When and how often Duet looks on its own. The graph has one
"Duet takes on its own work" under your consent, the AI switch, current Terms
and proactive work, and keeps only how it ends (Dev 2026-10-26).

**Must guarantee.** The routine looks Dev kept from coda-app still happen: a
daily look once your local day completes, a weekly recap once a Saturday-to-
Friday week with something new completes, and a look when portal or Health app
clinical records arrive (Dev 2026-10-25). Each day and week is decided once,
and a look still owed for an older one gives way to the newer one. Phone
readings wait for the next look rather than starting one (Dev 2026-10-06).

## For my doctor stays live

**Assumed away.** How the workspace syncs across your devices and Duet's saves.
The graph has one saved version and says only that it is live (Dev 2026-10-26).

**Must guarantee.** Every open editor receives newer saves as they land and keeps
the person's unsaved words; a save made from an older version never drops what
was saved since, and only an edit to the same words in two places asks the
person to pick. Records in the workspace resolve to their current version when
shown and when sent.

## Sending messages

**Assumed away.** How an unconfirmed send is checked. The graph keeps Sending,
Not confirmed yet, Not sent with Try again and Edit, and Sent.

**Must guarantee.** A sent message is never lost and never arrives twice: the
same message id is resent, never a copy (D1). Only a definite refusal shows Not
sent; a message on its way when you sign out still arrives and is answered
(Dev 2026-10-25).

## Retries and recovery

**Assumed away.** Retry budgets for replies, reviews and Check-ins (D3). Work
either finishes or shows that it couldn't, with Try again where the graph says.

**Must guarantee.** Recovery is silent. A retry never repeats an effect: no
second Reminder or Check-in delivery, no duplicate saved item, no second insight
for the same decision. Couldn't finish shows only once recovery has given up.

## Interruptions and restarts

**Assumed away.** A deploy, crash or restart while Duet writes, reviews or runs a
Check-in.

**Must guarantee.** A restart is invisible: the work resumes or reruns from a
fresh read, and a steer sent during it still reaches the reply. Pausing (AI off,
newer Terms, a new AI permission, proactive work paused) never uses work up.

## Health sources

**Assumed away.** Portal authorization, token custody, paging, retries, how
often sources refresh on their own, and phone uploads. The graph shows an import running and ending, and readings
arriving.

**Must guarantee.** A portal sign-in finishes only in the session and account
that started it, and work started under one grant never publishes under a later
one (B1). An interrupted import resumes without duplicates and never reports
itself complete when it isn't; one unreadable record fails alone, named (B2,
B5). Unsent phone readings survive app exit, relaunch and a locked phone, and
wait untouched while another account is signed in (A6). Continuous sensor data
never leaves the phone (A1). Each connected organization refreshes on its own
at most once a day (B7), and a refresh the person asks for never waits on it. Engineering sees when each connection last
succeeded and is alerted, naming the connection and the reason, when one is
overdue.

## Scheduling follow-through

**Assumed away.** Clocks and timers. The graph's "comes due" is an event; how it
fires is not.

**Must guarantee.** Reminders and Check-ins fire at their time in the timezone
the person agreed. A time that comes while AI work is paused passes undelivered
(Dev 2026-10-25). A Check-in may wait until the chat is quiet, but delivers
once (D2). The caps on follow-through (three per reply, twenty open) are bounds
the runtime enforces without changing what the person sees as saved.

## Command receipts

**Assumed away.** How a lost answer to a command is confirmed.

**Must guarantee.** A command changes its item only when accepted. A lost answer
shows as pending, is confirmed without the person's help where possible, and
never applies twice. A pending command ends at sign-out (physician decision 8,
applied to Duet).

## The live feed to the practice

**Assumed away.** Transport, ordering and catch-up between Duet and the
practice. The graph shows each change reaching the practice while connected
([connected-care](../connected-care/connection.mmd)).

**Must guarantee.** On joining, everything the patient already shares reaches
the practice; while connected, every change to it
reaches the practice exactly once, keeping its origin and date; a corrected
record arrives as a new version, and duplicates of what the practice already
holds arrive labelled. A For my doctor the patient sends arrives
once, as the version they sent. Leaving, discharge and account deletion stop new
data at once; the practice keeps what it already received. Duet and the
practice never show different connection status.
Coming back, the practice's documented care and its schedule reach Duet only
while connected, keeping source, author and date; a later change replaces what
Duet shows without losing its provenance.

## Accounts and sessions

**Assumed away.** How sessions on other devices are ended and how deletion
proceeds.

**Must guarantee.** Sign out everywhere says done only once every session has
ended. Deletion finishes even if interrupted, and nothing old work or
a restored backup does brings the account back (F2).

## Operations and release

**Assumed away.** The operator's AI switch, the daily message allowance and the
app-update gate (G1, G2).

**Must guarantee.** When an operator control or update gate stops AI work, the
person sees AI as unavailable, with their drafts and saved work intact, never a
silent failure or a lost message.
