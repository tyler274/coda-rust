---
name: coda-statechart
description: "Draw one Coda journey from the intent specs as a small Mermaid statechart, check its rules with search and Verus, and hand Dev the PNG. Invoke explicitly for journey design or diagram/model reconciliation."
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

3. Start with few states. Split a state only when the split separates two broken
   rules or the reader must see the step. Use one region for each thing that
   changes independently, such as the page and the session. Draw "the app closes"
   as an edge to what the reader sees next.
4. Write one `%% never` line per rule, in the spec's own sentence, ending with its
   ID. Never weaken a rule to make the chart pass. A promise about records
   (identity, revisions, collections, single use) goes in the folder's
   `data-promises.md` with the Rust owner that must prove it, marked unproven
   until it is.
5. Declare every event with `%% event`: who causes it, its `needs`, and the spec ID
   it comes from. The chart holds states, events and rules, and nothing else;
   explanation belongs in the spec.
6. Write `journeys.md`: the useful outcomes the spec walks through, one numbered
   step per event.

Keep each journey in its own folder under `verification/<product>/`. Charts
compose through shared declared events, never copied states; a composition file
holds the rules that span them.

## Check it

7. Run `lint`, then `trace` (`rules` when the chart is too large to exhaust), then
   `journey`. A broken rule in a chart Dev agreed is a design bug: take it to Dev.
8. Generate the model with `verus`, then run its proof with
   `verification/check.py formal`. Every chart gets one. When safety alone isn't
   inductive, strengthen `inductive()` in the proof file with a commented fact
   about reachable states. Run `check` to catch drift between the chart and the
   generated files.
9. Break a copy of the chart to match a real failure, and confirm that `trace` and
   Verus both fail on it.
10. Render it with `png` and look at the image. Each label should read as the
    screen, and each numbered path in order. If a label reads like code, fix it and
    render again.
11. Replay each bug against the real code, as a test or a live repro, one step per
    event. If a bug doesn't reproduce, assume the chart is wrong until you show
    otherwise.

## Hand off

Give Dev the PNG and an explorer: `explore` writes one HTML page holding the chart's
whole reachable graph, which you publish as a Claude artifact so Dev can click
through each step, rule and journey. Include the Mermaid. Add one line per bug: the rule it breaks, its
steps as `trace` numbers them, and whether it reproduced. Say what the chart leaves
out: data promises, GAPs, and anything not replayed against code. A checked draft is
still a draft until Dev agrees with it.

When Dev agrees with a journey's chart, delete what it replaces in the older
whole-app graph, with its coverage and journey rows, in the same change.

The [compiler reference](references/compiler.md) covers the supported Mermaid subset,
`%% when` guards, templates, commands and compositions, and what each command
establishes.

Adapted from [PStack statechart](https://github.com/cursor/plugins/blob/eb4db2b55b2b7dec3d9282754cddf0a56bf4ff10/pstack/skills/statechart/SKILL.md).
