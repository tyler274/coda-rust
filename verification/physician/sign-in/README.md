# Physician sign-in

How a physician signs in to Coda Agents, drawn from
[physician.md A1.1–A1.5](../../../intent/physician.md#a1-sign-in-and-access). The chart
shows what one browser tab shows; access inside the workspace (A1.6 onward) is its own
journey.

| File | Holds |
| --- | --- |
| [sign-in.mjs](sign-in.mjs) | The chart as an XState machine: its regions, events and five rules. |
| [journeys.md](journeys.md) | What a physician can do, replayed against the chart. |
| [code.mjs](code.mjs) | What the physician app's sign-in code does at d878a812, in the chart's words. |
| [data-promises.md](data-promises.md) | Promises about records, with their Rust owners. |
| [verus/](verus/) | The generated Verus model and the editable proof. |
| [evidence-2026-10-24/](evidence-2026-10-24/) | Staging screenshots of sign-in from 2026-10-24, taken before this chart existed. |

Check it from the repository root, after `npm install --prefix verification`:

```bash
node verification/statechart.mjs check verification/physician/sign-in/sign-in.mjs verification/physician/sign-in/journeys.md
node verification/statechart.mjs compare verification/physician/sign-in/sign-in.mjs verification/physician/sign-in/code.mjs
node verification/statechart.mjs verus verification/physician/sign-in/sign-in.mjs --module sign_in --check
python3 verification/check.py formal --spec verification/physician/sign-in/verus/sign_in_proof.rs
```

To click through it, paste [sign-in.mjs](sign-in.mjs) into a local
[Stately Sketch](https://github.com/statelyai/sketch).

The chart, search and proof cover its finite states only. `compare` shows where the
code differs from the chart; each difference waits on Dev's call, and none has been
replayed against the running app yet. The data promises are unproven, and the
staging screenshots predate the chart.
