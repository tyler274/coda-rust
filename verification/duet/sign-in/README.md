# Duet sign-in

This folder is the spec for how a person signs in to Duet on iPhone and Android, stays signed in and signs out of a phone. The model is the agreed behavior, TLC checks that it keeps its rules, and the code follows it. [duet.md](../../../intent/duet.md#d1-sign-up-talk-to-duet-keep-everything-across-a-restart) owns the intent this follows.

The rest of first run, newer Terms, AI permission, notifications, invitations, signing out everywhere and deletion follow duet.md and connected care, outside this spec. Where they meet sign-in, this spec takes them as one event: another phone signing out everywhere ends this phone's session like a revoke.

## How sign-in works

- **Signing up or in.** The person signs up on their own, with no invitation, by email on iPhone and Android. Duet emails a link that opens the app, with a short code as the fallback for typing on another device, and always says "Check your email". Duet sends the same email whether or not an account exists. A used or expired link says "This link no longer works", and if Duet can't be reached the screen says so. What a link does when opened on a phone already signed in isn't decided; the model takes it as opening Duet as it was. There's no guest, anonymous or local-only mode. Duet is for adults 18 and over in the U.S. Physicians never sign in to Duet.
- **The Terms.** A new account, or one that hasn't accepted the Terms, sees them right after signing in, with one explicit "I agree and continue" and no prechecked box. Accepting them also confirms the person is 18 or over and in the U.S. Duet records the acceptance, so a restart or another sign-in never asks again. A quiet Sign out sits under the Terms for a wrong account or declined Terms.
- **Staying signed in.** The session belongs to the phone and the account. After a restart the person is still signed in, and Duet reopens where they were, with any draft and unsent entries they left. Duet checks the session each time it opens, and the phone acts on what Duet last answered.
- **Duet unreachable.** The cached conversation and drafts stay usable, and the phone asks again once it can.
- **A session ended from elsewhere.** When Duet revokes the session, or the person signs out everywhere from another phone, Duet refuses the session from then on, including after a restart. The phone finds out when Duet refuses it: when Duet opens, or with the phone's next request. Duet also sends the open app a notice, which may never arrive. The phone then leaves the account. If it held an unsent draft or entries, the sign-in screen says what on this phone couldn't be saved.
- **Signing out of this phone.** With unsent notes or medications, Duet tries to send them first. If anything would still be lost, counting an unsent message draft, Duet says what and offers Stay signed in or Sign out anyway. Signing out takes effect at once, even offline, and clears the draft; the phone tells Duet now or as soon as it can reach it.
- **One phone, another person.** Once one person signs out, another can sign in on the same phone. The phone only ever shows the account signed in on it.

## Rules

The model checks these in every state it reaches.

- A phone shows only the account signed in on it: never another account's screens, draft or entries.
- Once Duet refuses the session, the phone shows none of that account's content.
- While the phone holds a live session, Duet never asks the person to sign in, including after a restart.
- Duet shows the account only after its Terms are accepted, and the Terms only until then.
- Unsent entries and a draft are never dropped without the person being warned first or told after.

These rest on stored records or the device, which the model can't hold. Each needs a test that reads the records back or runs on the installed app; none is named yet:

- One account per verified email.
- A sign-in link or code signs in at most once, and expires.
- Duet sends the same sign-in email whether or not an account exists.
- A message accepted before sign-out still arrives and gets a reply.
- The phone keeps its session, where it was, the draft and unsent entries across a restart.

## Files

| File | Holds |
| --- | --- |
| [DuetSignIn.tla](DuetSignIn.tla), [DuetSignIn.cfg](DuetSignIn.cfg) | The design as a TLA+ model: one phone and two accounts, the screens, the session the phone keeps, what Duet records, and the rules, one `INVARIANT` each. |

Check it from the repository root:

```bash
.agents/skills/statechart/scripts/statechart verification/duet/sign-in/DuetSignIn.tla
```

Duet owns the sessions and whether each account has accepted the Terms. The phone keeps its session, where it was, its draft and unsent entries, and acts on what Duet last answered. The model lets the person, Duet's answers, requests that don't get through, the app closing, Duet ending a session and its notice arriving or never arriving happen in any order. Duet acting on a request and its answer reaching the phone is one step; a request that doesn't get through, or that an app closed before the answer, is one Duet never acted on. Signing out is one step for the phone: it forgets the session and owes Duet the sign-out. Duet starts up to three sessions in a run, and the phone keeps one draft and one batch of entries at a time. TLC checks the rules in every state the model reaches; it doesn't reach the apps. The model is written from this spec, not from code; when the code is held to it, each action's comment gains the code that carries it out, and each bug path is replayed against the app.
