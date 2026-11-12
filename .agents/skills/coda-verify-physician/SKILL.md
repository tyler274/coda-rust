---
name: coda-verify-physician
description: "Verify the physician site's hosted-shape chat journeys on a disposable local scale stack: two physician-sample instances, physician-worker, Postgres and Valkey. Use for cross-instance chat, couldn't-finish/Try again, Valkey-restart resync, and duplicate/retried work."
---

# Verify the physician scale stack

See [Coda verification](../../../verification/README.md). The design
lives in [book/src/physician-site.md#scaling-across-instances](../../../book/src/physician-site.md)
and [book/src/gcp.md](../../../book/src/gcp.md); the feature map is
[features/README.md](features/README.md). This recipe drives the real Rust binaries over
their real HTTP and SSE entry points, with the worker claiming accepted work from
Postgres as it does when hosted. It never touches staging, Doppler,
Google credentials or a model key: the worker answers with the fixed text `Noted.`.

Every command runs inside the nix dev shell from the worktree root. If direnv is not
active, prefix with `nix develop --command`.

## When a change needs this

Run the matching [feature](features/README.md) when a change touches:

- `crates/physician-sample/src/live.rs` (chat snapshots, events and `resume_durable`) or
  `crates/physician-sample/src/bin/physician-worker.rs`.
- `crates/physician-bus` (cross-instance fan-out and resync).
- `crates/local-product/src/runtime.rs` (durable chat runs and their failed attempts).
- The conversation renderer in `crates/physician-app/src/lib.rs` or `src/shell.rs`.
- `scripts/physician-scale-stack.py`, this skill's driver, or the `fanout`, `try_again`,
  `worker` and `scale` tests in `crates/physician-sample/tests`.

## Launch

1. Build the three binaries, the page and the charts. Use a private target directory when
   other work shares `target/`:

   ```bash
   CARGO_TARGET_DIR=target/stream-e cargo build -p physician-sample --bins
   PHYSICIAN_RUNTIME_TEST_DATABASE_URL=postgres://... \
     cargo test -p physician-sample --test site -- --ignored # also writes target/physician-web
   cargo run -p patient-bank -- build                        # writes target/patient-bank/bank
   ```

   The site test needs a disposable Postgres: without `PHYSICIAN_RUNTIME_TEST_DATABASE_URL`
   (or `PHYSICIAN_DATABASE_URL`) its database-backed tests fail with `browser proof database`.
   `scripts/physician-scale-stack.py --dependencies-only --evidence <dir> -- cargo test -p
   physician-sample --test site -- --ignored` runs it against the stack's own Postgres.

   The page assets are served from `target/physician-web` (or `--site-dir`). A stale
   page or a stale binary silently drops recent behavior, so rebuild after pulling. A page
   built before `?chat=` existed ignores it and shows `This workspace has no chats.`.

2. Start the stack in its own terminal and leave it running:

   ```bash
   scripts/physician-scale-stack.py --bin-dir target/stream-e/debug
   ```

   The stack starts a loopback Responses stub. For a retry journey, set the number
   of upcoming provider responses that break the stream contract, each failing its attempt, in the file identified by `.worker.failures_file`
   in `stack.json` before sending the message. Run those journeys one at a time.
   The worker waits 1 s after a first failure and doubles it each time, so five failures
   take about 15 s; `--worker-retry-seconds 10` gives the hosted 10 s start.

   Readiness is these lines, in order:

   ```text
   Site A: http://127.0.0.1:<a> (port <a>)
   Site B: http://127.0.0.1:<b> (port <b>)
   Worker: http://127.0.0.1:<w> (port <w>); stub
   Sites: http://127.0.0.1:<a> and http://127.0.0.1:<b>
   Dependencies ready; Redis <r>, Postgres <g>. Logs: /tmp/...
   Evidence: <repo>/target/verification/runs/physician-scale-stack-<time>-<pid> ... Restart Valkey: kill -USR1 <pid>. Stop: kill -TERM <pid>
   ```

   Every port is a fresh loopback port; never assume a previous run's port. The stack runs
   `physician-migrate`, seeds the synthetic account `physician@codahealth.test`
   (`roger-synthetic-practice` / `staging-physician`), runs `physician-migrate load-charts`
   (`--charts`, about 30 s), and starts the sites one at a time. Sites run
   `PHYSICIAN_AUTH=required` with proof sign-in: `/auth/sign-in` answers the token in its
   body.

3. Optional load numbers: add `--load-numbers [--idle-seconds 20]`. After readiness it
   writes `<evidence>/load-numbers.json` (see [Evidence](#evidence)) and then keeps
   running (or exits with `--check`).

## Doctor

Read-only. Run before every drive and require each result.

```bash
RUN=$(ls -td target/verification/runs/physician-scale-stack-* | head -1)
jq '{pid, sites, worker, ports, ready_at, stopped_at}' "$RUN/stack.json"   # ready_at set, stopped_at absent
kill -0 "$(jq -r .pid "$RUN/stack.json")"                                     # the stack you launched is alive
for u in $(jq -r '.sites.a, .sites.b, .worker.url' "$RUN/stack.json"); do curl -fsS -o /dev/null -w "%{http_code} $u/readyz\n" "$u/readyz"; done   # 200 x3
curl -fsS -o /dev/null -w "%{http_code} page\n" "$(jq -r .sites.a "$RUN/stack.json")/physician-app_bg.wasm"   # 200, else the page is not built
D=.agents/skills/coda-verify-physician/scripts/physician-drive.py
$D subscribers                       # 3: site A, site B and the worker on physician:workspace-events:v1
S=$($D sign-in --site a) && echo "${#S}"   # a bearer session; proof sign-in works
jq '.worker | {fixed_failures, failures_file, retry_seconds}' "$RUN/stack.json"   # initial count, stub control file and 1
jq 'has("pubsub")' "$RUN/stack.json"   # false: no Pub/Sub process belongs to this stack
```

`physician-drive.py` reads the newest `physician-scale-stack-*/stack.json` (or `--stack`)
and refuses a stack whose pid is gone. Never drive an instance this run did not start.

## Drive

Pick the feature from [features/README.md](features/README.md). Each recipe pairs a user
action with an HTTP/SSE command through `physician-drive.py` and, where the page can
show it, a browser step on site A or B.

**Opening a chat.** `GET /patient-workspaces/{id}` lists the signed-in physician's
runtime chats, newest first, titled by their first message. The page's `New conversation`
button (Conversations sidebar, and the empty Conversation region) opens a new `chat-…` id
and writes it to `?chat=`. A drive that needs a known id opens it from the address:
`$D page-url --site b --chat "$C"` prints
`http://127.0.0.1:<b>/?patient=pw-2a995cfc98bbfd3c4e6c1d7492ee6998&chat=$C`. Before its
first message the page lists that chat as `Untitled`, opens it, and shows
`What would you like Coda to review?`. A message sent with the page's own `Send` button
creates the chat. The drive can also create it with the same `POST /commands`
`message.submit` that `tests/try_again.rs` and `tests/fanout.rs` use (`$D send`). After
the first message, a reload without `&chat=` still lists the chat, under its title.

Shared driving conventions:

- Sign in once per stack: `S=$($D sign-in --site a)`. The session works on both sites
  because both share the stack's Postgres. The browser signs in separately (see
  [Visual readback](#visual-readback)). It is the same physician, so it reads the same chats.
- New chat per drive: `C=chat-$(python3 -c 'import uuid; print(uuid.uuid4().hex)')`.
- Every evidence-producing call takes `--out "$RUN/<feature>"`.

## Evidence

Everything lives under `target/verification/runs/physician-scale-stack-<time>-<pid>/`
and survives cleanup:

| Artifact | Written by | Holds |
| --- | --- | --- |
| `stack.json` | the stack, at start and on each Valkey restart and exit | pids, ports, URLs, `fixed_failures`, `failures_file`, `retry_seconds`, `valkey_restarts`, `ready_at`, `stopped_at` |
| `logs/{postgres,valkey,worker,site-a,site-b}.log` | the stack, copied on exit | full process output |
| `<feature>/send-<site>.json`, `snapshot-<site>.json`, `resume-<site>.json`, `watch-<site>-<event>.json`, `announce.json` | `physician-drive.py --out` | the request key, HTTP status and full response body |
| `load-numbers.json` | `--load-numbers` | see below |
| `<feature>/*.png`, `*.aria.txt` | the browser driver | screenshot and accessibility snapshot |

`load-numbers.json` reads `pg_stat_database.xact_commit` on the stack's own database. It
waits 11 s (Postgres' stats flush interval) before each read and subtracts the probe's own
transaction. Fields:

- `idle_without_streams` and `idle_with_two_streams` hold `{before, after, transactions}`
  across `idle_window_seconds`. A held event stream must not poll Postgres, so both counts
  stay at the same small background level.
- `one_turn` holds the turn's transactions, including the reader's snapshot polls, plus
  `submit_to_reply_seconds` for a reply submitted on A and seen on B. It is skipped under
  a nonzero `--worker-failures` setting.
- `stream_close_after_sign_out` records how long a stream held on site B takes to close
  after its session signs out on site A, and confirms other sessions' streams stay open.

Run-time dimension: the numbers come from a laptop with `fsync=off`. They show shape
(no idle polling, fast cross-instance close), not hosted capacity.

## Visual readback

Use a browser driver against `Site A` or `Site B` with a desktop viewport:

- Navigate to `$D page-url --site <a|b> --chat "$C"`. The sign-in screen appears first.
- Sign in: fill `input[type="email"]` with `physician@codahealth.test`, choose `.control-primary`.
  Proof delivery redeems the token automatically, and the page opens Abigail Lemieux on `$C`.
  It does not go to Today, because the first shell after sign-in keeps `?patient=` and `?chat=`.
- Wait for `Records as of` and the `Conversation` region's `Write a message` composer.
  Write with `textarea.composer-input`, then choose the `Send` button in `.composer-toolbar`.
- Capture a screenshot plus the accessibility snapshot of the `Conversation` region.
  Look at the image: a DOM string can be present while the panel is clipped or off-screen.
  Report visual failures beside the functional HTTP pass, not instead of it.

## Cleanup

```bash
kill -TERM "$(jq -r .pid "$RUN/stack.json")"     # or Ctrl-C in the stack's terminal
jq .stopped_at "$RUN/stack.json"                  # set
ls "$RUN/logs"                                    # six logs retained, including responses.log
pgrep -af 'physician-sample|physician-worker|valkey-server'   # nothing from this run
```

The stack owns only its temporary directory and child process groups; stopping it
removes the database and the Valkey state. Keep `target/verification/runs/`.
Do not kill processes that belong to another run.

## Helpers

- [`scripts/physician-scale-stack.py`](../../../scripts/physician-scale-stack.py):
  `--worker-failures N`, `--worker-retry-seconds S`, `--load-numbers`, `--idle-seconds`, `--evidence DIR`,
  `--site-dir DIR`, `--check`, `--dependencies-only`, `-- <child command>`.
  `kill -USR1 <pid>` restarts Valkey on the same port.
- [`scripts/physician-drive.py`](scripts/physician-drive.py) (executable):
  `sign-in`, `page-url`, `send`, `snapshot --wait-text/--wait-failed`, `resume [--run]`,
  `watch --until <event>`, `announce`, `restart-valkey`, `subscribers`.

## Coverage boundary

This stack stands in for hosted Cloud Run, Memorystore and Cloud SQL. The provider is
the fixed text, mail is proof delivery, and the worker's retry delay starts at 1 s where
hosted starts at 10 s. Those remain unverified here and are proved only in staging per
[book/src/gcp.md](../../../book/src/gcp.md). The inline fallback for a site without
`PHYSICIAN_DURABLE_CHAT=1` is not on this stack; `crates/physician-sample` tests cover it.
A worker stopped or killed mid-turn, and a worker whose listener is gone, are proved in
`crates/physician-sample/tests/instance_loss.rs` and `tests/worker.rs`, not here.
Gmail push wake is proved only as a controlled Pub/Sub emulator test against a Gmail
stand-in (`crates/gmail/tests/push_wake.rs`), which starts its own emulator.
It has no feature file: nothing a physician sees changes until the product's Gmail intake exists.
