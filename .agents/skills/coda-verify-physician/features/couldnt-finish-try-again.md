# Couldn't finish, Try again

When the worker cannot finish a turn within the subscription's 5 delivery attempts, the
physician's message stays saved. The conversation shows the mechanical line
`Coda couldn’t finish this response. Your message is saved.` with a `Try again` button.
`Try again` sends `work.resume` for the named work and run; the reply then arrives, and
pressing it again cannot reopen finished work. Owners: `durable_snapshot_body` and
`resume_durable` in `crates/physician-sample/src/live.rs`, the conversation renderer in
`crates/physician-app/src/lib.rs`, `MAX_FAILED_ATTEMPTS` in
`crates/local-product/src/runtime.rs`, and `tests/try_again.rs`.

## Sub-features

- `retry-dead-letter` — five failed deliveries reach `physician-chat-turns-dead-letter`.
- `retry-saved` — the snapshot keeps the physician text, has no reply, and lists the run as `failed`.
- `retry-line` — one `mechanical` item carries the exact copy and `resume_target {work_item_id, expected_run_id}`.
- `retry-wrong-run` — `work.resume` naming another run is `rejected` and changes nothing.
- `retry-resume` — `work.resume` on the named run is `accepted`, and the reply `Noted.` arrives with no mechanical line left.
- `retry-idempotent` — the same resume key again is `accepted` (same request); a new key after completion is `rejected`.
- `retry-button` — the page renders the line and the `Try again` button in the Conversation region.

## How to get to it (user POV)

- Open Mary Louis on a chat (`?patient=…&chat=<id>`), send a message Coda cannot finish,
  wait for the line, and choose `Try again` beside it.
- `Try again` on a matching Today row uses the same `work.resume` command. The recorded
  Today does not list chat turns, so this recipe does not drive that entry.

## Driving it with physician-drive

Preconditions:

- Baseline from [README](./README.md), on a **fresh** stack started with
  `--worker-failures 5`. `jq .worker.fixed_failures "$RUN/stack.json"` prints `5`.
- `C=chat-$(uuidgen | tr -d -)`; `OUT="$RUN/couldnt-finish-try-again"`.

- **Send.** Run `$D --out "$OUT" send --site a --session "$S" --chat "$C" --text Hello`. `send-a.json` `.body.outcome` is `accepted`.
- **Couldn't finish (cross-instance).** Run `$D --out "$OUT" snapshot --site b --session "$S" --chat "$C" --wait-failed --timeout 120`. Exit `0` after the Pub/Sub backoff (about 10–25 s). `.body.snapshot.conversation` is `[{kind: physician, text: Hello}, {kind: mechanical, text: "Coda couldn’t finish this response. Your message is saved.", resume_target: {...}}]`, and `.runs` is `[{run_id, state: failed}]` whose `run_id` equals `expected_run_id`.
- **Dead letter.** Run `$D --out "$OUT" dead-letters` until `.messages` has one entry whose `.scope.chat` is `$C`. Each pull acknowledges nothing, but the emulator can return an empty batch before the forward lands.
- **Durable state.** Run `psql "$(jq -r .postgres.url "$RUN/stack.json")" -At -c "select state->>'status', state->>'failures', jsonb_array_length(state->'attempts') from physician_runtime_conversations where chat='$C'"`. Output: `failed|5|5`.
- **Wrong run.** Run `$D --out "$OUT/wrong" resume --site a --session "$S" --chat "$C" --run not-the-run`. Exit `1`; `.body.outcome` is `rejected`, and a re-read snapshot still shows the mechanical line.
- **Try again.** Run `$D --out "$OUT" resume --site a --session "$S" --chat "$C"`. Exit `0`; `.body.outcome` is `accepted` and `.body.run_id` is the failed run.
- **Reply.** Run `$D --out "$OUT/after" snapshot --site b --session "$S" --chat "$C" --wait-text Noted.`. The conversation is `[Hello, Noted.]` with no `mechanical` item, and `.runs` is `[]`.
- **Pressed twice.** Repeat the **Try again** command: `.body.outcome` is `accepted` again because it reuses the key `resume-<work>-<run>`. A new key after completion, `$D resume ... --run <same run>` with a different key, is not exposed by the helper. Prove it with `cargo test -p physician-sample --test try_again` (it asserts `rejected`).
- **Button, from the page (preferred order on a fresh `--worker-failures 5` stack).** Run this instead of the HTTP **Send**/**Try again** steps when proving `retry-button`. Open `$($D page-url --site a --chat "$C")`, sign in, write `Hello`, and choose `Send`. Without reloading, wait (up to 120 s) for `Coda couldn’t finish this response. Your message is saved.` and the `Try again` button in `.conversation-mechanical`. Save `$OUT/site-a-failed.png` and the `Conversation` accessibility snapshot. Choose `Try again`, wait for `Noted.`, and confirm the line and button are gone. Save `$OUT/site-a-after.png`. Then run the **Dead letter**, **Durable state** and **Reply** checks above for the same `$C`. `--worker-failures 5` fails only the first turn, so one stack proves either the page path or the HTTP path, not both.

## Gotchas

- `--worker-failures 5` counts provider calls per worker process. After the first dead-letter, every later turn on that stack succeeds. Restart the stack for a second failure drive, and do not use it for happy-path features.
- `--wait-failed` keys on the `mechanical` item. Before Stream A's snapshot change, a failed turn shows only `[{text: Hello}]`, so a stale `physician-sample` binary times out here even though Postgres says `failed`. Rebuild first.
- The snapshot can briefly show neither a reply nor the line while redeliveries back off. That is the expected in-between state, not a failure.
- A `rejected` resume answers HTTP 200. Check `.body.outcome`, not the status.
- The emulator forwards the dead letter asynchronously. Poll `dead-letters` a few times before you call it missing.
