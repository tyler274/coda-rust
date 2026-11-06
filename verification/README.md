# Verification

The [intent specs](../intent/README.md) say what Coda does. This folder checks it, one feature at a
time: each feature gets a precise spec, a TLA+ model of that spec, and a way to hold the code to
the model. Features not modeled yet follow the intent specs alone.

| Feature | Folder |
| --- | --- |
| Physician sign-in | [physician/sign-in](physician/sign-in/README.md) |

## A feature's folder

`verification/<product>/<feature>/` holds:

- **`README.md`:** the feature's spec in words, with its agreed rules. The intent spec keeps a
  short summary and links here. A promise the model can't hold, such as identity, revisions,
  collections or single use, is listed with the test that holds it, or says none does yet.
- **`<Feature>.tla` and `.cfg`:** the model, written from the README, not from the code. Each
  `INVARIANT` is one rule, quoted in its comment. Each action's comment names the code that
  carries it out. An `event` variable names each step, and the config's `VIEW` leaves it out.

## How a feature is checked

1. **Agree first.** Agree the feature's flows and rules before modeling them, and keep what the
   code does today separate from what was agreed.
2. **Check the model.** PStack's `statechart` skill runs TLC, which visits every state the model
   can reach and checks every rule. A deliberately broken copy of the model shows each rule can
   fail.
3. **Hold the code to it.** The code follows the model. When TLC or review finds a path on which
   the code breaks a rule, that path goes into the feature's browser test. There,
   [replay.mjs](replay.mjs) gives the model's state after each step, and the test plays the path
   in the real page and compares. The test must fail on the code before its fix.
4. **Settle differences.** A difference the model can't settle is a product decision, not a code
   fix.

Each feature's browser test lives in its own file named for the feature, such as
`sign_in_model.rs`, and runs locally. CI doesn't run TLC.

```sh
.agents/skills/statechart/scripts/statechart verification/physician/sign-in/SignIn.tla
```

The first run installs PStack's pinned TLC and Java with mise. The feature's README names its
browser test.

## Verify skills

A verify skill names the commands, expected results and evidence that hold one feature to
its spec, and the code whose changes call for it. When a change touches that code, its
acceptance evidence comes from the skill. A change outside every skill's code proves itself
with focused tests and the observations its criteria name.

| Skill | Proves | Run when a change touches |
| --- | --- | --- |
| [coda-verify-sign-in](../.agents/skills/coda-verify-sign-in/SKILL.md#when-a-change-needs-this) | Physician sign-in against its TLA+ model: TLC on the five rules, then the bug paths replayed in the browser | Physician session, sign-in and tab code, the auth server's answers, or the sign-in spec or model |
| [coda-verify-physician](../.agents/skills/coda-verify-physician/SKILL.md#when-a-change-needs-this) | Physician chat across two site instances and a worker: fan-out, couldn't-finish and Try again, resync, duplicate and retry delivery | Physician chat delivery, the worker, the event bus, durable chat runs, or the conversation renderer |

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

Each skill writes a project skill, such as
[coda-verify-sign-in](../.agents/skills/coda-verify-sign-in/SKILL.md), that names Coda's commands
and what each check covers. The PStack skills say to write it under `.cursor/skills/verify-<app>/`;
in Coda it goes in `.agents/skills/coda-verify-<feature>/` with a `.claude/skills` link, like every
repository skill, and gets a row under [Verify skills](#verify-skills). The maintain skills look
for it there.

The skills in `.agents/skills` are [PStack at 519d36a](https://github.com/cursor/plugins/tree/519d36aa98955bf680db01bcde5fdfba686dc901/pstack/skills),
copied unchanged except that `statechart/scripts/test`, the tool's own test corpus, is left out.
The [MIT license](PSTACK-LICENSE) retains Lauren Tan's attribution. The
[daily check](../.github/workflows/pstack-watch.yml) opens or refreshes one review PR when these
five imported skills change. [CI setup](../book/src/release.md#continuous-integration) covers PR publication.
