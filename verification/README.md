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
and what each check covers.

The skills in `.agents/skills` are [PStack at 519d36a](https://github.com/cursor/plugins/tree/519d36aa98955bf680db01bcde5fdfba686dc901/pstack/skills),
copied unchanged except that `statechart/scripts/test`, the tool's own test corpus, is left out.
The [MIT license](PSTACK-LICENSE) retains Lauren Tan's attribution. To move to a newer PStack,
replace the folders from that commit and update the commit here.
