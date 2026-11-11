# Duet sign-in

How you sign in to Duet on iPhone and Android, stay signed in and sign out of a phone. [duet.md](../../../intent/duet.md#d1-sign-up-and-stay-signed-in) links here.

## Words

- **Signed in on a phone:** the phone holds a session for your account.
- **Live:** a session Duet hasn't ended. It ends only when you sign out of that phone, sign out everywhere, or Duet revokes it.
- **Duet refuses a session:** Duet answers the phone's check or request by refusing that session, because it has ended.
- **Your account's content:** the You and Shared tabs, and signing out's screens over it.
- **Unconfirmed:** a draft you haven't sent, or notes or medications Duet hasn't confirmed.
- **A sign-in screen:** Enter your email, Check your email, Signing you in, This link no longer works, Duet couldn't be reached, or the screen saying what was removed.

## Journey

Signing up or in:

- You sign up on your own, with no invitation, by email on iPhone or Android.
- Signing up and signing in are one flow: you enter your email, and Duet emails a link that opens the app, with a short code as the fallback.
- The link signs in to that email's account, or creates it, so the screen says "Check your email" either way.
- Each verified email has one account.
- A link or code signs in once and expires; a used or expired one says "This link no longer works".
- If Duet can't be reached, the screen says so.
- A link opened while the phone is signed in opens Duet as it was.
- There's no guest, anonymous or local-only mode.
- Duet is for adults 18 and over in the U.S.
- Physicians never sign in to Duet.

The Terms:

- A new account, or one that hasn't accepted the Terms, sees them right after signing in, with one explicit "I agree and continue" and no prechecked box.
- Accepting them also confirms you're 18 or over and in the U.S.
- Duet records the acceptance, so signing in again never asks for it, and once the phone has heard it's recorded, a restart doesn't either.
- Until the phone hears, it stays on the Terms and checks again; agreeing again changes nothing Duet holds.
- A quiet Sign out sits under the Terms, for a wrong account or declined Terms.

Staying signed in:

- The session belongs to the phone and the account.
- After a restart you're still signed in, and Duet reopens where you were, with any draft you left.
- Duet checks the session each time it opens.
- Sessions don't expire with time.

When Duet can't be reached:

- Drafts stay usable, and the phone asks again once it can.

A session ended from elsewhere:

- When Duet revokes the session, or you sign out everywhere from another phone, Duet refuses the session from then on, including after a restart.
- The phone finds out when Duet next refuses it: when Duet opens, or with the phone's next request.
- Duet also tells the open app, but nothing depends on that notice arriving.
- The phone then leaves the account, and if it held anything unconfirmed, the sign-in screen says what it removed.
- Until Duet refuses it, a phone that can't reach Duet keeps showing the account, behind the app's screen cover and the phone's own passcode.

Signing out of this phone:

- With unconfirmed notes or medications, Duet tries to send them first.
- If it can't confirm them, or there's an unsent draft, Duet says what signing out removes from this phone and offers Stay signed in or Sign out anyway.
- If Duet confirms the entries while it says so, signing out finishes, or with a draft left, Duet says only that the draft goes.
- If Duet closes before signing out finishes, the sign-out is abandoned and Duet reopens where you were.
- Signing out takes effect at once, even offline, and clears the draft; Duet hears of it as soon as the phone can reach it.

One phone, another person:

- Once you sign out, another person can sign in on the same phone.
- The phone only ever shows the account signed in on it.

## Rules

1. A phone shows and keeps only the account signed in on it: never another account's screens, draft or entries.
2. Once Duet refuses a session, the phone never shows your account's content while it holds that session.
3. While the phone holds a live session, it never shows a sign-in screen.
4. Duet shows your account's content only after its Terms are accepted, and the Terms only until the phone hears they're accepted.
5. Nothing unconfirmed leaves the phone unless Duet confirms it, you were warned first, or you're told after.
6. Duet warns that signing out removes something only while the phone holds something unconfirmed.

## Outside this spec

The rest of first run, newer Terms, AI permission, notifications, invitations, signing out everywhere, changing an email, getting back in without access to one's email, deletion, and how messages and entries reach Duet follow [duet.md](../../../intent/duet.md) and connected care; how the app shows newer Terms is open question Q7 in [OPEN-QUESTIONS.md](../../../intent/OPEN-QUESTIONS.md). Where they meet sign-in, they act as one event: another phone signing out everywhere ends this phone's session the way a revoke does.
