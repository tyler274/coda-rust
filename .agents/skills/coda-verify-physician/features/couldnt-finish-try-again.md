# Couldn't finish, Try again

When the worker cannot finish a turn within its 5 attempts, the
physician's message stays saved. The conversation shows the mechanical line
`Coda couldn’t finish this response. Your message is saved.` with a `Try again` button.
`Try again` sends `work.resume` for the named work and run; the reply then arrives, and
pressing it again cannot reopen finished work. Owners: `snapshot_body` and
`resume_durable` in `crates/physician-sample/src/live.rs`, the conversation renderer in
`crates/physician-app/src/lib.rs`, `MAX_FAILED_ATTEMPTS` in
`crates/harness/src/engine.rs`, and `tests/try_again.rs`.

## Sub-features

- `retry-backoff` — the five failed attempts begin further apart each time, and nothing starts a sixth.
- `retry-saved` — the snapshot keeps the physician text and has no reply.
- `retry-line` — the exchange's answer is `{state: couldnt_finish, try_again: {work_item_id, expected_run_id}}`.
- `retry-wrong-run` — `work.resume` naming another run is `rejected` and changes nothing.
- `retry-resume` — `work.resume` on the named run is `accepted`, and the same exchange's answer becomes `finished` with the text `Noted.`.
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
  Run `printf 5 > "$F"` before sending. The stub fails five exchanges, so all five attempts fail
  and the sixth, from Try again with the same `input_id`, answers.

- **Send.** Run `$D --out "$OUT" send --site a --session "$S" --chat "$C" --text "$T"`. `send-a.json` `.body.outcome` is `accepted`.
- **Couldn't finish (cross-instance).** Run `$D --out "$OUT" snapshot --site b --session "$S" --chat "$C" --wait-failed --timeout 120`. Exit `0` after the four retry delays (1 + 2 + 4 + 8 s on the default stack). `.body.snapshot.transcript` is one exchange whose `messages[0].text` is `Hello` and whose `answer` is `{state: couldnt_finish, try_again: {work_item_id, expected_run_id}}`.
- **Growing delay.** Run `psql "$(jq -r .postgres.url "$RUN/stack.json")" -At -c "select status, expires from physician_runtime_chat_attempts where chat='$C' order by position" | tee "$OUT/attempts.txt"`. Output: five `failed` rows. A failed attempt's `expires` is the database time it settled, so the differences between neighbours are the waits (plus each attempt's brief run): about 1, 2, 4 and 8 s, each at least the one before. Run it again after 20 s: still five attempts, because a turn that gave up is never due again.
- **Durable state.** Run `psql "$(jq -r .postgres.url "$RUN/stack.json")" -At -c "select c.status, c.state->>'failures', (select count(*) from physician_runtime_chat_attempts a where a.chat = c.chat) from physician_runtime_conversations c where c.chat='$C'"`. Output: `failed|5|5`.
- **Wrong run.** Run `$D --out "$OUT/wrong" resume --site a --session "$S" --chat "$C" --run not-the-run`. Exit `1`; `.body.outcome` is `rejected`, and a re-read snapshot still shows the answer as `couldnt_finish` with the same `try_again`.
- **Try again.** Run `$D --out "$OUT" resume --site a --session "$S" --chat "$C"`. Exit `0`; `.body.outcome` is `accepted` and `.body.run_id` is the failed run's `expected_run_id`.
- **Reply.** Run `$D --out "$OUT/after" snapshot --site b --session "$S" --chat "$C" --wait-text Noted.`. The transcript is still one exchange: `Hello`, answered `{state: finished, text: "Noted.", ...}`.
- **Pressed twice.** `$D resume` needs a failed turn, so it cannot repeat Try again after the reply. Run `CODA_REQUIRE_POSTGRES_TESTS=1 PHYSICIAN_RUNTIME_TEST_DATABASE_URL="$(jq -r .postgres.url "$RUN/stack.json")" cargo test -p physician-sample --test try_again`: the same key `resume-<work>-<run>` answers `accepted` again, and a new key after completion answers `rejected`.
- **Button, from the page.** Set `printf 5 > "$F"` again and use a new chat `P=chat-$(python3 -c 'import uuid; print(uuid.uuid4().hex)')` on the same stack. Open `$($D page-url --site a --chat "$P")`, sign in, write `Hello`, and choose `Send`. Without reloading, wait (up to 120 s) for `Coda couldn’t finish this response. Your message is saved.` and the `Try again` button in `.conversation-mechanical`. Save `$OUT/site-a-failed.png` and the `Conversation` accessibility snapshot. Choose `Try again`, wait for `Noted.`, and confirm the line and button are gone. Save `$OUT/site-a-after.png`. Then run the **Growing delay**, **Durable state** and **Reply** checks above for `$P`. After Try again, **Durable state** prints `completed|0|6`: the new run resets the failures, and its attempt completed.

## Gotchas

- The provider stub survives worker restarts. Each failed attempt is retained on the conversation, so a worker restart keeps the count.
- `--wait-failed` waits for the newest exchange's answer to carry `resume` or `try_again`. While retries are still running the answer is `working` with a `stop` target instead, which does not count.
- The snapshot can show neither a reply nor the line while retries wait out their delay. That is the expected in-between state, not a failure.
- A `rejected` resume answers HTTP 200. Check `.body.outcome`, not the status.
- The waits are whole seconds on the database clock and a poll can add to one, so read them as "at least", not as exact doubles.
