# Coda Rust operating contract

This repository is the Rust workspace for Duet and the physician client. Product doctrine
stays in the product repositories. This file routes work to those contracts. It does not
copy them.

## Where work goes

| Work | Contract |
| --- | --- |
| Rust crates, the shared UI tokens, the physician Dioxus client, the sample practice server, and the skill loader | This file, then the crate you are changing |
| Physician product, clinical runtime, and the clinical skill catalog | [coda-agents AGENTS.md](submodules/coda-agents/AGENTS.md) |
| Clinical skill markdown | [submodules/coda-agents/skills](submodules/coda-agents/skills), loaded by `crates/agent-skills` |
| Clinical-corpus operations | [coda-agents skill](submodules/coda-agents/.agents/skills/coda-gcp-clinical-corpus-operations/SKILL.md) |
| Duet, the patient app | [coda-app AGENTS.md](submodules/coda-app/AGENTS.md) and [Duet UI](submodules/coda-app/.agents/skills/duet-ui-ux/SKILL.md) |
| Gym cases, promotion, and runs | [coda-gym AGENTS.md](submodules/coda-gym/AGENTS.md) and the skills under [coda-gym/.claude/skills](submodules/coda-gym/.claude/skills) |
| Computer use | [Coda-Computer-Use AGENTS.md](submodules/Coda-Computer-Use/AGENTS.md) |

`eval/` inside coda-agents is sealed. Do not read it or place it in prompts, fixtures, or model-visible context.

Duet and the physician product do not share clinical types, accounts, or servers. Shared crates are `ui` and `http` only.

The physician site is a client of the physician projections. `crates/physician-sample` serves a recorded projection of the synthetic practice. It does not run a model and it is not a second clinical engine. `PHYSICIAN_API_URL` points the same client at the Node server.

React and SwiftUI remain the shipping clients until a journey matches them. Do not claim a production cutover, a TestFlight release, or a physician production deploy from this repository.
