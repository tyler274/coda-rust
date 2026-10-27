# Duet experience

The Duet app's behavior, written as one statechart. It follows the physician
graph's method ([physician-experience](../physician-experience/README.md)).

| File | Owns |
| --- | --- |
| [duet.mmd](duet.mmd) | The spec: the ideal behavior of the full product for every person. |
| [decisions.md](decisions.md) | Why: Dev's dated decisions and the sources the graph cites. |
| [infrastructure-gaps.md](infrastructure-gaps.md) | What the graph assumes away and the runtime must still guarantee. |
| [../connected-care](../connected-care/README.md) | How this graph meets the physician graph: shared events and the rules across them. |

Check every change from the repository root:

```bash
node .agents/skills/coda-statechart/scripts/statechart.mjs lint verification/duet-experience/duet.mmd
```

```bash
node .agents/skills/coda-statechart/scripts/statechart.mjs rules verification/duet-experience/duet.mmd --max-cone 30
```

Lint must report no findings, and rules must report 0 violated.
