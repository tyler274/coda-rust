# Verification

The [intent specs](../intent/README.md) say what Coda does. This folder checks it, one feature at a
time: each feature gets a precise spec, a TLA+ model of that spec, and a way to hold the code to
the model. A feature's README can come before its model; until the model exists, its verify skill
names the tests that hold each rule, as [clinical-library/nightly](clinical-library/nightly/README.md)'s
does. Features without a folder follow the intent specs alone.

## A feature's folder

`verification/<product>/<feature>/` holds:

- **`README.md`:** the feature's spec in plain words, with nothing about how it's checked. The
  intent spec keeps a short summary and links here. Its sections, in order:
  - **Intro:** what the spec covers, and the link to the intent spec.
  - **Words:** each word a rule or step depends on, defined once, so each rule has one
    meaning and maps to one thing in the model.
  - **States and steps:** the feature's graph, as tables of from, step and to. Each step is one
    action in the model, and each state is one value of it.
  - **Rules:** the invariants, numbered, one sentence each, with every condition the check uses
    written into the sentence. A rule that rests on code or records rather than on the order of
    steps follows them, marked as one the model doesn't check.
  - **Outside this spec:** what other specs own.
- **`<Feature>.tla` and `.cfg`:** the model, written from the README, not from the code. Its
  header says what it takes as one step, its scope, and what it leaves to records. Each
  `INVARIANT` or `PROPERTY` is one rule, quoting its sentence word for word with nothing added.
  Each action's comment names the code that carries it out. An `event` variable names each
  step, and the config's `VIEW` leaves it out. The config sets the scope the rules are checked
  at: large enough for the races the feature's risks come from, never cut to fit a chart.
- **`chart*.cfg` and `chart*.mjs`:** the charts PStack's `statechart` skill draws, in the words
  of the person who decides whether the behavior is right. Each runs the same model at a scope
  small enough that `statechart` draws the whole chart: `chart.cfg` from the start, and the
  others from a moment `Charts.tla` sets, with only the steps that chart shows. Each `.mjs`
  names its states and steps, and says what its chart leaves out.

## How a feature is checked

1. **Agree first.** Agree the feature's flows and rules before modeling them, and keep what the
   code does today separate from what was agreed.
2. **Check the model.** PStack's `statechart` skill runs TLC, which visits every state the model
   can reach and checks every rule. A deliberately broken copy of the model shows each rule can
   fail. The skill also draws the charts, which is how the flows are read and agreed.
3. **Hold the code to it.** The code follows the model. When TLC or review finds a path on which
   the code breaks a rule, that path goes into the feature's browser test. There,
   [replay.mjs](replay.mjs) gives the model's state after each step, and the test plays the path
   in the real page and compares. The test must fail on the code before its fix.
4. **Settle differences.** A difference the model can't settle is a product decision, not a code
   fix.

Each feature's browser test lives in its own file named for the feature, such as
`sign_in_model.rs`, and runs locally and in manually requested CI browser qualification.
The dev shell supplies `mise`; the replay runner installs the statechart skill's pinned TLC
and Java when they are absent. The `statechart` skill gives its own command and setup; the
feature's verify skill names the model, its charts and its browser test.

[`model_text.rs`](model_text.rs) holds the test-only guard for skill text against the tools a
job offers. Chords includes it only in its test builds.

## Local model runs

Use the existing [statechart runner](../.agents/skills/statechart/SKILL.md), including for a
model too large to draw. Keep the runner's own settings; a separate batch of direct Java
invocations can multiply both the work and the memory used.

Run one model at a time. This invocation caps the Java heap at 4 GiB, limits the JVM's
processor and garbage-collector thread counts, and stops after 20 minutes. The output
directory must be new or empty. Use the skill's Java and TLC setup; `JAVA` and `TLA2TOOLS`
can select an existing installation.

```sh
JAVA_TOOL_OPTIONS='-Xmx4g -XX:ActiveProcessorCount=2 -XX:+UseParallelGC -XX:ParallelGCThreads=2' \
python3 -c '
import subprocess, sys
try:
    result = subprocess.run(sys.argv[1:], timeout=1200)
except subprocess.TimeoutExpired:
    print("Model check timed out; incomplete.", file=sys.stderr)
    sys.exit(124)
sys.exit(result.returncode)
' \
  .agents/skills/statechart/scripts/statechart \
  verification/physician/message-reply/MessageReply.tla \
  --out target/verification/message-reply
```

The heap limit is not a limit on all process memory. A timeout exits with status 124. A timeout or resource failure is an
incomplete check, never a pass. Preserve the command and available output, then stop; do not
automatically retry, increase the heap, or launch other cases. Investigate the model and
execution cost before agreeing a larger budget. Do not reduce model bounds or remove rules
to fit the budget. The runner removes temporary state on interruption, so an interrupted run
may have no TLC report. For a deliberate break, use the runner's documented failure exit and
inspect the counterexample to establish that the intended rule failed.

For a correction, check every rule at the default bounds and show the changed paths are
reachable. Select deliberate breaks for changed rule semantics and failure modes introduced
by changed actions; run each against its rule alone and against the other rules. Record why
the selection exercises the correction and how the default run covers neighboring guarantees.
Read affected charts and check the README-to-action mapping.
Independent review must leave no unresolved finding that affects the correction. Reuse
completed evidence only when its inputs and relevant dependencies still match the change.

Full qualification also runs every deliberate break, every chart and every larger bound
listed by the feature's skill. Scope and budget that work explicitly. Report the exact
scope completed; a checked correction does not establish full qualification, and an
unfinished qualification matrix does not authorize an automatic local batch.

## Verify skills

A verify skill names the commands, expected results and evidence that hold one feature to
its spec, and the code whose changes call for it. When a change touches that code, its
acceptance evidence comes from the skill. A change outside every skill's code proves itself
with focused tests and the observations its criteria name.

Each verify skill lives in `.agents/skills/coda-verify-*/`, with a `.claude/skills` link. Its
description says what it proves, and its "When a change needs this" section names the code that
calls for it.

## Scenario receipts

`scripts/scenarios.py` runs the product journeys that work on one machine and writes
`receipt.json` under `target/verification/runs/`. P1, a physician asking about a patient, runs over
HTTP against the [scale stack](../scripts/physician-scale-stack.py). That stack has two sites, the
worker answering through the local Responses stub, Postgres and Valkey.
D1, a Duet account, migrates a fresh database on the same Postgres through `duet-migrate` with a separate runtime
login, then drives the `duet-api` binary over HTTP: sign-up, a stored health record, a kill and
restart, two-account refusal, and sign out everywhere. The binary runs with its local identity
stand-in, `DUET_LOCAL_IDENTITY_SECRET` (HS256 tokens the runner signs); hosted startup refuses it
and the other local stand-ins. D1 then runs the duet-api Postgres suites. The run reads no credentials.

```sh
nix develop .#agent --command scripts/scenarios.py
```

The receipt records the git revision and binary and chart digests. It also records each step's
actor, command id, observation and artifact, and labels each step `controlled`, `real-model` or
`clinical-judgment`. The runner produces only controlled steps. Each capability a journey needs
that the run did not exercise goes under `missing` with the reason and its owning source, so the
outcome reads `incomplete` and does not pass silently. A failed step, build or shutdown exits 1
and still writes the receipt. `--require-complete` exits 3 while anything is missing.

## PStack

`statechart`, `create-formal-verification-skill`, `maintain-formal-verification-skill`,
`create-verification-skill` and `maintain-verification-skill` in `.agents/skills` are
[PStack at 0b46ec3](https://github.com/cursor/plugins/tree/0b46ec399d55f2829e88822998f53b84944837d3/pstack/skills),
copied unchanged except that `statechart/scripts/test`, the tool's own test corpus, is left out.
Their [MIT license](PSTACK-LICENSE) comes with them. The
[daily check](../.github/workflows/pstack-watch.yml) pushes `codex/update-pstack-skills` and
opens or refreshes one review PR (or a tracking issue with a compare link when repository
settings disable Actions PR creation) when these five imported skills change.
[CI setup](../book/src/release.md#continuous-integration) covers publication.

Claude Code also gets the whole PStack plugin: [`.claude/settings.json`](../.claude/settings.json)
declares the `cursor-plugins` marketplace and enables `pstack@cursor-plugins`, so Claude Code
offers to install it when the repository is opened. Its skills appear as `pstack:<name>`, with
`pstack:statechart` the same code as the vendored `statechart`. Codex and CI use the vendored
copies above and need no plugin.

A verify skill the create skills write goes where [Verify skills](#verify-skills) says, not in
`.cursor/skills/`. It refers to `statechart` by name and copies none of its commands, options or
paths, which change between PStack releases.
