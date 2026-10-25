# Physician experience

The physician web app's behavior, written as one statechart.

| File | Owns |
| --- | --- |
| [physician.mmd](physician.mmd) | The spec: the ideal behavior of the full product for every user. It covers what people see and how things act and work for them. |
| [decisions.md](decisions.md) | Why: Dev's dated decisions and the audit findings the graph cites. |
| [infrastructure-gaps.md](infrastructure-gaps.md) | What the graph leaves to the runtime, and what the runtime must still guarantee. |
| [verus/](verus/) | The Verus model derived from the graph, and the proof that its safety rules hold. |
| [sign-in.mmd](sign-in.mmd) | Signing in with an email link, in detail: its Session region details the graph's session, its Page region is what one browser tab shows, and its Patient link region is the Coda link that tab holds. It has no safety rules. |
| [sign-in.against.json](sign-in.against.json) | The map that connects sign-in.mmd to the graph, in the graph's words. |
| [evidence/](evidence/) | Screenshots from exercising the running app, such as the staging email sign-in on 2026-10-24. |

## How sign-in.mmd connects to the graph

The graph owns sign-in behavior. sign-in.mmd details it, and one map,
sign-in.against.json, labels sign-in.mmd's session states and steps in the
graph's words. The page and patient-link states map to nothing, because the
graph has no browser tabs. Check the connection from the repository root:

```bash
node .agents/skills/coda-statechart/scripts/statechart.mjs against verification/physician-experience/sign-in.mmd verification/physician-experience/physician.mmd --map verification/physician-experience/sign-in.against.json
```

It must report that sign-in.mmd follows the graph in both directions:

- every sign-in step is a graph step, or a page-only step that keeps the session;
- every graph step from a session state sign-in.mmd reaches is shown, unless the
  map's `omits` says why.

The omits are the open list of graph behavior sign-in doesn't do yet. None is
left: the expired prompt, the "No access to this practice" page and signing in
through a Coda link are built (Dev 2026-10-24, decisions.md).

When one is built, add it to sign-in.mmd and remove its omit. A change to how
signing in behaves goes in the graph first, and sign-in.mmd and the map follow.

## What the graph is

The graph is the spec. It is written from Dev's intent and the settled sources,
never from existing code. Verification models derive from it.

It sets hard limits only on four things. Each limit names the line it protects,
and everything Coda judges is a "may" (decisions.md, Charter):

- access;
- communication standards: Coda never sends on its own, and only the exact words
  a physician approved go out;
- data and consent;
- owner consistency: one owner per fact.

It describes the ideal state. Runtime capacity, retries, timeouts, interruptions
and sync are left out and listed in infrastructure-gaps.md. Failures from the
outside world stay in the graph.

## Changing it

Change the behavior in physician.mmd and record the reason in decisions.md in the
same commit. When a source is silent or two sources conflict, ask Dev, one
question at a time. The graph's syntax and the tool are described in
[the statechart skill](../../.agents/skills/coda-statechart/SKILL.md) and its
[compiler reference](../../.agents/skills/coda-statechart/references/compiler.md).

Check every change from the repository root:

```bash
node .agents/skills/coda-statechart/scripts/statechart.mjs lint verification/physician-experience/physician.mmd
```

```bash
node .agents/skills/coda-statechart/scripts/statechart.mjs rules verification/physician-experience/physician.mmd --max-cone 30
```

Lint must report no findings, and rules must report 0 violated. `rules` checks
each `%% never` rule on the smallest group of regions that can affect it. A rule
reported incomplete hit the search limit; raise `--max-states` for it.

`compare` shows how two versions of the graph differ as flat models. Use it to
confirm that a refactor keeps the same behavior. `query` shows the ways in and
out of a state and which regions react to an event.

## The Verus proof

`verus/` derives from the graph. `physician.rs`, `physician_steps.rs` and
`physician.map.json` are generated and never edited by hand.
`physician_proof.rs` is the proof: its `inductive()` is the graph's safety rules
plus a few support facts that induction needs. Each fact has a plain-words
comment, and each is proved along with the rules.

After any graph change, regenerate the model, then check the proof:

```bash
node .agents/skills/coda-statechart/scripts/statechart.mjs verus verification/physician-experience/physician.mmd --module physician --dir verification/physician-experience/verus
```

```bash
python3 verification/check.py formal --spec verification/physician-experience/verus/physician_proof.rs
```

The proof covers every reachable state, which the `rules` search can only bound.
A failure names the event whose step breaks the invariant. Replay the path on
the graph before changing anything: either the graph has a real gap to fix, or
the proof needs one more true support fact. Never weaken a rule to make it pass.
`statechart.mjs check` (same arguments as `verus`) reports a generated file that
no longer matches the graph.
