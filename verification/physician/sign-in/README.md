# Physician sign-in

How you, a physician, sign in to Chords, stay signed in across the browser's tabs and sign out. [physician.md](../../../intent/physician.md#a1-sign-in-and-access) links here.

## Words

- **Signed in:** the browser stores a session Coda still accepts, one that hasn't expired or been signed out, while your access stands. Otherwise you're signed out.
- **A sign-out:** you sign out, or another physician's sign-in link signs out the physician signed in.
- **Caught up:** a tab has heard every sign-in and sign-out from the other tabs, and waits on no answer from Coda.
- **A sign-in page:** Enter your work email, Check your email, This link no longer works, No access to this practice, or Coda couldn't reach sign-in.
- **Acting:** opening Coda, a patient link or a sign-in link, asking for a link, using a different email, signing out, or working in your workspace.

## Journey

Signing in:

- You sign in only through Chords, with a link Coda emails you. There's no password and no code screen.
- Accounts are invite-only: Coda sets up each practice and invites its first physician, and any member can then invite colleagues.
- You enter your work email and always see "Check your email".
- Coda never reveals whether an account exists, and an address with no account gets no email.
- The email never carries a patient.
- Opening the link signs you in and opens Today.
- A used or expired link says "This link no longer works", with a way to ask for a new one or use a different email.
- A link that no longer grants access says "No access to this practice".
- If Coda can't send or confirm the link, it says so and offers to try again.

One session across tabs:

- The session belongs to the browser: a new tab, or Coda reopened after closing it, opens your workspace while you're signed in.
- When you sign in from the link's tab, every other tab waiting on sign-in goes on to where it was.
- A tab catches up a moment after another tab signs in or out, or after Coda answers it.

When your session expires:

- Your session ends after 12 hours without a request from you, and always 7 days after you sign in.
- A sign-in prompt replaces the workspace and can email you a link. The patient page, drafts, the open reader and the conversation are cleared before the prompt is shown.
- After you sign back in, you start at Today. Coda's running work on the server continues.
- Once the session has ended, reopening Coda asks you to sign in.

A patient link while signed out:

- The tab holds the link while you sign in, then lands on that patient.
- A signed-out tab can't tell who signs in, so it opens the link for whoever does, and the practice's access rules decide what they see.

Signing out:

- Signing out signs out every tab and forgets where you were, including any patient link a tab held.

Another physician:

- Opening another physician's sign-in link while you're signed in signs you out first, then signs them in and opens Today.
- If Coda can't say whether you're still signed in, nobody is signed out: another physician's link says Coda couldn't confirm sign-in and offers to try again.
- Opening your own link while signed in keeps your session, and the link's tab opens Today.
- If another physician signs in in another tab, a tab that was yours becomes theirs and opens Today, never your patient.
- Drafts and seen state stay with each physician.

Acting in a tab:

- Coda acts only for the physician a tab shows, and only while that physician is signed in to the browser.
- A request from a tab whose physician isn't the one signed in is refused, and the tab catches up.

Losing access:

- If the practice removes your access, you're signed out and see "No access to this practice".

## Rules

1. After a sign-out, a caught-up tab never shows the workspace from before it, or any workspace on the session signed out.
2. A caught-up tab never shows your workspace on an expired session.
3. Your workspace, or the prompt that replaced it, leaves for a sign-in page only when you sign out in that tab, after a sign-out the tab missed, or when your access is removed.
4. While you're signed in, a caught-up tab never shows a sign-in page or the sign-in prompt, except on a page another physician's sign-in link opened.
5. A workspace opened from a sign-in link's page shows Today.
6. A patient link opened while you're signed out lands on that patient once you sign in, unless before then comes a sign-out, or that tab loses access, is taken over by another physician, or loads another page.
7. Coda acts only for the physician a tab shows, and only while that physician is signed in to the browser, whether or not the tab has caught up.
8. The sign-in prompt holds no patient page, no patient link and no patient owed after sign-in.
9. A sign-out ends that session at Coda, whether you sign out or another physician's link signs you out, so Coda refuses it afterwards.
10. Once you stop acting, every open tab catches up.

## Outside this spec

What a physician may see and do inside the workspace, how members invite colleagues, and where drafts and seen state live follow [physician.md](../../../intent/physician.md#a1-sign-in-and-access).
