# Physician sign-in

How a physician signs in to Coda Agents, drawn from
[physician.md A1.1–A1.5](../../../intent/physician.md#a1-sign-in-and-access). The chart
shows what one browser tab shows; access inside the workspace (A1.6 onward) is its own
journey.

| File | Holds |
| --- | --- |
| [sign-in.mjs](sign-in.mjs) | The chart as an XState machine: its regions, events and four rules. |
| [journeys.md](journeys.md) | What a physician can do, replayed against the chart. |
| [data-promises.md](data-promises.md) | Promises about records, with their Rust owners. |
| [verus/](verus/) | The generated Verus model and the editable proof. |

Check it from the repository root, after `npm install --prefix verification`:

```bash
node verification/statechart.mjs check verification/physician/sign-in/sign-in.mjs verification/physician/sign-in/journeys.md
node verification/statechart.mjs verus verification/physician/sign-in/sign-in.mjs --module sign_in --check
python3 verification/check.py formal --spec verification/physician/sign-in/verus/sign_in_proof.rs
```

To click through it, paste [sign-in.mjs](sign-in.mjs) into a local
[Stately Sketch](https://github.com/statelyai/sketch).

The chart, search and proof cover its finite states only. The data promises are
unproven, and no code has been replayed against the chart yet.
