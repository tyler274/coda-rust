# Coda Rust operating contract

This repository is the Rust workspace for Duet and the physician client. Product doctrine
stays in the product repositories. This file routes work to those contracts. It does not
copy them.

## Where work goes

| Work | Contract |
| --- | --- |
| Rust crates, the shared UI tokens, the physician Dioxus client, the sample practice server, and the skill loader | This file, then the crate you are changing |
| Public marketing site, its Rust server, and prepared deployment artifacts | [Public site](book/src/public-site.md), then `crates/public-site`; `submodules/kytos` is optional read-only developer reference; builds/tests/CI use committed source snapshots, not private-repo access; no DNS or Vercel cutover |
| Physician product, clinical runtime, and the clinical skill catalog | [Hoisted Coda Agents contract](book/src/agent-contracts.md) and [coda-agents AGENTS.md](submodules/coda-agents/AGENTS.md) |
| Clinical skill markdown | [skills](skills), loaded once by `crates/harness` through `crates/agent-skills` |
| Clinical-corpus operations | [coda-agents skill](submodules/coda-agents/.agents/skills/coda-gcp-clinical-corpus-operations/SKILL.md) |
| Duet, the patient app | [Hoisted Duet contract](book/src/agent-contracts.md#duet-patient-app), [coda-app AGENTS.md](submodules/coda-app/AGENTS.md) and [Duet UI](submodules/coda-app/.agents/skills/duet-ui-ux/SKILL.md) |
| Gym cases, promotion, and runs | [Hoisted Gym contract](book/src/agent-contracts.md#coda-gym-evaluation), [coda-gym AGENTS.md](submodules/coda-gym/AGENTS.md) and the skills under [coda-gym/.claude/skills](submodules/coda-gym/.claude/skills) |
| Computer use | [Hoisted computer-use contract](book/src/agent-contracts.md#computer-use) and [Coda-Computer-Use AGENTS.md](submodules/Coda-Computer-Use/AGENTS.md) |
| Dev container setup, ports, and storage | [Dev container](README.md#dev-container) |

The operative rules of all four submodule contracts are hoisted into
[Agent contracts, hoisted](book/src/agent-contracts.md) so work in this
workspace can follow them without opening a submodule. Submodules stay
read-only reference; their sources win over the hoisted page when they
disagree.

`eval/` inside coda-agents is sealed. Do not read it or place it in prompts, fixtures, or model-visible context.

Duet and the physician product do not share clinical types, accounts, or servers. Shared crates are `ui` and `http` only.

The physician site is a client of the physician projections. `crates/physician-sample` serves the page and a recorded projection of the synthetic practice on one origin. Unset `CODA_LOCAL_MODEL_EXECUTION` is `live`: physician chat runs through the Codex CLI subscription (`codex login`) and does not read an API key. `record` writes a cassette and `replay` answers from that cassette, also without a key. `api` answers physician chat through `crates/local-product` and OpenAI Responses; the key comes from the environment (`CODA_ALLOW_MODEL_CALLS=1` and `OPENAI_API_KEY`, loaded by Doppler in the nix shell) and is not stored in the page. The page mounts into `#main`. On the web build, an unset `PHYSICIAN_API_URL` uses that origin; `?api=` still points the same build at the Node server. A physician-site change is not done unless `cargo test -p physician-sample --test site` passed. That Chromium test fails when the document is blank.

The live Node projection server moves into `physician-sample` route family by route family. Duet stays in `duet-api` and `duet-app`. Gym and computer use get their own crates after those, and neither imports the other's clinical types.

## Dev shell

`nix develop` in this repository is the shell for the monorepo. It carries the Rust toolchain, the product tools, Doppler, and `act`.

Doppler project `coda-rust` has three configs, `dev`, `stg` and `prd`. Log in once with `doppler login`, then prefix a local command with `doppler run --`. The shell loads `dev` when a login exists and continues when it does not. Production Rust processes start with `doppler run --project coda-rust --config prd --` and still read environment variables. Tests and the sample server do not need a Doppler account. Creating `prd` is an operator login step; this repo stores no token and no secret value.

Doppler holds every secret value. GCP Secret Manager holds only `DOPPLER_TOKEN`, a read-only service token scoped to one config: Cloud Run mounts it, and the process starts under `doppler run --`, which loads the rest. GitHub release jobs read the same token from an environment secret. Do not copy Doppler values into Secret Manager. `scripts/gcp/install_doppler.sh` installs the pinned, checksum-verified CLI for images and runners.

`infra/gcp` is the monorepo copy of the physician OpenTofu roots (`foundation`, `staging`, `clinical-evidence`). Apply authority stays in the product repositories. This workspace does not init a backend or apply those roots.

`act` runs [`.github/workflows/ci.yml`](.github/workflows/ci.yml) with the host Docker daemon. `actionlint` checks that file. The workflow does not call Doppler.

CI runs on Blacksmith runners (`blacksmith-2vcpu-ubuntu-2404`), which boot GitHub's Ubuntu 24.04 image and serve `actions/cache` from Blacksmith's own cache. `.actrc` maps that label to the `act` image, and [`.github/actionlint.yaml`](.github/actionlint.yaml) declares it. To roll back, set `runs-on` to `ubuntu-latest`.

React and SwiftUI remain the shipping clients until a journey matches them. Do not claim a production cutover, a TestFlight release, or a physician production deploy from this repository.
