# Cross-instance chat

A physician sends a message through one site instance. The worker answers it, and a page
held on another instance gets the reply live and still shows it after a reload. The site
accepts the turn into Postgres and announces `{scope, input_id}` there, the worker claims
it, and the change notice goes out on the Valkey
bus ([book](../../../../book/src/physician-site.md), `crates/physician-bus`,
`crates/physician-sample/src/bin/physician-worker.rs`, `tests/fanout.rs`).

## Sub-features

- `xchat-accept` — `message.submit` on site A answers `accepted` and saves the message.
- `xchat-live` — a stream held on site B receives `workspace_changed` for the workspace.
- `xchat-reply` — the snapshot read on site B holds the physician text and the reply `Noted.`.
- `xchat-reload` — a fresh session on site B reads the same conversation after the turn.
- `xchat-page` — the page on site B, open on the chat, shows the reply `Noted.` live without a reload.
- `xchat-page-send` — the page's own composer and `Send` on site A create the chat and get the reply.
- `xchat-new-chat` — `New conversation` on site A opens a new chat and writes `?chat=`; after a turn, a reload on site B without `?chat=` lists it under its first message.

## How to get to it (user POV)

- Open Abigail Lemieux, choose `New conversation`, write in `Write a message`, and choose
  `Send`. A chat named in the address (`?patient=…&chat=<id>`) opens directly.
- A second browser or tab on the other site, open on the same chat.

## Driving it with physician-drive

Preconditions:

- Baseline from [README](./README.md). Unmarked messages never fail on the shared stack.
- `C=chat-$(uuidgen | tr -d -)`; `OUT="$RUN/cross-instance-chat"`.

- **Page open on B.** In the browser, open `$($D page-url --site b --chat "$C")` and sign in (see [Visual readback](../SKILL.md#visual-readback)). Wait for `What would you like Coda to review?` and `Write a message`. Save `$OUT/site-b-before.png`.
- **Hold the stream on B.** Run `$D --out "$OUT" watch --site b --session "$S" --until workspace_changed --timeout 60 &`. Stderr prints `watching site b (HTTP 200)`.
- **Send on A.** Run `$D --out "$OUT" send --site a --session "$S" --chat "$C" --text "Cross-instance hello"`. Exit `0`; `send-a.json` `.body.outcome` is `accepted` and `.body.chat_id` is `$C`.
- **Live notice on B.** `wait` for the watcher. Exit `0`; `watch-b-workspace_changed.json` has `seen: true` and its frames name `pw-2a995cfc98bbfd3c4e6c1d7492ee6998`.
- **Reply on B.** Run `$D --out "$OUT" snapshot --site b --session "$S" --chat "$C" --wait-text Noted.`. Exit `0`; `.body.snapshot.conversation` is `[{kind: physician, text: "Cross-instance hello"}, {kind: model, text: "Noted."}]` and `.runs` is `[]`.
- **Reload on B.** Run `S2=$($D sign-in --site b)`, then `$D --out "$OUT/reload" snapshot --site b --session "$S2" --chat "$C"`. The same two items come back.
- **Page on B, live.** Without reloading, wait for `Noted.` under `Cross-instance hello` in the `Conversation` region. Save `$OUT/site-b.png` and the region's accessibility snapshot.
- **Send from the page on A.** Set `C2=chat-$(uuidgen | tr -d -)`. In a second tab, open `$($D page-url --site a --chat "$C2")`, write `Sent from the page` in `textarea.composer`, and choose `Send`. Then run `$D --out "$OUT/page-send" snapshot --site b --session "$S" --chat "$C2" --wait-text Noted.`: the conversation is `[Sent from the page, Noted.]`. The page on A shows the same two lines. Save `$OUT/site-a-page-send.png`.
- **New conversation, then reload on B.** In a tab on A, open `$($D page-url --site a)` (no `--chat`) and choose `New conversation`. The address gains `chat=chat-…`; set `C3` to it. Write `Listed after reload` and choose `Send`, then wait for `Noted.`. Open `$($D page-url --site b)` on B (no `&chat=`). The Conversations sidebar lists `Listed after reload`; choosing it shows `[Listed after reload, Noted.]`. Save `$OUT/site-b-listed.png`. HTTP proof: `curl -fsS -H "authorization: Bearer $S" "$(jq -r .sites.b "$RUN/stack.json")/patient-workspaces/$(jq -r .workspace "$RUN/stack.json")" | jq '.chats' > "$OUT/workspace-chats-b.json"` holds `{chat_id: $C3, title: "Listed after reload"}`.
- **Timing.** For the A→B reply time, run the stack with `--load-numbers`. `load-numbers.json` `.one_turn.submit_to_reply_seconds` is that measurement, and `.reply_seen_on_site_b` is `true`.

## Gotchas

- A chat is listed after a reload only once its first message is accepted. A `New conversation` with nothing sent is not saved anywhere, so a reload drops it.
- The composer keeps its text after `Send` (the page does not clear the draft). Judge the send by the conversation lines and the send notice, not by an empty box. `Coda couldn’t accept that message.` beside the composer means the command was refused.
- Read the reply on the *other* site. A read on A succeeds even when the bus is down, because both sites share Postgres.
- `watch` exits on the first matching event. Start it before `send`, or it can miss the notice.
- `stack.json` ports change every launch. Always re-read `$RUN` after restarting the stack.
- A stack started with `--worker-failures N` fails its first N turns, whatever the message. Use `--worker-fail-marker` with `--worker-failures 0` here.
