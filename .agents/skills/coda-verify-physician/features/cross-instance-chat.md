# Cross-instance chat

A physician sends a message through one site instance. The worker answers it, and a page
held on another instance gets the reply live and still shows it after a reload. The turn
travels as `{scope, input_id}` on Pub/Sub and the change notice goes out on the Valkey
bus ([book](../../../../book/src/physician-site.md), `crates/physician-bus`,
`crates/physician-sample/src/bin/physician-worker.rs`, `tests/fanout.rs`).

## Sub-features

- `xchat-accept` — `message.submit` on site A answers `accepted` and saves the message.
- `xchat-live` — a stream held on site B receives `workspace_changed` for the workspace.
- `xchat-reply` — the snapshot read on site B holds the physician text and the reply `Noted.`.
- `xchat-reload` — a fresh session on site B reads the same conversation after the turn.
- `xchat-page` — the page on site B, open on the chat, shows the reply `Noted.` live without a reload.
- `xchat-page-send` — the page's own composer and `Send` on site A create the chat and get the reply.

## How to get to it (user POV)

- Open Mary Louis on a chat (`?patient=…&chat=<id>`), write in `Write a message`, and
  choose `Send`. The recorded workspace lists no chats, so the address names the chat.
- A second browser or tab on the other site, open on the same chat.

## Driving it with physician-drive

Preconditions:

- Baseline from [README](./README.md), stack started with `--worker-failures 0`.
- `C=chat-$(uuidgen | tr -d -)`; `OUT="$RUN/cross-instance-chat"`.

- **Page open on B.** In the browser, open `$($D page-url --site b --chat "$C")` and sign in (see [Visual readback](../SKILL.md#visual-readback)). Wait for `What would you like Coda to review?` and `Write a message`. Save `$OUT/site-b-before.png`.
- **Hold the stream on B.** Run `$D --out "$OUT" watch --site b --session "$S" --until workspace_changed --timeout 60 &`. Stderr prints `watching site b (HTTP 200)`.
- **Send on A.** Run `$D --out "$OUT" send --site a --session "$S" --chat "$C" --text "Cross-instance hello"`. Exit `0`; `send-a.json` `.body.outcome` is `accepted` and `.body.chat_id` is `$C`.
- **Live notice on B.** `wait` for the watcher. Exit `0`; `watch-b-workspace_changed.json` has `seen: true` and its frames name `pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e`.
- **Reply on B.** Run `$D --out "$OUT" snapshot --site b --session "$S" --chat "$C" --wait-text Noted.`. Exit `0`; `.body.snapshot.conversation` is `[{kind: physician, text: "Cross-instance hello"}, {kind: model, text: "Noted."}]` and `.runs` is `[]`.
- **Reload on B.** Run `S2=$($D sign-in --site b)`, then `$D --out "$OUT/reload" snapshot --site b --session "$S2" --chat "$C"`. The same two items come back.
- **Page on B, live.** Without reloading, wait for `Noted.` under `Cross-instance hello` in the `Conversation` region. Save `$OUT/site-b.png` and the region's accessibility snapshot.
- **Send from the page on A.** Set `C2=chat-$(uuidgen | tr -d -)`. In a second tab, open `$($D page-url --site a --chat "$C2")`, write `Sent from the page` in `textarea.composer`, and choose `Send`. Then run `$D --out "$OUT/page-send" snapshot --site b --session "$S" --chat "$C2" --wait-text Noted.`: the conversation is `[Sent from the page, Noted.]`. The page on A shows the same two lines. Save `$OUT/site-a-page-send.png`.
- **Timing.** For the A→B reply time, run the stack with `--load-numbers`. `load-numbers.json` `.one_turn.submit_to_reply_seconds` is that measurement, and `.reply_seen_on_site_b` is `true`.

## Gotchas

- The chat only appears when the address names it (`&chat=`). Reloading without it shows `This workspace has no chats.`, because the workspace projection does not list runtime chats. That is a product gap, not a failed turn.
- The composer keeps its text after `Send` (the page does not clear the draft). Judge the send by the conversation lines and the send notice, not by an empty box. `Coda couldn’t accept that message.` beside the composer means the command was refused.
- Read the reply on the *other* site. A read on A succeeds even when the bus is down, because both sites share Postgres.
- `watch` exits on the first matching event. Start it before `send`, or it can miss the notice.
- `stack.json` ports change every launch. Always re-read `$RUN` after restarting the stack.
- A stack started with `--worker-failures` fails the first turns. Use a 0-failure stack here.
