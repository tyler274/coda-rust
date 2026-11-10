# Physician scale-stack verification map

This directory is the maintained source for verifying the physician site's chat journeys
when the site runs as more than one instance behind a worker. Read this index, then use
the matching feature file as the recipe. Launch, doctor, evidence and cleanup live in
[../SKILL.md](../SKILL.md).

## Baseline preconditions

- The binaries in `target/stream-e/debug` (or the `--bin-dir` you pass) were built from
  the current worktree, `target/physician-web` holds the current page, and
  `target/patient-bank/bank` holds the current charts.
- `scripts/physician-scale-stack.py --bin-dir target/stream-e/debug --worker-fail-marker`
  is running in its own terminal and has printed `Sites: http://127.0.0.1:<a> and http://127.0.0.1:<b>`.
- `RUN=$(ls -td target/verification/runs/physician-scale-stack-* | head -1)` names that
  stack, and every [doctor](../SKILL.md#doctor) check passes.
- `D=.agents/skills/coda-verify-physician/scripts/physician-drive.py` and
  `S=$($D sign-in --site a)` are set in the driving shell.
- The workspace is Abigail Lemieux, `pw-2a995cfc98bbfd3c4e6c1d7492ee6998` (`stack.json`
  `.workspace`), a bank patient on the staging panel. Search finds her, and
  `$D page-url` opens her page.

## Driving conventions

- Every feature starts a new chat, `C=chat-$(uuidgen | tr -d -)`. The browser opens it
  at `$D page-url --chat "$C"` (`?patient=…&chat=…`; see [../SKILL.md](../SKILL.md#drive)).
  The page's `Send` or `$D send` creates it.
- Send on one site and read on the other. A pass on a single instance proves nothing
  about fan-out.
- Wait on a state (`--wait-text`, `--wait-failed`, `watch --until`), never a fixed sleep.
  The only exception is the worker's retry delay, which the recipe names.
- Use `--out "$RUN/<feature-id>"` on every call whose result is proof.
- One stack serves every feature. With `--worker-fail-marker`, a message containing
  `[fail:K]` fails its first K attempts, counted from its saved attempts; unmarked messages never
  fail. Retry uses `[fail:1]`, Try again uses `[fail:5]`, and both leave `fixed_failures` 0.

## Proof and skip reporting

- HTTP proof is the JSON file `physician-drive.py` writes: request key, status, full body.
- Stream proof is the `watch-*.json` frames, including the event name and elapsed seconds.
- Durable proof is a second read on the *other* site, or a `psql` query of
  `physician_runtime_conversations` against `stack.json` `.postgres.url`.
- Browser proof is a screenshot plus an accessibility snapshot of the `Conversation`
  region with the page identity (`Records as of`, the site's port) visible. An HTTP
  pass never stands in for a browser step. If the page is stale or a step fails to
  render, report that step as unverified beside the HTTP result.
- Record the feature ID and site with each artifact.

## Feature entry contract

Each feature file starts with an H1 title and one paragraph describing the user-visible
behavior. It then has exactly four H2s: `Sub-features`, `How to get to it (user POV)`,
`Driving it with physician-drive`, and `Gotchas`.

## Features

- [Cross-instance chat](./cross-instance-chat.md): a message sent through one site gets
  its reply, live and on reload, through the other.
- [Couldn't finish, Try again](./couldnt-finish-try-again.md): a turn the worker gives up
  on keeps the message, shows the couldn't-finish line, and Try again gets the reply.
- [Resync after a Valkey restart](./resync-after-redis-restart.md): held streams get
  `resync` and the page refetches; fan-out works again afterward.
- [Duplicate and retry](./duplicate-and-retry.md): a repeated command or announcement
  runs the turn once, and a failed attempt is retried.
