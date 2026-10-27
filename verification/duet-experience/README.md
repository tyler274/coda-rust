# Duet experience

The Duet app's behavior, written as one statechart. It follows the physician
graph's method ([physician-experience](../physician-experience/README.md)).

| File | Owns |
| --- | --- |
| [duet.mmd](duet.mmd) | The spec: the ideal behavior of the full product for every person. |
| [decisions.md](decisions.md) | Why: Dev's dated decisions and the sources the graph cites. |
| [infrastructure-gaps.md](infrastructure-gaps.md) | What the graph assumes away and the runtime must still guarantee. |
| [journeys.md](journeys.md) | Useful outcomes a person reaches, replayed from the graph's start by `statechart.mjs journey`; it fails when the graph stops allowing one. |
| [coverage.md](coverage.md) | What the Rust Duet app shows today for each graph state: built, partial, missing or not shown, and whether the wire carries the data. `statechart.mjs coverage` checks it against the graph. |
| [verus/](verus/) | The Verus model derived from the graph, and the proof that its safety rules hold. |
| [../connected-care](../connected-care/README.md) | How this graph meets the physician graph: shared events and the rules across them. |

Check every change from the repository root:

```bash
node .agents/skills/coda-statechart/scripts/statechart.mjs lint verification/duet-experience/duet.mmd
```

```bash
node .agents/skills/coda-statechart/scripts/statechart.mjs rules verification/duet-experience/duet.mmd --max-cone 30
```

Lint must report no findings, and rules must report 0 violated.

## The Verus proof

`verus/` derives from the graph. `duet.rs`, `duet_steps.rs` and `duet.map.json`
are generated and never edited by hand. `duet_proof.rs` is the proof: its
`inductive()` is the graph's safety rules plus a few support facts that
induction needs. Each fact has a plain-words comment, and each is proved along
with the rules.

After any graph change, regenerate the model, then check the proof:

```bash
node .agents/skills/coda-statechart/scripts/statechart.mjs verus verification/duet-experience/duet.mmd --module duet --dir verification/duet-experience/verus
```

```bash
python3 verification/check.py formal --spec verification/duet-experience/verus/duet_proof.rs
```

The proof covers every reachable state, which the `rules` search can only bound.
A failure names the event whose step breaks the invariant. Replay the path on
the graph before changing anything: either the graph has a real gap to fix, or
the proof needs one more true support fact. Never weaken a rule to make it pass.
`statechart.mjs check` (same arguments as `verus`) reports a generated file that
no longer matches the graph.
