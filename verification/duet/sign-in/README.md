# Duet sign-in

This folder is the spec for how a person signs in to Duet on iPhone and Android, stays signed in and signs out of a phone. The model is the agreed behavior, TLC checks that it keeps its rules, and the code follows it. [duet.md](../../../intent/duet.md#d1-sign-up-talk-to-duet-keep-everything-across-a-restart) owns the intent this follows.

The rest of first run, newer Terms, AI permission, notifications, invitations, signing out everywhere, changing an email, getting back in without access to one's email, and deletion follow duet.md and connected care, outside this spec. Where they meet sign-in, this spec takes them as one event: another phone signing out everywhere ends this phone's session like a revoke.

The person signs up on their own, with no invitation, by email on iPhone and Android. There's no guest, anonymous or local-only mode. Duet is for adults 18 and over in the U.S. Physicians never sign in to Duet. Sessions don't expire with time: a session ends only when the person signs out of that phone, signs out everywhere, or Duet revokes it.

## Flows

Every step of the model names the flow it carries out, and [Flows.tla](Flows.tla) gives each flow a scene: the shortest path on which it happens.

**Signing up or in**

- **F1.** The person enters their email, and Duet emails a link that opens the app, with a short code as the fallback. The screen says "Check your email" either way.
- **F2.** The link or code signs in to that email's account, or creates it.
- **F3.** A used or expired link or code says "This link no longer works".
- **F4.** If Duet can't be reached, the screen says so. Duet may have signed the person in without its answer reaching the phone; the person tries again.
- **F5.** A link opened while the phone is already signed in opens Duet as it was.

**The Terms**

- **F6.** A new account, or one that hasn't accepted the Terms, sees them right after signing in, with one explicit "I agree and continue" and no prechecked box. Accepting them also confirms the person is 18 or over and in the U.S.
- **F7.** Duet records the acceptance, so signing in again never asks for it, and once the phone has heard it's recorded, a restart doesn't either.
- **F8.** Until the phone hears, it stays on the Terms and checks again; agreeing again changes nothing Duet holds.
- **F9.** A quiet Sign out sits under the Terms for a wrong account or declined Terms.

**Staying signed in**

- **F10.** After a restart the person is still signed in, and Duet reopens where they were, with any draft and unconfirmed entries they left. Duet checks the session each time it opens, and the phone acts on what Duet last answered.
- **F11.** With no session, opening Duet asks for an email.
- **F12.** When Duet can't be reached, the cached conversation and drafts stay usable, and the phone asks again once it can.
- **F13.** The person writes a message draft or logs notes or medications; the phone sends the entries until Duet takes them.

**A session ended from elsewhere**

- **F14.** Duet ends a session when it revokes it, or when the person signs out everywhere from another phone. Duet refuses that session from then on, including after a restart.
- **F15.** Duet also tells the open app, which then checks, but nothing depends on that notice arriving.
- **F16.** The phone finds out when Duet refuses it: when Duet opens, or with the phone's next request. The phone then leaves the account. If it held a draft or entries Duet hadn't confirmed, the sign-in screen says what it removed.
- **F17.** Until Duet refuses it, a phone that can't reach Duet keeps showing the account, its draft and its entries. The app's screen cover and the phone's own passcode protect it meanwhile.

**Signing out of this phone**

- **F18.** With notes or medications Duet hasn't confirmed, the phone sends them first.
- **F19.** If any are still unconfirmed, or there's an unsent message draft, Duet says what signing out removes from this phone and offers Stay signed in or Sign out anyway.
- **F20.** If Duet confirms the entries while it says so, signing out finishes, or with a draft left, Duet says only that the draft goes.
- **F21.** Signing out takes effect at once, even offline, and clears the draft; the phone tells Duet now or as soon as it can reach it.

**One phone, another person**

- **F22.** Once one person signs out, another can sign in on the same phone. The phone only ever shows the account signed in on it.

## Rules

The model checks these in every state it reaches.

| Rule | In the model |
| --- | --- |
| A phone shows only the account signed in on it: never another account's screens, draft or entries. | `OnlyItsAccount` |
| Once Duet refuses the session, the phone shows none of that account's content. | `GoneOnceRefused` |
| While the phone holds a live session, Duet never asks the person to sign in, including after a restart. | `StaysSignedIn` |
| Duet shows the account only after its Terms are accepted, and the Terms only until the phone hears they're accepted. | `TermsFirst` |
| A draft, or entries Duet hasn't confirmed, are never removed without the person being warned first or told after. | `NothingRemovedSilently` |
| Duet warns that signing out removes something only while the phone holds a draft or entries Duet hasn't confirmed. | `WarnsOnlyWhenUnconfirmed` |

F17 is why the second rule starts at Duet's refusal: a phone that can't reach Duet can't learn its session ended.

These rest on stored records or the device, which the model can't hold, and no test holds them yet:

- One account per verified email.
- A sign-in link or code signs in at most once, and expires.
- Every email address gets a link, which signs in to its account or creates it.
- Duet keeps each entry once, however often the phone sends it.
- A message accepted before sign-out still arrives and gets a reply.
- The phone keeps its session, where it was, the draft and unconfirmed entries across a restart.

## What the model holds

Duet owns the sessions and whether each account has accepted the Terms. The phone keeps its session, where it was, its draft and unconfirmed entries, and acts on what Duet last answered. The model lets the person, Duet's answers, requests that don't get through, the app closing, Duet ending a session and its notice reaching the app or not happen in any order. A request can fail before Duet acts on it, or Duet can act and its answer never reach the phone. Signing out is one step for the phone: it forgets the session and owes Duet the sign-out. The phone keeps one draft and one batch of entries at a time.

[DuetSignIn.cfg](DuetSignIn.cfg) checks the rules with two accounts, so the phone can pass from one person to another, and three sessions, so the phone can sign in a third time while Duet hasn't heard of both earlier sign-outs. TLC reaches 141,385 states. Each rule fails on a copy of the model broken for it.

## Charts

Each chart is the same model run at a scope small enough to draw, in the screens' words. [Charts.tla](Charts.tla) gives the other charts their starting moment, signed in with a draft and unconfirmed entries, and the steps of their scene. The rules are checked in every chart's run too.

| Chart | Shows | Leaves out |
| --- | --- | --- |
| [chart.cfg](chart.cfg), [chart.mjs](chart.mjs) | One person's sign-in, with one account and one session: signing in, the Terms, staying signed in, a session ended elsewhere, and signing out | A second person on the phone and signing in again. The two sign-in screens that didn't finish are one state, and a session Duet ended elsewhere shows on the arrows of its refusal. It doesn't draw closing Duet from a sign-in screen or while the Terms are being accepted, or opening a link anywhere but where the person waits for it. |
| [chart-ended.cfg](chart-ended.cfg), [chart-ended.mjs](chart-ended.mjs) | Duet ends the session while the phone may not reach it: the conversation, draft and entries still shown until Duet refuses the session, then signed out and told what was removed | Writing, sending and signing out, and signing back in. |
| [chart-passed-on.cfg](chart-passed-on.cfg), [chart-passed-on.mjs](chart-passed-on.mjs) | You sign out with a draft and unconfirmed entries, and another person signs in on the same phone and accepts the Terms | A session ended elsewhere, the link failing, writing or logging anything new, and your own link. |

## Files

| File | Holds |
| --- | --- |
| [DuetSignIn.tla](DuetSignIn.tla), [DuetSignIn.cfg](DuetSignIn.cfg) | The design as a TLA+ model: one phone, the screens, the session the phone keeps, what Duet records, and the rules, one `INVARIANT` each. Each step names its flow. |
| [Flows.tla](Flows.tla), [flows.cfg](flows.cfg), [scenes.mjs](scenes.mjs) | Each flow, claimed never to happen. The statechart skill refutes every claim with the shortest path on which the flow happens, told step by step: that path is the flow's scene. A flow that no step carries out has no scene. |
| [Charts.tla](Charts.tla), `chart*.cfg`, `chart*.mjs` | The charts above. |

From the repository root, check the rules, tell the scenes, then draw a chart:

```bash
.agents/skills/statechart/scripts/statechart verification/duet/sign-in/DuetSignIn.tla
.agents/skills/statechart/scripts/statechart verification/duet/sign-in/Flows.tla --config verification/duet/sign-in/flows.cfg --view verification/duet/sign-in/scenes.mjs
.agents/skills/statechart/scripts/statechart verification/duet/sign-in/DuetSignIn.tla --config verification/duet/sign-in/chart.cfg --view verification/duet/sign-in/chart.mjs
.agents/skills/statechart/scripts/statechart verification/duet/sign-in/Charts.tla --config verification/duet/sign-in/chart-ended.cfg --view verification/duet/sign-in/chart-ended.mjs
```

The passed-on chart runs the same way with `chart-passed-on`. TLC checks the rules in every state the model reaches; it doesn't reach the apps, and no code follows the model yet.
