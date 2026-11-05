---
name: coda-verify-messaging
description: "Check a change to patient messages against its TLA+ model: TLC on the README's agreed rules, then the charts read in the physician's words. Use when a change touches the patient-messaging spec or model, or the code that reads the practice's channel, links messages, reviews them or shows the reply."
---

# Verify patient messages

The spec is [verification/physician/messaging/README.md](../../../verification/physician/messaging/README.md)
and the model is [Messaging.tla](../../../verification/physician/messaging/Messaging.tla). Each `INVARIANT`
or `PROPERTY` in [Messaging.cfg](../../../verification/physician/messaging/Messaging.cfg) is one of the
README's rules, quoted in its comment. The harm when one breaks: a physician sees a patient they may
not access, a patient's words reach another patient, a patient's message goes unseen, a reply closes
as answered when it wasn't, a physician's words are lost or changed, or the history misstates what
the practice sent.

## When a change needs this

Run it when a change touches the messaging README, `Messaging.tla`, `Messaging.cfg`, `Charts.tla` or
a chart. No code carries the design yet; as the channel reader, linking, review and reply code land,
each action's comment in `Messaging.tla` names the code it models, and that code goes in this list.

## Run

Check the model, and read every result, with the pstack `/statechart` skill on
`verification/physician/messaging/Messaging.tla`. It passes when the report lists every rule in
`Messaging.cfg`, and NoDeadlock, under the rules that hold.

Then draw each chart with its `.cfg` and `.mjs` on `Charts.tla`, render it as the skill says, and
read it. Each chart's `.mjs` says what it draws and what it leaves out. Every rule must hold in each.

## Bounds and assumptions

`Messaging.cfg` sets the scope the rules are checked at, and the model's header says what it takes as
one step. Which patients each physician may access, and who sees a message before it's linked, are
inputs the config sets every way, since they're open in P2. TLC covers every state of that finite
model; it doesn't reach the code.

## Promises tests hold

These rest on stored records or the channel, which the model can't hold. No test holds them yet:

- Coda reads each message once, however often the channel delivers it, and saves it before moving on.
- Each practice's channel has one reader at a time.
- A review starts once for its event, across restarts and retries.
- The patient's words are never changed, only formatted, with unsafe content stripped.
- The card shows the messages and the reply from one version.
- Undo restores the reply exactly.
- Moved messages are marked entered in error and never deleted, and what Coda wrote from them can't surface.
- One unreadable message, or one patient whose linking fails, never holds up the others.

## A failure

- **TLC breaks a rule.** The report gives the shortest path. If every step is what the README says,
  the design breaks the rule: take it to Dev, and never weaken the rule.
- **The code differs from the model on a path.** If the code does what the README says and the model
  doesn't, the model is wrong: fix the action from the README's words. If the code differs from both,
  it's a bug: replay the path as a test, see it fail, then fix the code.
- **An intended change.** Change the README with Dev first, then the model, then the code.
