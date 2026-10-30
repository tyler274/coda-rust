# Coda product verification

> **Journey models are built one at a time from the [intent specs](../intent/README.md)**, under
> `physician/`, `duet/` and `connected-care/`, starting with
> [physician sign-in](physician/sign-in/README.md). The specs govern every journey not yet modeled.

Each journey's spec is markdown; its model is a TLA+ spec that TLC checks; the code follows the
model, and a browser test holds it there. Coda uses [PStack](#pstack)'s verification skills as they
are: `statechart` checks a journey's model, `create-formal-verification-skill` and
`maintain-formal-verification-skill` connect it to the code and keep it in step,
`create-verification-skill` and `maintain-verification-skill` do the same for driving a running app.
Each skill writes a project skill, such as
[coda-verify-physician](../.agents/skills/coda-verify-physician/SKILL.md), which names Coda's
commands and what each check covers.

## A journey's folder

| File | Holds |
| --- | --- |
| `README.md` | The spec in words: how the journey works and the rules agreed with Dev. A promise that rests on records the model can't hold (identity, revisions, collections, single use) is listed under the rules with the test that holds it, or says none does yet. |
| `<Journey>.tla`, `.cfg` | The model, written from the README, not from the code. Each `INVARIANT` is one agreed sentence, with the sentence as its comment. Each action's comment says where the code carries it out. An `event` variable names each step, and the config's `VIEW` leaves it out of the state. |

The code follows the model. Where it differs, write a test that fails on the code, then change the
code; a difference the model can't settle is a design question for Dev. Discuss intended flows
with Dev before modeling them, and keep observed implementation separate.

The agent harness stays out of the models and meets them only at declared events. An agent's
event is the harness's output, and each tool it admits is one such event: the product host checks
that event's `needs` before the effect commits, and the model decides when to use it. Retries,
timeouts, cadence, memory and prompts stay in the harness, whose guarantees the
[harness spec](../intent/harness.md) states. An app's own session and request handling is not the
harness: when a rule depends on it, it belongs in the model.

## Holding the code to the model

TLC checks the model; it doesn't reach the code. When TLC or review finds a path on which the code
breaks a rule, that path is replayed against the real code, as PStack's `statechart` skill directs.
[replay.mjs](replay.mjs) asks TLC for the model's state after each step of each path, and the
journey's browser test does what each event names in the real page and, after every step, compares
the page with the model. Each bug path stays in that test, which failed on the code before its fix.
The test lives in its own file, named for the journey such as `sign_in_model.rs`, and runs locally:
CI doesn't run TLC.

## Commands

From the repository root, for a journey's model:

```sh
.agents/skills/statechart/scripts/statechart verification/physician/sign-in/SignIn.tla
```

The first run installs PStack's pinned TLC and Java with mise. The journey's README names its
browser test.

## Feature specs

| Folder | Spec |
| --- | --- |
| [physician/sign-in](physician/sign-in/README.md) | How a physician signs in: the spec in words, its rules, and its TLA+ model. |

## PStack

The five skills in `.agents/skills` are [PStack at 519d36a](https://github.com/cursor/plugins/tree/519d36aa98955bf680db01bcde5fdfba686dc901/pstack/skills),
copied unchanged, except that `statechart/scripts/test`, the tool's own test corpus, is left
out. The [MIT license](PSTACK-LICENSE) retains Lauren Tan's attribution. To move to a newer
PStack, replace the folders from that commit and update the commit here.
