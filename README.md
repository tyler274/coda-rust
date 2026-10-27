# coda-rust

This is the monorepo for Coda's Rust workspace. It holds the shared Duet client, the physician Dioxus client, and the crates they use. The product repositories are checked out here as submodules. Linear owns product intent and acceptance; this repository owns Rust implementation and proof. Start with the [documentation map](book/src/reading.md). Submodules are read-only historical references. React and SwiftUI remain the shipping clients until the Rust journeys are qualified.

## Layout

- `crates/` is one Cargo workspace. Duet is `duet-app` and `duet-api`. The physician page is `physician-app`. `physician-sample` serves that page and a recorded projection of the synthetic practice. `local-product` is the model-execution owner. Unset `CODA_LOCAL_MODEL_EXECUTION` is `live`: physician chat uses the Codex CLI subscription. `record` and `replay` use a cassette and do not read an API key. `api` answers physician chat through OpenAI Responses when `CODA_ALLOW_MODEL_CALLS=1`. `ui`, `http`, and the product-neutral `harness` and its neutral supporting libraries are shared; each product keeps its own authority, state and recovery.
- `platform/` holds the iPhone and Android bindings for the shared Duet client.
- `skills/` is the clinical skill catalog, read by `agent-skills` for `harness`; no product runtime loads it yet.
- `verification/` holds product statecharts, their design decisions, Verus models and scoped runtime evidence. Start with [verification](verification/README.md) and the [physician experience](verification/physician-experience/README.md). `.agents/skills/` holds the engineering workflows used to maintain them; it is separate from the clinical catalog.
- `crates/workspace-tools/` owns workspace commands. `scripts/` holds supporting checks and adapters; `scripts/duet-wire/` groups the OpenAPI composer, finalizer and templates behind `cargo run -p workspace-tools -- generate-duet-wire`. See [API generation](book/src/backend-architecture.md#api-and-clients) for inputs and drift checks.
- `book/` documents Rust owners, setup, operational procedures and proof boundaries. Build with `mdbook build book`, serve this repository with `python3 -m http.server` from `nix develop`, and open `/target/book/reading.html`. `mdbook serve` cannot see `/crates` or `/submodules`. `scripts/check-book-links.sh` builds the book and checks every link against this checkout, offline; CI runs it without submodules, so links into them are checked only in a checkout that has them.
- `submodules/` checks out the product repositories. Use their pinned READMEs as reference for the original implementation: [coda-app](submodules/coda-app/README.md), [coda-agents](submodules/coda-agents/README.md), [coda-gym](submodules/coda-gym/README.md), and [Coda-Computer-Use](submodules/Coda-Computer-Use/README.md).

Where a change belongs, and how the dev shell, Doppler, and CI fit, is [AGENTS.md](AGENTS.md).

Agent VM and sandbox recipes live in [Coda Open-Inspect](https://github.com/Coda-Health/coda-open-inspect/tree/main/deploy).
This repository continues to own its Nix toolchain and dev-container setup.

## Start

```sh
git clone --recurse-submodules git@github.com:Coda-Health/coda-rust.git
cd coda-rust
nix develop
cargo test --workspace
```

`nix develop` is the shell for this checkout. Tests and the sample server do not need a Doppler account.

### Physician chat (Codex CLI)

Local physician chat uses the Codex CLI subscription. Projections stay on the recording. Live, record, and replay do not read `OPENAI_API_KEY`.

Sign in once, outside this repo's nix shell if that is where the CLI is installed:

```sh
codex login
```

Then start the sample server. Unset `CODA_LOCAL_MODEL_EXECUTION` is `live`.

```sh
cargo run -p physician-sample
```

Record a cassette outside the repository, then replay it. Replay makes no model call.

```sh
CODA_LOCAL_MODEL_EXECUTION=record \
CODA_CASSETTE_AUTHORIZATION='who approved this synthetic run' \
CODA_CASSETTE_OUTPUT=/absolute/path/cassette.json \
cargo run -p physician-sample

CODA_LOCAL_MODEL_EXECUTION=replay \
CODA_CASSETTE_PATH=/absolute/path/cassette.json \
cargo run -p physician-sample
```

`CODA_LOCAL_MODEL_EXECUTION=api` still calls OpenAI Responses and requires `CODA_ALLOW_MODEL_CALLS=1` and `OPENAI_API_KEY`.

A live chat after `codex login` is a check for the person who holds the subscription. Hold one physician chat on the sample server and confirm the reply lands in the thread. Cassette replay is the check that does not need that login.

### Dev container

The same workspace runs in a VS Code Dev Container whose `dev` service executes
the identical `nix dev shell` (the flake stays the single toolchain owner) and
a `pgvector` PostgreSQL 16 service on `127.0.0.1:5433`. Prerequisites: Docker,
Compose, and the Dev Containers extension (x86_64 Linux; the flake hardcodes
that system).

1. Open the repository in VS Code and choose "Reopen in Container".
2. The first build installs pinned Nix into a `/nix` volume, fetches missing
   submodules and materializes the dev shell. Database initialization is explicit:
   provision a distinct unprivileged runtime login, then run the Rust-owned
   `duet-migrate` with `DUET_MIGRATION_DATABASE_URL` and `DUET_RUNTIME_ROLE`
   as described in [backend migrations](book/src/backend-architecture.md#persistence-and-migrations).
   Set the API `DATABASE_URL` to that runtime login before starting it. The
   container-provided `coda` database user is the local bootstrap administrator.
3. Run from the command palette, Terminal > Run Task: `physician page (recorded)`
   (http://127.0.0.1:8790), `duet-api` (http://127.0.0.1:8080), and the optional
   `React local product` (http://127.0.0.1:8787) after its one-time host-side
   setup (see [coda-agents README](submodules/coda-agents/README.md)). Build
   the page bundle first with `physician page web bundle`.

Storage: the database lives in the `coda-rust-pg` named volume; it survives
rebuilds and resets only with `docker compose down -v` from `.devcontainer/`.
`DATABASE_URL` is provided by the container. Test clusters, vitest, and the
Postgres-backed suites behave identically inside the container because the
flake's socket-directory wrapper engages there too. Live model execution and
the encoder need host-only local data; uncomment the `coda-local-data` bind
mount in [.devcontainer/docker-compose.yml](.devcontainer/docker-compose.yml).
