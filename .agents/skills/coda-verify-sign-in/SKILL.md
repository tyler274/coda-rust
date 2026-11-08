---
name: coda-verify-sign-in
description: "Check a change to physician sign-in against its TLA+ model: TLC on the agreed rules, then the model's bug paths replayed in two browser tabs against the real page. Use when a change touches the physician app's session, sign-in or tab code, the auth server's answers, or the sign-in spec or model."
---

# Verify physician sign-in

The spec is [verification/physician/sign-in/README.md](../../../verification/physician/sign-in/README.md)
and the model is [SignIn.tla](../../../verification/physician/sign-in/SignIn.tla). Each `INVARIANT`
or `PROPERTY` in [SignIn.cfg](../../../verification/physician/sign-in/SignIn.cfg) is one of the README's
rules. The harm when one breaks: a physician sees another session's work, isn't covered when their
session ends, is asked to sign in while signed in, lands somewhere other than the agreed place, or has Coda act as a physician the tab doesn't show.

## When a change needs this

Run it when a change touches any of these. Each action's comment in `SignIn.tla` names the code it
models.

- `crates/physician-app/src/lib.rs`: `SessionState` (`catch_up`, `refused`, `end`, `no_session`),
  `PhysicianRoot`, `PhysicianShell`'s `/today` read and its answer, and `SignInScreen` (the link
  effect, `verify_link`, `request_token`).
- `crates/physician-app/src/tabs.rs` (tab news) and `crates/physician-app/src/main.rs`
  (`?patient=`, the sign-in link).
- `crates/physician-client/src/lib.rs` (the session a client presents, on reads and on every
  request a workspace makes).
- `crates/physician-auth/src/lib.rs` (the `AuthStore` session check and sign-out, and the
  in-memory store's) and `crates/physician-auth/src/store.rs` (the same on Postgres).
- `crates/physician-sample/src/auth.rs` (the session check's refusals, `access_removed` among
  them, and sign-out).
- The sign-in README, `SignIn.tla` or `SignIn.cfg`.

## Run

Check the model, and read every result, with the pstack `/statechart` skill on
`verification/physician/sign-in/SignIn.tla`. It passes when the report lists OnceOut, Expiry,
KeepsWork, SignedIn, TodayFirst, Lands, ActsAsShown, PromptIsEmpty, ServerRefusesSignedOut, CatchesUp and NoDeadlock under the rules that hold. The run is long, most of it checking CatchesUp, so run it in the
background.
Then draw each chart, render it as the skill says, and read it: `chart.cfg` with `chart.mjs` on
`SignIn.tla` draws one tab with one session, and `chart-sign-in`, `chart-another`, `chart-expiry`
and `chart-sign-out`, each `.cfg` with its `.mjs` on `Charts.tla`, draw two tabs when you sign in,
another physician signs in, the session expires, or you sign out. Every rule must hold in each,
except CatchesUp, which needs the full model's fairness.

Then, from the repository root:

```sh
CHROMIUM=<path to Chromium> cargo test -p physician-sample --test sign_in_model -- --ignored --test-threads=1
```

It replays each path in `BUG_PATHS` in `sign_in_model.rs`, in two tabs of one browser against the
real page and sample server, and compares both tabs with the model after every step.

A change to how the page looks or to what a stage offers also needs its stages walked:

```sh
CHROMIUM=<path to Chromium> cargo test -p physician-sample --test sign_in_states -- --ignored
```

It reaches every stage at desktop and phone sizes in light and dark, and checks each stage's words,
its way on and that nothing scrolls sideways. With `SIGN_IN_SHOTS` set to an absolute folder it also
saves each stage as a screenshot, to compare two revisions.

## Bounds and assumptions

Two tabs and three sessions; the charts, one or two tabs and one or two sessions. Each wait in the code is one step, and the model's header says
what it takes as one step and the timing it assumes. TLC covers every state of that finite model;
the browser test covers only the bug paths, not every path the code can take. No code holds
ActsAsShown yet: a workspace's requests carrying its tab's session. CatchesUp is the one rule about
what eventually happens: it assumes Coda and the browser eventually answer whatever a tab waits on,
and it holds once you stop acting. Signing out assumes Coda receives the request and ends the
session: the page doesn't wait for or retry it, so a request that never arrives, or that Coda
answers with 503 because its store is down, leaves the session live at Coda for any tab that
hasn't heard the news yet.

## Promises tests hold

These rest on stored records the model can't hold, so tests that read the records back hold them:

- An address with no account gets no email, and the answer looks the same either way: `an_unknown_or_inactive_email_never_receives_a_token` (physician-auth), and `an_unknown_email_never_confirms_an_account` with `hosted_sign_in_mails_the_link_and_omits_the_token` (physician-sample) give the same answer.
- A link signs in at most once: `a_sign_in_token_verifies_once_and_mints_one_session` and `concurrent_sign_in_consumes_one_token_and_mints_one_session`, on Postgres. A link expires: `an_expired_sign_in_token_is_invalid`, on the in-memory store only; no test covers expiry on Postgres yet.
- The emailed link carries no patient: `hosted_sign_in_mails_the_link_and_omits_the_token` checks the link is the code and email alone.
- Accounts are invite-only: `an_unknown_or_inactive_email_never_receives_a_token`; the site's only way to add a physician is `/auth/invitations`.
- A session end clears the patient page and stored patient content, and signing back in opens Today: `session_end_clears_patient_content` and `an_ended_session_replaces_the_page_and_signing_in_again_opens_today`. No test yet shows Coda's running work unaffected.
- Work stays with each physician: `missing_and_other_physician_drafts_are_not_created_or_exposed` for reply drafts on the server. No test yet covers seen state, or this tab's drafts when another physician signs in.

## A failure

- **TLC breaks a rule.** The report gives the shortest path. Add it to `BUG_PATHS` and run the
  browser test. If every step can happen in the code and the code shows the same, the design breaks
  the rule: take it to the user, and never weaken the rule.
- **The page differs from the model on a path.** The message names the step and the difference. If
  the code does what the README says and the model doesn't, the model is wrong: fix the action from
  the README's words. If the code differs from both, it's a bug: keep the path in `BUG_PATHS`, see
  it fail, then fix the code. A step the driver can't take or a page that never loads is the driver.
- **An intended change.** Change the README with the user first, then the model, then the code.

## Checks seen failing

With two tabs and three sessions, each rule fails on its broken copy of the model:

| Rule | Broken copy |
| --- | --- |
| OnceOut | a tab that missed a sign-out keeps what it showed when it catches up |
| Expiry | an ended session's answer leaves the workspace showing |
| KeepsWork | an ended session's answer sends the workspace to the sign-in page |
| SignedIn | a tab on a sign-in page doesn't follow another tab's sign-in |
| TodayFirst | a sign-in link page keeps the place the tab was at |
| TodayFirst | Coda's answer about who signed in opens the workspace at a patient |
| Lands | a tab holding a patient link opens Today when its read answers |
| Lands | a tab still opening without a session forgets its patient link when another tab signs in |
| Lands | another physician's link signs out a session that has already ended |
| ActsAsShown | Coda acts on the session the browser stores rather than the one the tab holds |
| ActsAsShown | Coda acts on any live session the tab holds, whichever physician the browser has signed in |
| PromptIsEmpty | an ended session's answer shows the prompt and keeps the patient page |
| ServerRefusesSignedOut | signing out with nothing stored leaves the tab's own session live at Coda |
| ServerRefusesSignedOut | another physician's link signs you out and leaves your session live at Coda |
| CatchesUp | a tab at the prompt never hears the news |

A path joins `BUG_PATHS` only after it fails on the code it guards. After a change to the model or a rule,
break it again the same way and confirm TLC reports that rule.

## Sync

The model and bug paths were last confirmed to describe the code above at commit `dc77bcc2`. Move this
forward only after reading each change to that code since then against the model and running both
checks under Run.
