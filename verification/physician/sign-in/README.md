# Physician sign-in

This folder is the spec for how a physician signs in to Coda Agents. The chart is the agreed behavior, the journeys tell it as stories, and the proof shows the chart keeps its rules. [physician.md](../../../intent/physician.md#a1-sign-in-and-access) links here.

## How sign-in works

Physicians sign in only through Coda Agents, with a link Coda emails them. There's no password and no code screen. Accounts are invite-only: Coda sets up each practice and invites its first physician by email, and any member can then invite colleagues.

- **Asking for a link.** The physician enters their work email and always sees "Check your email". Coda never reveals whether an account exists, and an address with no account gets no email. The email never carries a patient.
- **Opening the link.** Clicking the button in the email signs them in and opens Today. A used or expired link says "This link no longer works", with a way to ask for a new one or use a different email. A link that no longer grants access says "No access to this practice". If Coda can't send or confirm the link, it says so and offers to try again.
- **One session across tabs.** The session belongs to the browser, so a new tab, or Coda reopened after closing it, opens the workspace while the session lasts. When the physician signs in from the link's tab, every other tab waiting on sign-in goes on to where it was.
- **When the session expires.** A sign-in prompt covers the screen and can email a link. After signing back in, the physician is exactly where they were, with drafts and the open reader intact, and Coda's running work never notices. Once the session has ended, reopening Coda asks them to sign in.
- **A patient link while signed out.** The tab holds the link while the physician signs in, then lands on that patient.
- **Signing out.** Sign-out signs out every tab and forgets where the physician was, including any patient link the tab held.
- **Another physician.** Opening another physician's sign-in link while signed in signs the current physician out first, then signs the new one in and opens Today. If another physician signs in in another tab, this tab becomes theirs and opens Today, never a patient this tab held for someone else. Opening your own link while signed in changes nothing. Drafts and seen state stay with each physician.
- **Losing access.** If the practice removes a physician's access mid-session, they're signed out and see "No access to this practice".

## Files

| File | Holds |
| --- | --- |
| [sign-in.mjs](sign-in.mjs) | The chart as an XState machine: its regions, events and rules. |
| [journeys.md](journeys.md) | The behavior as stories, replayed against the chart. |
| [code.mjs](code.mjs) | What the physician app's sign-in code does, drawn in the chart's words for comparison. |
| [data-promises.md](data-promises.md) | Promises about records rather than screens, with their Rust owners. |
| [verus/](verus/) | The Verus model generated from the chart, and its proof. |

Check it from the repository root, after `npm install --prefix verification`:

```bash
node verification/statechart.mjs check verification/physician/sign-in/sign-in.mjs verification/physician/sign-in/journeys.md
node verification/statechart.mjs compare verification/physician/sign-in/sign-in.mjs verification/physician/sign-in/code.mjs
node verification/statechart.mjs verus verification/physician/sign-in/sign-in.mjs --module sign_in --check
python3 verification/check.py formal --spec verification/physician/sign-in/verus/sign_in_proof.rs
```

To click through it, paste [sign-in.mjs](sign-in.mjs) into a local [Stately Sketch](https://github.com/statelyai/sketch).

The chart, its search and its proof cover the chart's finite states, not the code. The compare and the replays against the running app are what check the code.
