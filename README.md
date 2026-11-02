# coda-rust

This is the monorepo for Coda's Rust workspace. It holds the shared Duet client, the physician Dioxus client, and the crates they use. The product repositories are checked out here as submodules. The [intent specs](intent/README.md) own product intent; Linear tracks scope, work and acceptance against them; this repository owns Rust implementation and proof. Start with the [documentation map](book/src/reading.md). Submodules are read-only historical references.

## Layout

- `crates/` is one Cargo workspace. The [crate map](book/src/crate-map.md) lists every crate, what it owns, and which crates both products share.
- `platform/` holds the iPhone and Android bindings for the shared Duet client.
- `skills/` is the clinical skill catalog, read by `agent-skills` for `harness`; hosted physician chat in `physician-worker` loads it at startup.
- `verification/` holds product features as TLA+ models, their design decisions and scoped runtime evidence. Start with [verification](verification/README.md) and [physician sign-in](verification/physician/sign-in/README.md). `.agents/skills/` holds the engineering workflows used to maintain them; it is separate from the clinical catalog.
- `crates/workspace-tools/` owns workspace commands, and `scripts/` holds supporting checks and generator assets. [API generation](book/src/backend-architecture.md#api-and-clients) covers the Duet wire generator.
- `book/` documents Rust owners, setup, operational procedures and proof boundaries. Build with `mdbook build book`, serve this repository with `python3 -m http.server` from `nix develop`, and open `/target/book/reading.html`. `mdbook serve` cannot see `/crates` or `/submodules`. `scripts/check-book-links.sh` builds the book and checks every link against this checkout, offline; CI runs it without submodules, so links into them are checked only in a checkout that has them.
- `submodules/` holds the pinned product repositories, read-only.

[AGENTS.md](AGENTS.md) holds the working rules, and the [documentation map](book/src/reading.md) says where each area is documented.

Agent VM and sandbox recipes live in [Coda Open-Inspect](https://github.com/Coda-Health/coda-open-inspect/tree/main/deploy).
This repository owns the Coda toolchain, dev-container setup and repository hook described below.

## Start

```sh
git clone --recurse-submodules git@github.com:Coda-Health/coda-rust.git
cd coda-rust
nix develop
cargo test --workspace
```

`nix develop` is the shell for this checkout. Tests and the sample server do not need a Doppler account.

`scripts/ui-gallery.sh` builds the UI gallery and serves it at <http://127.0.0.1:8080/>, each section at its own address, such as `/#charts`.

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

### Unattended development

`nix develop .#agent` uses the same locked toolchain as the default shell,
without Cursor or automatic Doppler credential discovery/loading. Use
`nix develop .#agent --command <command>` for builds, tests and Git commits;
the existing commit checks then run inside that environment.

[`.openinspect/setup.sh`](.openinspect/setup.sh) materializes this profile using
Nix supplied by the sandbox image. Machine installation, persistent command paths,
agent CLIs and sandbox lifecycle belong to
[Coda Open-Inspect](https://github.com/Coda-Health/coda-open-inspect/tree/main/packages/sandbox-images).
The hook prepares the store; subsequent commands still enter the Nix shell.

### Local CI checks

From the repository's agent dev shell, install the small validation dependency
into a virtual environment and run the same lightweight checks as CI:

```sh
python3 -m venv /tmp/coda-ci-venv
/tmp/coda-ci-venv/bin/pip install -r scripts/ci-requirements.txt
/tmp/coda-ci-venv/bin/python -m unittest discover -s scripts -p 'test_ci_*.py' -v
/tmp/coda-ci-venv/bin/python scripts/check_engineering_skills.py
/tmp/coda-ci-venv/bin/python -m unittest discover -s .agents/skills/review-pr/scripts -p 'test_*.py' -v
lychee --offline --no-progress --include-fragments --root-dir "$PWD" '.agents/skills/**/*.md'
scripts/check-book-links.sh
```

[ci_scope.py](scripts/ci_scope.py) owns the lightweight path categories. Other or
mixed changes keep the full suite; missing Git comparison data also selects it.
PRs use their complete diff from the merge base, and main pushes use the full
before/after range. Manual dispatch retains full browser qualification. New PR
revisions cancel superseded CI runs, without cancelling main deployment runs.

### Dev container

The same workspace runs in a VS Code Dev Container whose `dev` service executes
the identical `nix dev shell` (the flake stays the single toolchain owner) and
a `pgvector` PostgreSQL 16 service on `127.0.0.1:5433`. Prerequisites: Docker,
Compose, and the Dev Containers extension on x86_64 Linux. On Apple silicon, use
[Apple's container tool](#apple-container) instead.

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

### Apple container

On Apple silicon with macOS 26, [`container`](https://github.com/apple/container)
runs the same dev shell as native `aarch64-linux`. It has no Docker API, Compose
or host networking, so "Reopen in Container" and `act` do not work with it, and
Rosetta cannot host the x86_64 shell because it does not preserve `argv[0]`,
which Nix's Python wrappers rely on. CI remains the x86_64 proof.

```sh
brew install --formula container
dev/apple-container.sh
container exec -it --user vscode --workdir /workspace coda-dev-arm nix develop .#agent
```

The script reuses an existing container and takes `CODA_CONTAINER_NAME`,
`CODA_CONTAINER_CPUS` and `CODA_CONTAINER_MEMORY`. The Nix store, Cargo home and
`target` live in volumes named after the container. It publishes ports 8790 and
8080 on loopback, so stop one container before starting another under a new name.
Rerun the script after a restart rather than `container start` alone.

Run the rest inside that shell. Bind a server to `0.0.0.0` to reach it from the
Mac. The physician page needs its bundle first:

```sh
cargo build -p physician-app --target wasm32-unknown-unknown --features web
wasm-bindgen --target web --out-dir target/physician-web "$PWD/target/wasm32-unknown-unknown/debug/physician-app.wasm"
PHYSICIAN_LISTEN_ADDR=0.0.0.0:8790 cargo run -p physician-sample
```

The UI gallery builds and serves with one command, here on the published port:

```sh
UI_GALLERY_LISTEN_ADDR=0.0.0.0:8080 scripts/ui-gallery.sh
```

Live chat on the physician page needs the Codex CLI and `codex login` in the container, or
a [replay cassette](#physician-chat-codex-cli). The dev shell does not include it.

The script starts no Postgres service, and the database-backed tests fail without
one. Start a disposable cluster, point the tests at it as
[CI](.github/workflows/ci.yml) does, and clear the encoder model path unless that
directory exists, because the dev shell always sets it. The browser tests build
offline, so fetch their crates first:

```sh
initdb -D /tmp/coda-pg -U postgres --auth=trust
pg_ctl -D /tmp/coda-pg -o "-p 5544 -k /tmp" -l /tmp/coda-pg.log start
export DUET_TEST_DATABASE_URL=postgresql://postgres@127.0.0.1:5544/postgres
export PHYSICIAN_TEST_DATABASE_URL=$DUET_TEST_DATABASE_URL
export FRONT_DOOR_TEST_DATABASE_URL=$DUET_TEST_DATABASE_URL
unset DUET_DISCOVERY_ENCODER_MODEL CODA_PATIENT_ENCODER_MODEL
cargo test --workspace
cargo fetch --locked --target wasm32-unknown-unknown
cargo test -p physician-sample --test site --test auth_browser -- --ignored
```

`duet-api` also needs the [schema setup](book/src/backend-architecture.md#persistence-and-migrations)
and its Doppler secrets; bind it with `DUET_LISTEN_ADDR=0.0.0.0:8080`.

Commit inside this shell, after setting your Git identity there. Entering it sets
`core.hooksPath` in the shared `.git/config`, and that hook cannot run on a Mac
without Nix. Push from the Mac, which holds your credentials.
