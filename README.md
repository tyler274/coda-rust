# coda-rust

This is the monorepo for Coda's Rust workspace. It holds the shared Duet client, the physician Dioxus client, and the crates they use. The product repositories are checked out here as submodules. The [intent specs](intent/README.md) own product intent; Linear tracks scope, work and acceptance against them; this repository owns Rust implementation and proof. Start with the [documentation map](book/src/reading.md). Submodules are read-only historical references.

## Layout

- `crates/` is one Cargo workspace. The [crate map](book/src/crate-map.md) lists every crate, what it owns, and which crates both products share.
- `platform/` holds the iPhone and Android bindings for the shared Duet client.
- [`skills/`](skills) holds the physician skill package. The site saves it with each accepted chat so workers can open the same guidance after a restart. [Physician chat skills](book/src/physician-site.md#skills-in-physician-chat) describes the runtime path.
- `verification/` holds each feature's spec, TLA+ model and charts. Start with [verification](verification/README.md). `.agents/skills/` holds the engineering workflows used to maintain them; it is separate from the clinical catalog.
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

`nix develop` is the shell for this checkout. Tests and the sample server do not need a Doppler account. On an Apple silicon Mac without Nix, use the [Apple container](#apple-silicon-mac-without-nix-apple-container) instead.

`scripts/ui-gallery.sh` builds the UI gallery and serves it at <http://127.0.0.1:8080/>, each section at its own address, such as `/#charts`.

### Coding agent connections

Open the checkout in Codex and trust the project to load [its MCP config](.codex/config.toml).
In Claude Code, approve the [project servers](.mcp.json) when prompted.
In Cursor, approve the [project servers](.cursor/mcp.json) in Customize, once in each checkout or worktree.
Launch the client with any required token variables in its environment. Each developer uses their own credentials.
Cursor starts its local servers with the `PATH` from the environment it was launched in. That `PATH` needs `uvx` for NixOS, and `gh` plus `nix` or `github-mcp-server` for GitHub. If a server can't find one, quit Cursor fully, then start it from a shell that has them.

| Server | Setup |
| --- | --- |
| [GitHub](https://github.com/github/github-mcp-server#remote-github-mcp-server) | For Codex and Claude Code, set `GITHUB_MCP_TOKEN` to a personal access token scoped to the repositories and operations you need. Cursor runs the [local server](https://github.com/github/github-mcp-server#local-github-mcp-server) through [`scripts/github-mcp.sh`](scripts/github-mcp.sh). It uses `GITHUB_MCP_TOKEN` when that is set, and otherwise your `gh auth login` with all of that login's scopes. With Nix it runs the server from the flake's locked nixpkgs. On the [Apple container](#apple-silicon-mac-without-nix-apple-container) route, `brew bundle` installs `gh` and the current Homebrew `github-mcp-server`, which is not pinned and can offer different tools. |
| [Linear](https://linear.app/docs/mcp) | From this checkout, run `codex mcp login linear`, open `/mcp` in Claude Code, or connect Linear in Cursor's Customize. |
| [NixOS](https://github.com/utensils/mcp-nixos) | Put `uvx` on the client's PATH. The dev shell includes it through `uv`. On the [Apple container](#apple-silicon-mac-without-nix-apple-container) route, the client runs on the Mac, and `brew bundle` installs `uv` there. On first use it downloads the pinned server for package and option lookups. |
| [Slack](https://docs.slack.dev/ai/slack-mcp-server/) | In Claude Code, open `/mcp` and sign in with the [Slack integration for Claude Code](https://docs.slack.dev/ai/slack-mcp-server/connect-to-claude/) approved by your workspace admin. In Cursor, connect Slack in Customize; the config names [Slack's app for Cursor](https://docs.slack.dev/ai/slack-mcp-server/connect-to-cursor/), which your workspace admin approves. For Codex, set `SLACK_MCP_USER_TOKEN` to a user token from a workspace-approved, MCP-enabled Slack app with the scopes you need. |
| [Datadog](https://docs.datadoghq.com/mcp_server/setup/) | Sign in to the US1 site with `codex mcp login datadog`, through `/mcp` in Claude Code, or in Cursor's Customize. Use a role with MCP Read and read access to the logs, monitors and traces you investigate. |
| [Local Langfuse](dev/observability/README.md) | Run `scripts/observability-up.sh` on the client host. It needs Docker Compose there, which Apple's `container` tool does not provide. All three clients use port 3000 and the local Compose project's fixed synthetic-data credentials. If you override `LANGFUSE_PORT`, set that port in a personal client entry rather than the committed config, and keep the hostname `localhost`. In Cursor, add the entry under another name in `~/.cursor/mcp.json` and turn off the project `langfuse` server in Customize. The v3 stack exposes project tools such as `getHealth` and `listPrompts` through MCP. Use the local UI for generation inputs and outputs. |

Open `/mcp` in Codex or Claude Code, or Customize in Cursor, to inspect the servers. To check access, ask for a known pull request, Linear issue, Nix package, or Slack thread you can read.

### Local practice

`scripts/local-practice.sh` runs the synthetic practice on a local Postgres database. It runs the migration and the chart load, then starts the site with sign-in required. Sign in as `physician@codahealth.test`; the local site redeems the link itself.

```sh
PHYSICIAN_DATABASE_URL=postgres://user@127.0.0.1:5432/physician scripts/local-practice.sh
```

`cargo run -p physician-sample` alone has no database, so its practice has no patients.

### Local physician chat

For a synthetic Chords chat on your own ChatGPT plan, authorize this local installation in the dev shell:

```sh
cargo run -p local-product --bin chords-login
```

Open the printed **Continue with ChatGPT** link in a browser on the same machine. If the shell is in a dev container, forward the loopback port in the printed redirect URI to the same port on your machine. Run the sign-in command and the local server with the same config directory. After the terminal confirms authorization, select subscription mode explicitly:

```sh
CODA_LOCAL_MODEL_EXECUTION=subscription scripts/local-practice.sh
```

Chords sends its physician instructions and approved skill catalog to the Responses API, runs requested `skill_view` reads locally, and returns their results to the model. Every enabled model role requires explicit model, effort, working-window and input-limit settings; see [model settings](book/src/physician-site.md#model-settings). The model must be available to your account. The command saves the developer grant in `$XDG_CONFIG_HOME/coda/chords-chatgpt` (by default `~/.config/coda/chords-chatgpt`). Chords renews it locally. Disconnect Chords in ChatGPT Settings to revoke access. Use synthetic questions on this path. The local stack starts Postgres, Valkey, Pub/Sub, the site and the worker, and keeps the practice database under `target/local-practice` (override with `CODA_LOCAL_PRACTICE_DIR`). It selects the role settings in `scripts/physician-scale-stack.py`.

`local-practice.sh` defaults to subscription mode. The qualification stack can use its scripted provider with `scripts/physician-scale-stack.py --model-execution stub`. API-key execution requires `CODA_LOCAL_MODEL_EXECUTION=api`, `CODA_ALLOW_MODEL_CALLS=1` and `OPENAI_API_KEY`.

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

### Agent sandbox

`coda-sandbox`, built from its own flake under [`crates/workspace-tools/sandbox/`](crates/workspace-tools/sandbox/flake.nix),
runs the agent sandbox: `worktree` makes worktrees under `.worktrees/<slug>`
whose `target/` is a reflink clone of a shared seed of third-party build output,
`ci run` queues a CI job of a committed revision on the self-hosted Firecracker
runner, and `carry` bundles the host's worktrees for the sandbox. It reads the
machine's paths from `/etc/coda-sandbox/config.toml`, or from `--config`. Where
it is not installed yet, run it from the checkout through Nix:

```sh
nix run ./crates/workspace-tools/sandbox -- worktree new <slug>
nix run ./crates/workspace-tools/sandbox -- --help
```

`coda-sandbox worktree seed` builds the seed with
[`dev/session-seed.sh`](dev/session-seed.sh).

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

The runtime crash-restart check uses a disposable PostgreSQL cluster in CI shard 1. Run it locally
as an unprivileged user inside the agent dev shell, with port 55489 free:

```sh
coda_restart_test=$(cargo test --locked -p local-product --test runtime_postgres \
  --no-run --message-format=json-render-diagnostics | python3 -c '
import json, sys
artifacts = [json.loads(line) for line in sys.stdin]
[binary] = [a["executable"] for a in artifacts
            if a.get("executable") and a["target"]["name"] == "runtime_postgres"]
print(binary)') &&
python3 scripts/check-crash-restart.py "$coda_restart_test"
```

The wrapper initializes a temporary cluster, runs the ignored
`database_crash_restart_retains_acknowledged_command_and_event` test, and stops
and removes that cluster. It does not use the development database. A container
running as root must invoke the wrapper as an unprivileged user with the dev
shell's tools on `PATH` and read access to the compiled executable.

### Dev container

The same workspace runs in a VS Code Dev Container whose `dev` service executes
the identical `nix dev shell` (the flake stays the single toolchain owner) and
a `pgvector` PostgreSQL 16 service on `127.0.0.1:5433`. Prerequisites: Docker,
Compose, and the Dev Containers extension on x86_64 Linux. On Apple silicon, use
[Apple's container tool](#apple-silicon-mac-without-nix-apple-container) instead.

1. Open the repository in VS Code and choose "Reopen in Container".
2. The first build installs pinned Nix into a `/nix` volume, fetches missing
   submodules and materializes the dev shell. Database initialization is explicit:
   provision a distinct unprivileged runtime login, then run the Rust-owned
   `duet-migrate` with `DUET_MIGRATION_DATABASE_URL` and `DUET_RUNTIME_ROLE`
   as described in [backend migrations](book/src/backend-architecture.md#persistence-and-migrations).
   Set the API `DATABASE_URL` to that runtime login before starting it. The
   container-provided `coda` database user is the local bootstrap administrator.
3. Run from the command palette, Terminal > Run Task: `local practice`
   (http://127.0.0.1:8790), `duet-api` (http://127.0.0.1:8080), and the optional
   `React local product` (http://127.0.0.1:8787) after its one-time host-side
   setup (see [coda-agents README](submodules/coda-agents/README.md)).

Storage: the database lives in the `coda-rust-pg` named volume; it survives
rebuilds and resets only with `docker compose down -v` from `.devcontainer/`.
`DATABASE_URL` is provided by the container. Test clusters, vitest, and the
Postgres-backed suites behave identically inside the container because the
flake's socket-directory wrapper engages there too. Live model execution and
the encoder need host-only local data; uncomment the `coda-local-data` bind
mount in [.devcontainer/docker-compose.yml](.devcontainer/docker-compose.yml).

### Apple silicon Mac without Nix (Apple container)

On Apple silicon with macOS 26, [`container`](https://github.com/apple/container)
runs the same dev shell as native `aarch64-linux`. It has no Docker API, Compose
or host networking, so "Reopen in Container" and `act` do not work with it, and
Rosetta cannot host the x86_64 shell because it does not preserve `argv[0]`,
which Nix's Python wrappers rely on. CI remains the x86_64 proof.

Each checkout or worktree gets its own container, called a session, so several
agents can build and run the app side by side. Set up once, then use a session
per worktree:

```sh
nix profile add --inputs-from . nixpkgs#container   # or: brew bundle
dev/apple-session.sh seed
dev/apple-session.sh up
dev/apple-session.sh exec cargo test -p ui
dev/apple-session.sh down
```

`seed` builds the shared starting point: the Nix store, the Cargo home and the
third-party build output. It takes about 20 minutes and rerunning it picks up new
dependencies; a session also fetches and builds whatever the seed lacks, so
rerun it when sessions start spending their time there. A seed interrupted
partway resumes from its build output with `CODA_SESSION_KEEP_TARGET=1`. `up` takes seconds, because
it makes APFS clones of those volumes, which share disk with the seed until they
diverge. A session's own builds take 20 to 30 GB at their peak, and `down` returns
them. `prune` removes the sessions of worktrees that no longer exist, and `status`
lists them all.

The session mounts the worktree and the shared `.git` at their Mac paths, so edit
files with your usual tools. `target/` is a volume inside the container, so build
output is not visible from the Mac, except `target/verification`, which stays on
the Mac after `down`. `exec` runs one command in the
[unattended dev shell](#unattended-development) and needs no terminal, so agents
use it the same way; `dev/apple-session.sh exec bash` opens a shell for the longer
steps below.

A file replaced from the Mac or another session can read as missing for about a
second, which shows up as errors like `failed to read index: ... no longer exists`.
Run the same command again; the error does not mean the session is broken or
justify `--no-verify`.

Sessions run with `CODA_SESSION_CPUS`, `CODA_SESSION_MEMORY` (12 GB by default)
and `CODA_SESSION_JOBS`. A second clone of the repository on the same Mac needs its
own `CODA_SESSION_PREFIX` and `CODA_SESSION_PORT_BASE`. The cloning swaps the image
file inside a volume, which `container` has no command for, so a new release of
the tool could need a change here.

Servers keep their usual ports inside the session. `up` prints the Mac address of
each one, and `exec` binds them where those addresses reach. The local practice
builds the physician page and runs on the session's own Postgres:

```sh
url=$(dev/apple-session.sh postgres)
dev/apple-session.sh exec env PHYSICIAN_DATABASE_URL="$url" scripts/local-practice.sh
```

The UI gallery builds and serves with one command, here on the published port:

```sh
dev/apple-session.sh exec scripts/ui-gallery.sh
```

For live or offline physician chat, follow [local physician chat](#local-physician-chat).

The session's Postgres lasts as long as the session. The database-backed tests
need a cluster of their own, so run these in `dev/apple-session.sh exec bash`. Point the tests at it as
[CI](.github/workflows/ci.yml) does, and clear the encoder model path unless that
directory exists, because the dev shell always sets it. The browser tests build
offline, so fetch their crates first:

```sh
initdb -D /tmp/coda-pg -U postgres --auth=trust
pg_ctl -D /tmp/coda-pg -o "-p 5544 -k /tmp" -l /tmp/coda-pg.log start
export DUET_TEST_DATABASE_URL=postgresql://postgres@127.0.0.1:5544/postgres
export PHYSICIAN_TEST_DATABASE_URL=$DUET_TEST_DATABASE_URL
export FRONT_DOOR_TEST_DATABASE_URL=$DUET_TEST_DATABASE_URL
export CLINICAL_LIBRARY_TEST_DATABASE_URL=$DUET_TEST_DATABASE_URL
unset DUET_DISCOVERY_ENCODER_MODEL CODA_PATIENT_ENCODER_MODEL
cargo test --workspace
cargo fetch --locked --target wasm32-unknown-unknown
cargo test -p physician-sample --test site --test auth_browser --test sign_in_states --test shell_states -- --ignored
```

`duet-api` also needs the [schema setup](book/src/backend-architecture.md#persistence-and-migrations)
and its Doppler secrets, which the unattended shell leaves out.

Commit through the session, so the pre-commit hook runs. The hook cannot run on a
Mac without Nix. `up` copies your Git identity from the Mac. Fetching submodules and
pushing stay on the Mac, which holds your credentials:

```sh
dev/apple-session.sh exec git commit -m "Your message"
git push
```
