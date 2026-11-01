# Resync after a Valkey restart

When the shared Valkey bus drops and comes back, every held page stream on every site
gets `event: resync`. The page then refetches what it shows instead of trusting a feed
that may have missed changes. Fan-out works again once the sites and the worker
resubscribe. Owners: `crates/physician-bus` (`Event::Resync`), the SSE writer in
`crates/physician-sample/src/live.rs`, the client's `event_refetch` in
`crates/physician-app/src/shell.rs`, and `tests/scale` (`ValkeyServer::restart`).

## Sub-features

- `resync-event` — a stream held on site B receives `event: resync` with `data: {}` after the restart.
- `resync-both-sites` — the stream on site A receives it too.
- `resync-recover` — after the restart the bus has 3 subscribers again, and a new turn sent on A reaches B live.
- `resync-page` — the page held on a workspace refetches on `resync` and stays signed in with its content.

## How to get to it (user POV)

- Keep Today or Abigail Lemieux's workspace open while the bus restarts. The physician sees
  no error; the page quietly refetches.

## Driving it with physician-drive

Preconditions:

- Baseline from [README](./README.md). Unmarked messages never fail on the shared stack.
- `OUT="$RUN/resync-after-redis-restart"`.

- **Hold streams on both sites.** Run `$D --out "$OUT" watch --site b --session "$S" --until resync --timeout 60 &` and `$D --out "$OUT" watch --site a --session "$S" --until resync --timeout 60 &`.
- **Page held on B.** Set `C=chat-$(uuidgen | tr -d -)` and send one turn on it (`$D send --site a ... --chat "$C"`, then `snapshot --site b --wait-text Noted.`). In the browser, open `$($D page-url --site b --chat "$C")`, sign in, wait for `Noted.`, and save `$OUT/before.png`.
- **Restart Valkey.** Run `$D restart-valkey`, which sends SIGUSR1 to the stack. The stack prints `Valkey restarted on <port> (1)`, and `jq .valkey_restarts "$RUN/stack.json"` prints `1`.
- **Resync on both.** `wait`. Both watchers exit `0`; `watch-a-resync.json` and `watch-b-resync.json` have `seen: true`, and `frames` contains `event: resync\ndata: {}`. Measured locally: about 2 s from the signal.
- **Subscribers back.** Run `$D subscribers` until it prints `3`.
- **Fan-out recovers.** Follow [cross-instance chat](./cross-instance-chat.md) with a new chat: send on A, hold `watch --site b --until workspace_changed`, and read `Noted.` on B.
- **Page after.** Save `$OUT/after.png` and the accessibility snapshot. Abigail Lemieux's `Records as of` and the conversation (`Noted.`) are still shown, the page did not return to `Sign in to Coda`, and no `Coda couldn’t open this conversation.` notice appears. Then send a second message from the page's composer and see its `Noted.` arrive live. That proves the page's own stream recovered, not only the drive's watchers.

## Gotchas

- Start both watchers before the restart. A stream opened after reconnection never sees the `resync`.
- One restart can produce more than one `resync` while the client reconnects. Do not assert exactly one.
- `restart-valkey` kills Valkey with SIGKILL and restarts it empty, which is the real failure mode. Nothing persisted in Valkey survives, by design, because only the bus lives there.
- Postgres `NOTIFY` (sign-in and panel changes) is not affected by this restart. Do not use a panel change to prove bus recovery.
- In the stack's own terminal, `fred` logs `Connection closed` warnings during the restart. They are expected.
