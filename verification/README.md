# Verification

The [intent specs](../intent/README.md) say what Coda does. This folder checks it, one feature at a
time: each feature gets a precise spec, a TLA+ model of that spec, and a way to hold the code to
the model. Features not modeled yet follow the intent specs alone.

## A feature's folder

`verification/<product>/<feature>/` holds:

- **`README.md`:** the feature's spec in plain words, with nothing about how it's checked. The
  intent spec keeps a short summary and links here. Its sections, in order:
  - **Intro:** what the spec covers, and the link to the intent spec.
  - **Words:** each word a rule or journey line depends on, defined once, so each rule has one
    meaning and maps to one thing in the model.
  - **Journey:** what happens, one claim per line, grouped by step. A promise that rests on
    stored records stays here as a plain promise; the verify skill names the tests that hold it.
  - **Rules:** numbered, one sentence each, with every condition the check uses written into
    the sentence.
  - **Outside this spec:** what other specs own, and open decisions with their
    [DECISIONS.md](../intent/DECISIONS.md) link.
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
`sign_in_model.rs`, and runs locally. CI doesn't run TLC. The `statechart` skill gives its own
command and setup; the feature's verify skill names the model, its charts and its browser test.

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
worker with its fixed reply, Postgres, Valkey and the Pub/Sub emulator. D1, a Duet conversation,
migrates a fresh database on the same Postgres through `duet-migrate` with a separate runtime
login, then drives the `duet-api` binary over HTTP: sign-up, a message, a kill and restart while
the reply is open, a retried send, and sign out everywhere. The binary runs with its local
stand-ins, `DUET_LOCAL_IDENTITY_SECRET` (HS256 tokens the runner signs) and
`DUET_LOCAL_MODEL_REPLY` (a fixed reply held open for `DUET_LOCAL_MODEL_DELAY_MS`); hosted
startup refuses all three. D1 then runs the duet-api Postgres suites. The run reads no credentials.

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
[PStack at 519d36a](https://github.com/cursor/plugins/tree/519d36aa98955bf680db01bcde5fdfba686dc901/pstack/skills),
copied unchanged except that `statechart/scripts/test`, the tool's own test corpus, is left out.
Their [MIT license](PSTACK-LICENSE) comes with them. The
[daily check](../.github/workflows/pstack-watch.yml) opens or refreshes one review PR when these
five imported skills change. [CI setup](../book/src/release.md#continuous-integration) covers PR publication.

A verify skill the create skills write goes where [Verify skills](#verify-skills) says, not in
`.cursor/skills/`. It refers to `statechart` by name and copies none of its commands, options or
paths, which change between PStack releases.
