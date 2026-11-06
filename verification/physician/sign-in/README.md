# Physician sign-in

This folder is the spec for how a physician signs in to Coda Agents. The model is the agreed behavior, TLC checks that it keeps its rules, and the code follows it. [physician.md](../../../intent/physician.md#a1-sign-in-and-access) links here.

## How sign-in works

Physicians sign in only through Coda Agents, with a link Coda emails them. There's no password and no code screen. Accounts are invite-only: Coda sets up each practice and invites its first physician by email, and any member can then invite colleagues.

- **Asking for a link.** The physician enters their work email and always sees "Check your email". Coda never reveals whether an account exists, and an address with no account gets no email. The email never carries a patient.
- **Opening the link.** Clicking the button in the email signs them in and opens Today. A used or expired link says "This link no longer works", with a way to ask for a new one or use a different email. A link that no longer grants access says "No access to this practice". If Coda can't send or confirm the link, it says so and offers to try again.
- **One session across tabs.** The session belongs to the browser, so a new tab, or Coda reopened after closing it, opens the workspace while the session lasts. When the physician signs in from the link's tab, every other tab waiting on sign-in goes on to where it was. A tab catches up a moment after another tab signs in or out, or after Coda answers it.
- **When the session expires.** The session ends after 12 hours without a physician request, and always 7 days after sign-in. A sign-in prompt covers the screen and can email a link. After signing back in, the physician is exactly where they were, with drafts and the open reader intact, and Coda's running work never notices. Once the session has ended, reopening Coda asks them to sign in.
- **A patient link while signed out.** The tab holds the link while the physician signs in, then lands on that patient. A signed-out tab can't tell who signs in, so it opens the link for whoever does, and the practice's access rules decide what they see.
- **Signing out.** Sign-out signs out every tab and forgets where the physician was, including any patient link the tab held.
- **Another physician.** Opening another physician's sign-in link while signed in signs the current physician out first, then signs the new one in and opens Today. If another physician signs in in another tab, a tab that was yours becomes theirs and opens Today, never your patient. Opening your own link while signed in keeps your session, and the link's tab opens Today. Drafts and seen state stay with each physician.
- **Losing access.** If the practice removes a physician's access mid-session, they're signed out and see "No access to this practice".

## Rules

The model checks these in every state it reaches. The first three hold once the tab has caught up.

- Once you sign out, your workspace is gone from this tab.
- When your session expires mid-work, a sign-in prompt covers the screen.
- While you are signed in, Coda never asks you to sign in or tells you sign-in failed.
- An emailed sign-in link always opens Today.
- A patient link opened while signed out lands on that patient once you sign in.

These rest on stored records the model can't hold, so tests that read the records back hold them:

- An address with no account gets no email, and the answer looks the same either way: `an_unknown_or_inactive_email_never_receives_a_token` (physician-auth), and `an_unknown_email_never_confirms_an_account` with `hosted_sign_in_mails_the_link_and_omits_the_token` (physician-sample) give the same answer.
- A link signs in at most once: `a_sign_in_token_verifies_once_and_mints_one_session` and `concurrent_sign_in_consumes_one_token_and_mints_one_session`, on Postgres. A link expires: `an_expired_sign_in_token_is_invalid`, on the in-memory store only; no test covers expiry on Postgres yet.
- The emailed link carries no patient: `hosted_sign_in_mails_the_link_and_omits_the_token` checks the link is the code and email alone.
- Accounts are invite-only: `an_unknown_or_inactive_email_never_receives_a_token`; the site's only way to add a physician is `/auth/invitations`.
- After signing back in, the page is where it was: `an_ended_session_covers_the_page_and_signing_in_again_resumes_it`. No test yet shows drafts intact, or Coda's running work unaffected.
- Work stays with each physician: `missing_and_other_physician_drafts_are_not_created_or_exposed` for reply drafts on the server. No test yet covers seen state, or this tab's drafts when another physician signs in.

## Files

| File | Holds |
| --- | --- |
| [SignIn.tla](SignIn.tla), [SignIn.cfg](SignIn.cfg) | The design as a TLA+ model: the screens, the session the browser stores, what each of two tabs and Coda do, and the rules, one `INVARIANT` each. Each step names where the code carries it out. |

Check it from the repository root:

```bash
.agents/skills/statechart/scripts/statechart verification/physician/sign-in/SignIn.tla
```

The session the browser stores is the one owner of whether you are signed in, and the browser also counts its sign-outs. A tab follows them a moment late: news from another tab only says that something changed, so news can arrive late or out of order, and a tab that catches up after a sign-out still forgets where you were. The model lets other tabs, Coda's answers and expiry happen in any order. Signing out is one step for the browser: the tab forgets the stored session, counts the sign-out, tells the other tabs and asks Coda to end the session without waiting. TLC checks the rules in every state the model reaches; it doesn't reach the Rust itself. The code follows the model. Each bug path found so far is replayed in two tabs of one browser against the real page and server, and after each step both tabs must look as the model says:

```bash
CHROMIUM=<path to Chromium> cargo test -p physician-sample --test sign_in_model -- --ignored --test-threads=1
```
