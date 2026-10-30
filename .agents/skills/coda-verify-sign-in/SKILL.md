---
name: coda-verify-sign-in
description: "Check a change to physician sign-in against its TLA+ model: TLC on the five agreed rules, then the model's walks and bug paths in two browser tabs against the real page. Use when a change touches the physician app's session, sign-in or tab code, the auth server's answers, or the sign-in spec or model."
---

# Verify physician sign-in

The spec is [verification/physician/sign-in/README.md](../../../verification/physician/sign-in/README.md)
and the model is [SignIn.tla](../../../verification/physician/sign-in/SignIn.tla). Each `INVARIANT`
in [SignIn.cfg](../../../verification/physician/sign-in/SignIn.cfg) is one of the README's five
rules. The harm when one breaks: a physician sees another session's work, isn't covered when their
session ends, is asked to sign in while signed in, or lands somewhere other than the agreed place.

## When a change needs this

Run it when a change touches any of these. Each action's comment in `SignIn.tla` names the code it
models.

- `crates/physician-app/src/lib.rs`: `SessionState` (`catch_up`, `refused`, `end`, `no_session`),
  `PhysicianRoot`, `PhysicianShell`'s `/today` read and its answer, and `SignInScreen` (the link
  effect, `verify_link`, `request_token`).
- `crates/physician-app/src/tabs.rs` (tab news) and `crates/physician-app/src/main.rs`
  (`?patient=`, the sign-in link).
- `crates/physician-client/src/lib.rs` (the session a client presents).
- `crates/physician-auth/src/store.rs` (session expiry and sign-out) and
  `crates/physician-sample/src/auth.rs` (the `access_removed` refusal).
- The sign-in README, `SignIn.tla` or `SignIn.cfg`.

## Run

From the repository root:

```sh
.agents/skills/statechart/scripts/statechart verification/physician/sign-in/SignIn.tla
CHROMIUM=<path to Chromium> cargo test -p physician-sample --test auth_browser -- --ignored --test-threads=1
```

The first command passes when the report lists OnceOut, Expiry, SignedIn, TodayFirst, Lands and
NoDeadlock under "Rules that hold". TLC reaches 1,626,132 states in about two minutes, more than
the chart's budget, so it draws a chart only for a rule that breaks. The second plays, in two tabs
of one browser against the real page and sample server, the shortest walk to each kind of step
`COVER` in `auth_browser.rs` tells apart, 100 random 12-step walks, and the bug paths in
`BUG_PATHS`, and compares both tabs with the model after every step. It takes about six minutes.

## Bounds and assumptions

Two tabs and up to three sessions. Each wait in the code is one step, and the model's header says
what it takes as one step and the timing it assumes. TLC covers every state of that finite model;
the walks cover the steps they take, not every order. Safety only: no rule says anything must
eventually happen. The README lists the promises that rest on records the model can't hold, and
the tests that hold them.

## A failure

- **TLC breaks a rule.** The report gives the shortest path. Replay it in the browser with
  `node verification/walks.mjs verification/physician/sign-in/SignIn.tla --paths '[[...]]'` and
  the bug-path test. If every step can happen in the code and the code shows the same, the design
  breaks the rule: take it to Dev, and never weaken the rule.
- **A walk differs from the model.** The message names the step and the difference. If the code
  does what the README says and the model doesn't, the model is wrong: fix the action from the
  README's words. If the code differs from both, it's a bug: add the path to `BUG_PATHS`, see it
  fail, then fix the code. A step the driver can't take or a page that never loads is the driver.
- **An intended change.** Change the README with Dev first, then the model, then the code.

## Checks seen failing

At the sync commit, each rule failed on a broken copy of the model, as the README sentence warns:

| Rule | Broken copy |
| --- | --- |
| OnceOut | a tab that missed a sign-out keeps what it showed when it catches up |
| Expiry | an ended session's answer leaves the workspace showing |
| SignedIn | a tab on a sign-in page doesn't follow another tab's sign-in |
| TodayFirst | a sign-in link page keeps the place the tab was at |
| Lands | a tab holding a patient link opens Today when its read answers |
| Lands | a tab still opening without a session forgets its patient link when another tab signs in |

Each path in `BUG_PATHS` failed on the code before its fix. After a change to the model or a rule,
break it again the same way and confirm TLC reports that rule.

## Sync

The model and walks were last confirmed to describe the code above at commit `d24e027d`. Move this
forward only after reading each change to that code since then against the model and running both
commands.
