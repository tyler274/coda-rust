# Physician sign-in

How a physician signs in to Coda Agents, drawn from
[physician.md A1.1–A1.5](../../../intent/physician.md#a1-sign-in-and-access). The chart
shows what one browser tab shows; access inside the workspace (A1.6 onward) is its own
journey.

| File | Holds |
| --- | --- |
| [sign-in.mmd](sign-in.mmd) | The chart, its declared events and its four rules. |
| [journeys.md](journeys.md) | What a physician can do, replayed against the chart. |
| [data-promises.md](data-promises.md) | Promises about records, with their Rust owners. |
| [verus/](verus/) | The generated Verus model and the editable proof. |
| [stateright.rs](stateright.rs) | The generated Stateright model, with the graph the chart search reaches. |

Check it from the repository root:

```bash
node .agents/skills/coda-statechart/scripts/statechart.mjs trace verification/physician/sign-in/sign-in.mmd
node .agents/skills/coda-statechart/scripts/statechart.mjs journey verification/physician/sign-in/sign-in.mmd verification/physician/sign-in/journeys.md
node .agents/skills/coda-statechart/scripts/statechart.mjs check verification/physician/sign-in/sign-in.mmd --module sign_in --dir verification/physician/sign-in/verus
python3 verification/check.py formal --spec verification/physician/sign-in/verus/sign_in_proof.rs
node .agents/skills/coda-statechart/scripts/statechart.mjs stateright verification/physician/sign-in/sign-in.mmd --out verification/physician/sign-in/stateright.rs --check
python3 verification/check.py stateright --chart physician/sign-in
node .agents/skills/coda-statechart/scripts/statechart.mjs png verification/physician/sign-in/sign-in.mmd --out target/verification/sign-in.png
node .agents/skills/coda-statechart/scripts/statechart.mjs explore verification/physician/sign-in/sign-in.mmd --journeys verification/physician/sign-in/journeys.md --title "Physician Sign-in" --out target/verification/sign-in-explorer.html
```

Click through every state and step in Stateright's Explorer at http://localhost:3000:

```bash
CARGO_TARGET_DIR=target/verification/stateright cargo run --release --manifest-path verification/stateright/Cargo.toml -- physician/sign-in serve localhost:3000
```

The chart, search, Stateright and proof cover its finite states only. The data promises are
unproven, and no code has been replayed against the chart yet.
