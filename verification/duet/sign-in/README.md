# Duet sign-in

How a person signs in to Duet on iPhone and Android, stays signed in and signs out of a phone. [duet.md](../../../intent/duet.md#d1-sign-up-talk-to-duet-keep-everything-across-a-restart) owns the intent this follows.

The rest of first run, newer Terms, AI permission, notifications, invitations, signing out everywhere, changing an email, getting back in without access to one's email, deletion, and how messages and entries reach Duet follow duet.md and connected care, outside this spec. Where they meet sign-in, this spec takes them as one event: another phone signing out everywhere ends this phone's session like a revoke.

## How sign-in works

- **Signing up or in.** The person signs up on their own, with no invitation, by email on iPhone and Android. Signing up and signing in are one flow: the person enters their email, and Duet emails a link that opens the app, with a short code as the fallback. The link signs in to that email's account, or creates it, so the screen says "Check your email" either way, and each verified email has one account. A link or code signs in once and expires; a used or expired one says "This link no longer works", and if Duet can't be reached the screen says so. A link opened while the phone is already signed in opens Duet as it was. There's no guest, anonymous or local-only mode. Duet is for adults 18 and over in the U.S. Physicians never sign in to Duet.
- **The Terms.** A new account, or one that hasn't accepted the Terms, sees them right after signing in, with one explicit "I agree and continue" and no prechecked box. Accepting them also confirms the person is 18 or over and in the U.S. Duet records the acceptance, so signing in again never asks for it, and once the phone has heard it's recorded, a restart doesn't either. Until the phone hears, it stays on the Terms and checks again; agreeing again changes nothing Duet holds. A quiet Sign out sits under the Terms for a wrong account or declined Terms.
- **Staying signed in.** The session belongs to the phone and the account. After a restart the person is still signed in, and Duet reopens where they were, with any draft they left. Duet checks the session each time it opens. Sessions don't expire with time: a session ends only when the person signs out of that phone, signs out everywhere, or Duet revokes it.
- **Duet unreachable.** The cached conversation and drafts stay usable, and the phone asks again once it can.
- **A session ended from elsewhere.** When Duet revokes the session, or the person signs out everywhere from another phone, Duet refuses the session from then on, including after a restart. The phone finds out when Duet next refuses it: when Duet opens, or with the phone's next request. Duet also tells the open app, but nothing depends on that notice arriving. The phone then leaves the account, and if it held a draft or entries Duet hadn't confirmed, the sign-in screen says what it removed. Until Duet refuses it, a phone that can't reach Duet keeps showing the account, behind the app's screen cover and the phone's own passcode.
- **Signing out of this phone.** With notes or medications Duet hasn't confirmed, Duet tries to send them first. If it can't confirm them, or there's an unsent message draft, Duet says what signing out removes from this phone and offers Stay signed in or Sign out anyway. If Duet confirms the entries while it says so, signing out finishes, or with a draft left, Duet says only that the draft goes. Signing out takes effect at once, even offline, and clears the draft; Duet hears of it as soon as the phone can reach it.
- **One phone, another person.** Once one person signs out, another can sign in on the same phone. The phone only ever shows the account signed in on it.

## Rules

- A phone shows only the account signed in on it: never another account's screens, draft or entries.
- Once Duet refuses the session, the phone shows none of that account's content.
- While the phone holds a live session, Duet never asks the person to sign in, including after a restart.
- Duet shows the account only after its Terms are accepted, and the Terms only until the phone hears they're accepted.
- A draft, or entries Duet hasn't confirmed, are never removed without the person being warned first or told after.
- Duet warns that signing out removes something only while the phone holds a draft or entries Duet hasn't confirmed.
