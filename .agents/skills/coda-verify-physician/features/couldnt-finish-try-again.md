# Couldn't finish, Try again

When the worker cannot finish a turn within the subscription's 5 delivery attempts, the
physician's message stays saved. The conversation shows the mechanical line
`Coda couldn’t finish this response. Your message is saved.` with a `Try again` button.
`Try again` sends `work.resume` for the named work and run; the reply then arrives, and
pressing it again cannot reopen finished work. Owners: `durable_snapshot_body` and
`resume_durable` in `crates/physician-sample/src/live.rs`, the conversation renderer in
`crates/physician-app/src/lib.rs`, `MAX_FAILED_ATTEMPTS` in
`crates/harness/src/engine.rs`, and `tests/try_again.rs`.

## Sub-features

- `retry-dead-letter` — five failed deliveries reach `physician-chat-turns-dead-letter`.
- `retry-saved` — the snapshot keeps the physician text, has no reply, and lists the run as `failed`.
- `retry-line` — one `mechanical` item carries the exact copy and `resume_target {work_item_id, expected_run_id}`.
- `retry-wrong-run` — `work.resume` naming another run is `rejected` and changes nothing.
- `retry-resume` — `work.resume` on the named run is `accepted`, and the reply `Noted.` arrives with no mechanical line left.
- `retry-idempotent` — the same resume key again is `accepted` (same request); a new key after completion is `rejected`.
- `retry-button` — the page renders the line and the `Try again` button in the Conversation region.

## How to get to it (user POV)

- Open Abigail Lemieux on a chat (`?patient=…&chat=<id>`), send a message Coda cannot
  finish, wait for the line, and choose `Try again` beside it.
- `Try again` on a matching Today row uses the same `work.resume` command. Today is empty
  until a real owner fills it, so this recipe does not drive that entry.

## Driving it with physician-drive

Preconditions:

- Start from the shared stack in [README](./README.md). Set `F=$(jq -r .worker.failures_file "$RUN/stack.json")`.
- Set `C=chat-$(python3 -c 'import uuid; print(uuid.uuid4().hex)')`, `OUT="$RUN/couldnt-finish-try-again"` and `T='Hello'`.
  Run `printf 5 > "$F"` before sending. The stub fails five exchanges. Try again can then publish.

- **Send.** Run `$D --out "$OUT" send --site a --session "$S" --chat "$C" --text "$T"`. `send-a.json` `.body.outcome` is `accepted`.
- **Couldn't finish (cross-instance).** Run `$D --out "$OUT" snapshot --site b --session "$S" --chat "$C" --wait-failed --timeout 120`. Exit `0` after the Pub/Sub backoff (about 10–25 s). `.body.snapshot.conversation` is `[{kind: physician, text: "Hello"}, {kind: mechanical, text: "Coda couldn’t finish this response. Your message is saved.", resume_target: {...}}]`, and `.runs` is `[{run_id, state: failed}]` whose `run_id` equals `expected_run_id`.
- **Dead letter.** Run `$D --out "$OUT" dead-letters` until `.messages` has one entry whose `.scope.chat` is `$C`. Each pull acknowledges nothing, but the emulator can return an empty batch before the forward lands.
- **Durable state.** Run `psql "$(jq -r .postgres.url "$RUN/stack.json")" -At -c "select c.status, c.state->>'failures', (select count(*) from physician_runtime_chat_attempts a where a.chat = c.chat) from physician_runtime_conversations c where c.chat='$C'"`. Output: `failed|5|5`.
- **Wrong run.** Run `$D --out "$OUT/wrong" resume --site a --session "$S" --chat "$C" --run not-the-run`. Exit `1`; `.body.outcome` is `rejected`, and a re-read snapshot still shows the mechanical line.
- **Try again.** Run `$D --out "$OUT" resume --site a --session "$S" --chat "$C"`. Exit `0`; `.body.outcome` is `accepted` and `.body.run_id` is the failed run.
- **Reply.** Run `$D --out "$OUT/after" snapshot --site b --session "$S" --chat "$C" --wait-text Noted.`. The conversation is `[Hello, Noted.]` with no `mechanical` item, and `.runs` holds one run with `state: completed` and a new `run_id`.
- **Pressed twice.** `$D resume` needs a failed turn, so it cannot repeat Try again after the reply. Run `CODA_REQUIRE_POSTGRES_TESTS=1 PHYSICIAN_RUNTIME_TEST_DATABASE_URL="$(jq -r .postgres.url "$RUN/stack.json")" cargo test -p physician-sample --test try_again`: the same key `resume-<work>-<run>` answers `accepted` again, and a new key after completion answers `rejected`.
- **Button, from the page.** Set `printf 5 > "$F"` again and use a new chat `P=chat-$(python3 -c 'import uuid; print(uuid.uuid4().hex)')` on the same stack. Open `$($D page-url --site a --chat "$P")`, sign in, write `Hello`, and choose `Send`. Without reloading, wait (up to 120 s) for `Coda couldn’t finish this response. Your message is saved.` and the `Try again` button in `.conversation-mechanical`. Save `$OUT/site-a-failed.png` and the `Conversation` accessibility snapshot. Choose `Try again`, wait for `Noted.`, and confirm the line and button are gone. Save `$OUT/site-a-after.png`. Then run the **Dead letter**, **Durable state** and **Reply** checks above for `$P`. After Try again, **Durable state** prints `completed|0|6`: the new run resets the failures, and its attempt completed.

## Gotchas

- The provider stub survives worker restarts. Each failed delivery is retained on the conversation.
- `--wait-failed` waits for the `mechanical` item that carries `resume_target`. The `Coda is writing this response.` line is also `mechanical`, but it carries `stop_target`, so the wait does not stop on it.
- While redeliveries back off, the snapshot shows `Coda is writing this response.` and no reply. That is the expected in-between state, not a failure.
- A `rejected` resume answers HTTP 200. Check `.body.outcome`, not the status.
- The emulator forwards the dead letter asynchronously. Poll `dead-letters` a few times before you call it missing.
