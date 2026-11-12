# Duplicate and retry

A physician's message runs once, even when the same command is sent twice, through two
sites, or announced to the worker twice. An attempt that fails is retried after a delay
and can still succeed. Owners: command idempotency in `crates/local-product/src/runtime.rs`
(`physician_runtime_commands`, `input_id`), the worker's claim in
`crates/physician-sample/src/bin/physician-worker.rs`, and `tests/worker.rs`
(`hearing_of_settled_work_again_runs_the_fixed_provider_once`,
`a_failed_attempt_is_retried_and_links_to_the_accepting_request`,
`failed_attempts_wait_longer_each_time`,
`one_provider_failure_retries_one_message`).

## Sub-features

- `dup-command` — the same `message.submit` idempotency key on site A and then site B is `accepted` both times and saves one message.
- `dup-conflict` — the same key with different text is `rejected` with `Coda couldn’t accept that message.`.
- `dup-delivery` — a second announcement of a completed turn on `physician_work` makes no new attempt and no second reply.
- `retry-delivery` — the provider stub breaks its first response so the attempt fails, the worker retries it after the delay, and the turn completes with one `Noted.`.

## How to get to it (user POV)

- Pressing `Send` twice, or a network retry of the same send, from either site. A page
  retry (`Check again`) resends the same command with the same key. A second `Send`
  click is a *new* message with a new key, so the duplicate is driven with `$D send --key`.
- Repeated announcements and retries are infrastructure behavior with no user control. The
  physician sees exactly one reply.

## Driving it with physician-drive

Preconditions:

- Start from the shared stack in [README](./README.md). Set `F=$(jq -r .worker.failures_file "$RUN/stack.json")`.
- `C=chat-$(python3 -c 'import uuid; print(uuid.uuid4().hex)')`; `K=dup-$C`; `OUT="$RUN/duplicate-and-retry"`;
  `PG=$(jq -r .postgres.url "$RUN/stack.json")`.

- **Same command twice, two sites.** Run `$D --out "$OUT" send --site a --session "$S" --chat "$C" --text Once --key "$K"` and `$D --out "$OUT" send --site b --session "$S" --chat "$C" --text Once --key "$K"`. Both exit `0`, with `.body.outcome` `accepted`.
- **One reply.** Run `$D --out "$OUT" snapshot --site a --session "$S" --chat "$C" --wait-text Noted.`. The conversation is exactly `[Once, Noted.]`.
- **Conflicting reuse.** Run `$D --out "$OUT/conflict" send --site a --session "$S" --chat "$C" --text Different --key "$K"`. Exit `1`; `.body` is `{"outcome": "rejected", "message": "Coda couldn’t accept that message."}`.
- **Durable count.** Run `psql "$PG" -At -c "select c.status, (select count(*) from physician_runtime_chat_attempts a where a.chat = c.chat), json_array_length(c.state->'messages') from physician_runtime_conversations c where c.chat='$C'"`. Output: `completed|1|2`.
- **Duplicate delivery.** Run `$D --out "$OUT" announce --chat "$C" --key "$K"`. It sends the accepting transaction's `physician_work` notification once more and exits `0`. Wait 2 s (nothing observable changes when the worker ignores it), then re-run the **Durable count** query: it still prints `completed|1|2`, and the snapshot is still `[Once, Noted.]`.
- **Retried delivery.** Run `printf 1 > "$F"` before sending. Set `R=chat-$(python3 -c 'import uuid; print(uuid.uuid4().hex)')`. Run `$D --out "$OUT/retry" send --site a --session "$S" --chat "$R" --text 'Retry'`, then `$D --out "$OUT/retry" snapshot --site b --session "$S" --chat "$R" --wait-text Noted. --timeout 60`. Exit `0` after one retry delay (`stack.json` `.worker.retry_seconds`, 1 s unless the stack names another), and the conversation is `[Retry, Noted.]`. Run `psql "$PG" -At -c "select c.status, (select count(*) from json_array_elements(c.state->'messages') m where m->>'role' = 'assistant'), (select json_agg(json_build_array(a.status, a.expires) order by a.position) from physician_runtime_chat_attempts a where a.chat = c.chat) from physician_runtime_conversations c where c.chat='$R'" | tee "$OUT/retry/attempts.txt"`. Output: `completed|1|[["failed", <t1>], ["completed", <t2>]]` (one reply; the first attempt failed, the retry completed). The failed attempt's `expires` is the database time it settled, while the completed one keeps its claim time plus the 60 s lease, so `t2 - t1 - 60` is the wait between the two attempts: at least the retry delay.

## Gotchas

- The idempotency key is scoped to the signed-in physician, not to the site. Use one session on both sites to prove the cross-site duplicate.
- A rejected conflicting key answers HTTP 200. Assert `.body.outcome`, not the status code.
- The worker serves no work route: `curl -s -o /dev/null -w "%{http_code}\n" -X POST "$(jq -r .worker.url "$RUN/stack.json")/pubsub/chat-turn"` prints `404`. Work reaches it only through Postgres.
- A worker restart leaves the provider stub running and the conversation retains its attempt history.
- `--worker-failures N` still exists and still fails the first N provider calls of the process, whatever the message. Leave it at `0` when the stack also serves happy paths.
- `--key` must equal the key of the accepted command. An announcement naming another input is ignored as stale, which proves nothing about duplicates.
