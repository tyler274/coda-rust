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

| Skill | Proves | Run when a change touches |
| --- | --- | --- |
| [coda-verify-sign-in](../.agents/skills/coda-verify-sign-in/SKILL.md#when-a-change-needs-this) | Physician sign-in against its TLA+ model: TLC on its rules, then the bug paths replayed in the browser | Physician session, sign-in and tab code, the auth server's answers, or the sign-in spec or model |
| [coda-verify-physician](../.agents/skills/coda-verify-physician/SKILL.md#when-a-change-needs-this) | Physician chat across two site instances and a worker: fan-out, couldn't-finish and Try again, resync, duplicate and retry delivery | Physician chat delivery, the worker, the event bus, durable chat runs, or the conversation renderer |

## The agent harness

The agent harness stays out of the models and meets them only at declared events. An agent's
event is the harness's output, and each tool it admits is one such event: the product host checks
that event's `needs` before the effect commits, and the model decides when to use it. Retries,
timeouts, cadence, memory and prompts stay in the harness, whose guarantees the
[harness spec](../intent/harness.md) states. An app's own session and request handling is not the
harness: when a rule depends on it, it belongs in the model.

## PStack

Coda uses PStack's five verification skills as they are:

- `statechart` checks a feature's model.
- `create-formal-verification-skill` and `maintain-formal-verification-skill` connect the model to
  the code and keep them in step.
- `create-verification-skill` and `maintain-verification-skill` do the same for driving a running
  app.

Each create skill writes a project skill, such as
[coda-verify-sign-in](../.agents/skills/coda-verify-sign-in/SKILL.md), that names Coda's commands
and what each check covers. `create-verification-skill` says to write it under
`.cursor/skills/verify-<app>/`; in Coda it goes in `.agents/skills/coda-verify-<feature>/` with a
`.claude/skills` link, like every repository skill, and gets a row under
[Verify skills](#verify-skills). The maintain skills find a verify skill by what it does, not by
its path. A project skill refers to `statechart` by name and copies none of its commands, options
or paths, which change between PStack releases.

The skills in `.agents/skills` are [PStack at 519d36a](https://github.com/cursor/plugins/tree/519d36aa98955bf680db01bcde5fdfba686dc901/pstack/skills),
copied unchanged except that `statechart/scripts/test`, the tool's own test corpus, is left out.
The [MIT license](PSTACK-LICENSE) retains Lauren Tan's attribution. The
[daily check](../.github/workflows/pstack-watch.yml) opens or refreshes one review PR when these
five imported skills change. [CI setup](../book/src/release.md#continuous-integration) covers PR publication.
