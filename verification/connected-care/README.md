# Connected care

How Duet and the physician web app meet. Each graph owns its side of one patient's
connection to one practice; [connection.mmd](connection.mmd) composes them through
shared events, never shared states, and holds the rules that span both.

| File | Owns |
| --- | --- |
| [connection.mmd](connection.mmd) | Which events cross between the graphs, and the rules across them. |
| [../duet-experience/duet.mmd](../duet-experience/duet.mmd) | The patient's side: invitation, joining and consent, leaving, deletion, and the live feed. |
| [../physician-experience/physician.mmd](../physician-experience/physician.mmd) | The practice's side: inviting, discharge, and the physician agents' work on what arrives. |
| [verus/](verus/) | The Verus model of the composition, and the proof that every rule holds across both graphs. |

The connection is live (Dev 2026-10-25): while connected, each change to what the
patient shares in Duet reaches the practice as "a shared Duet record arrives",
which makes Coda's work owed. Nothing crosses otherwise.

## Decided, not yet drawn

- **The practice writes back in Duet (Dev 2026-10-26).** When a patient is connected
  through Duet, the words a physician approves for them reach Duet's thread, marked as
  from their practice, instead of only their email or text. The communication standard
  holds: only the exact approved words go out. Gmail and Spruce stay the channel for
  patients not on Duet. It is drawn later as a connected graph on both sides, since the
  Duet graph holds only the MVP (Dev 2026-10-25).

Check it from the repository root; the composition reads both graphs:

```bash
node .agents/skills/coda-statechart/scripts/statechart.mjs lint verification/connected-care/connection.mmd
```

```bash
node .agents/skills/coda-statechart/scripts/statechart.mjs rules verification/connected-care/connection.mmd --max-cone 30
```

Rules say what must never happen; they can't say that an event must reach the other
graph. Check each handoff with a query instead: the physician's Coda work must be
among the reacting regions.

```bash
node .agents/skills/coda-statechart/scripts/statechart.mjs query verification/connected-care/connection.mmd --event "you answer a Check-in about your doctor's plan"
node .agents/skills/coda-statechart/scripts/statechart.mjs query verification/connected-care/connection.mmd --event "your Send to my practice on For my doctor goes through"
```

Lint must report no findings. Every rule written here must hold; each graph's own
rules hold as they do alone. The physician graph's `NoRevisionWithoutReply` stops at
the explicit search's state limit here and alone; the Verus proof below proves it,
with every other rule, in every reachable state.

## The Verus proof

`verus/` derives from the composition and both graphs. `connection.rs`,
`connection_steps.rs` and `connection.map.json` are generated and never edited by
hand; the map lists each composed graph with its hash, and `check` reports drift
when the composition or either graph changes. `connection_proof.rs` is the proof:
its `inductive()` is every rule (both graphs' and the composition's) plus each
graph's own support facts, carried over in the composed names. The shared events
need no support fact of their own.

After any change to either graph or the composition, regenerate the model, then
check the proof:

```bash
node .agents/skills/coda-statechart/scripts/statechart.mjs verus verification/connected-care/connection.mmd --module connection --dir verification/connected-care/verus
```

```bash
python3 verification/check.py formal --spec verification/connected-care/verus/connection_proof.rs
```

When a graph gains a support fact, add it here too, in the composed names
(`region_duet_...`, `region_physician_web_app_...`).
