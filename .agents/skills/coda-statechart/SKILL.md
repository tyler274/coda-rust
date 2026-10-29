---
name: coda-statechart
description: "Draw one Coda journey from the intent specs as a small XState machine, check its rules and journeys with XState's graph search, prove it with Verus, and show Dev in Stately Sketch. Invoke explicitly for journey design or chart/model reconciliation."
disable-model-invocation: true
---

# Draw one journey

Follow the [shared principles](../../../verification/README.md#principles).

The chart shows what the person using Coda sees, so Dev can read it and decide
whether that behavior is right. One chart answers one journey question, such as
"how does a physician sign in?". Label each state with what is on the screen, such
as "Check your email", and each step with what the person does or sees happen,
such as "open the email button". Keep code names out of labels.

## Start from the spec

1. Read the journey's section of the [intent specs](../../../intent/README.md) and
   [PRINCIPLES.md](../../../intent/PRINCIPLES.md). They are the only source. A chart
   cites spec lines by ID, such as `physician A1.3`, never transcripts, dates,
   Linear or other charts. A step the spec doesn't state, or marks `GAP:`, stays
   out of the chart and is named in the hand-off; a product choice goes to
   DECISIONS.md and Dev. Existing code is evidence about the implementation;
   missing code never narrows the chart.
2. A state or event belongs only when the reader sees or controls it, it is a
   limit on access, consent, data or communication, another party depends on it,
   or it orders what happens. Judgment, memory, prompts, schedules, retries and
   queues are harness, and stay out.

## Draw it

The chart is an XState machine in `<journey>.mjs` that exports `machine`. The same
file opens in [Stately Sketch](https://github.com/statelyai/sketch) and runs in
[statechart.mjs](../../../verification/statechart.mjs).

3. Start with few states. Split a state only when the split separates two broken
   rules or the reader must see the step. Make the machine `type: 'parallel'`, with
   one region for each thing that changes independently, such as the page and the
   session, and flat states inside each region. Name regions, states and events in
   the reader's words: the state is the screen, the event is what happens. Draw
   "the app closes" as an event to what the reader sees next.
4. Name every guard in `setup({ guards })` in plain words, such as `'signed in'`,
   built only from `stateIn({ region: state })`, `not`, `and` and `or`. An event
   that needs a state carries that guard on each of its transitions.
5. In the machine's `meta.events`, give every event who causes it (`by`) and its
   intent, ending with the spec ID it comes from.
6. In `meta.never`, write one rule per spec sentence: the states that must never
   hold together, by region (a list where any of several states breaks it), and the
   spec's own sentence ending with its ID. Anything outside steps 3–6 (`always`,
   `after`, `entry`, actions, context, a root that is not parallel, region-level or
   cross-region transitions) is refused, because Verus would not see it. Never weaken a rule to make the chart
   pass. A promise about records (identity, revisions, collections, single use) goes
   in the folder's `data-promises.md` with the Rust owner that must prove it, marked
   unproven until it is.
7. Write `journeys.md`: the useful outcomes the spec walks through, one numbered
   step per event, as `1. event → State; State`, with `From: Journey` to continue
   another journey.

Keep each journey in its own folder under `verification/<product>/`.

## Check it

From the repository root, after `npm install --prefix verification`:

8. `node verification/statechart.mjs check CHART.mjs journeys.md`. XState's graph
   search reaches every state, prints the shortest path to any broken rule, and
   replays each journey on the machine. A broken rule in a chart Dev agreed is a
   design bug: take it to Dev.
9. `node verification/statechart.mjs verus CHART.mjs --module NAME` writes the
   Verus model next to the chart, then `python3 verification/check.py formal
   --spec .../NAME_proof.rs` proves it. When safety alone isn't inductive,
   strengthen `inductive()` in the proof file with a commented fact about
   reachable states. `verus ... --check` fails when the model no longer matches
   the chart.
10. Break a copy of the chart to match a real failure, and confirm that `check`
    and Verus both fail on it.
11. Draw what the code does as `code.mjs` beside the chart: the same regions and
    the chart's words, with the code's own guards, plus any steps and screens only
    the code has. Give each event a `code` note naming where the code does it, and
    name the commit it reads in the file's header. Make each step one indivisible
    step in the code, with every region's part of it guarded by the screen the code
    needs. Then `node verification/statechart.mjs compare CHART.mjs code.mjs`
    prints the states and steps only one of them has, and every agreed rule the
    code breaks with its shortest path. For each difference, ask Dev whether the
    code is wrong or the chart needs a correction.
12. Replay each bug against the real code, as a test or a live repro, one step per
    event. If a bug doesn't reproduce, assume the chart is wrong until you show
    otherwise.

## Hand off

Open the chart in Stately Sketch: clone it outside this repository, run
`pnpm install && pnpm dev`, paste the chart into its Code panel and press Update.
Its simulation outlines the current state in every region and lights only the steps
allowed now. Send Dev screenshots of the journeys and of each bug as Sketch shows
them. Add one line per bug: the rule it breaks, its steps as `check` numbers them,
and whether it reproduced. Say what the chart leaves out: data promises, GAPs, and
anything not replayed against code. A checked draft is still a draft until Dev
agrees with it.

Adapted from [PStack statechart](https://github.com/cursor/plugins/blob/eb4db2b55b2b7dec3d9282754cddf0a56bf4ff10/pstack/skills/statechart/SKILL.md).
