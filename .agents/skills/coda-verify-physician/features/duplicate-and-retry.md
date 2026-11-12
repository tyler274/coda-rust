# Duplicate and retry

A physician's message runs once, even when the same command is sent twice, through two
sites, or announced to the worker twice. An attempt that fails is retried after a delay
and can still succeed. Owners: command idempotency in `crates/local-product/src/runtime.rs`
(`physician_runtime_commands`, `input_id`), the worker's claim in
`crates/physician-sample/src/bin/physician-worker.rs`, and `tests/worker.rs`
<<<<<<< ours
(`duplicate_delivery_runs_the_fixed_provider_once`,
`failed_delivery_is_redelivered_and_can_succeed`,
`one_provider_failure_retries_one_message`).
||||||| base
(`duplicate_delivery_runs_the_fixed_provider_once`,
`failed_delivery_is_redelivered_and_can_succeed`,
`fail_marker_fails_only_the_marked_message`).
=======
(`hearing_of_settled_work_again_runs_the_fixed_provider_once`,
`a_failed_attempt_is_retried_and_links_to_the_accepting_request`,
`failed_attempts_wait_longer_each_time`,
`fail_marker_fails_only_the_marked_message`).
>>>>>>> theirs

## Sub-features

- `dup-command` — the same `message.submit` idempotency key on site A and then site B is `accepted` both times and saves one message.
- `dup-conflict` — the same key with different text is `rejected` with `Coda couldn’t accept that message.`.
<<<<<<< ours
- `dup-delivery` — a duplicate push of a completed turn to `/pubsub/chat-turn` answers `204` and makes no new attempt and no second reply.
- `retry-delivery` — the provider stub breaks its first response so the attempt fails, Pub/Sub redelivers, and the turn completes with one `Noted.` and no dead letter.
||||||| base
- `dup-delivery` — a duplicate push of a completed turn to `/pubsub/chat-turn` answers `204` and makes no new attempt and no second reply.
- `retry-delivery` — a message marked `[fail:1]` fails its first delivery, Pub/Sub redelivers, and the turn completes with one `Noted.` and no dead letter.
=======
- `dup-delivery` — a second announcement of a completed turn on `physician_work` makes no new attempt and no second reply.
- `retry-delivery` — a message marked `[fail:1]` fails its first attempt, the worker retries it after the delay, and the turn completes with one `Noted.`.
>>>>>>> theirs

## How to get to it (user POV)

- Pressing `Send` twice, or a network retry of the same send, from either site. A page
  retry (`Check again`) resends the same command with the same key. A second `Send`
  click is a *new* message with a new key, so the duplicate is driven with `$D send --key`.
- Repeated announcements and retries are infrastructure behavior with no user control. The
  physician sees exactly one reply.

## Driving it with physician-drive

Preconditions:

<<<<<<< ours
- Start from the shared stack in [README](./README.md). Set `F=$(jq -r .worker.failures_file "$RUN/stack.json")`.
- `C=chat-$(python3 -c 'import uuid; print(uuid.uuid4().hex)')`; `K=dup-$C`; `OUT="$RUN/duplicate-and-retry"`;
  `PG=$(jq -r .postgres.url "$RUN/stack.json")`; `WK=$(jq -r .worker.url "$RUN/stack.json")`.
||||||| base
- Baseline from [README](./README.md), on a stack started with `--worker-fail-marker` and `--worker-failures 0`. `jq .worker.fail_marker "$RUN/stack.json"` prints `true`. Unmarked messages never fail, so `dup-*` and `retry-delivery` share the stack with every other feature.
- `C=chat-$(uuidgen | tr -d -)`; `K=dup-$C`; `OUT="$RUN/duplicate-and-retry"`;
  `PG=$(jq -r .postgres.url "$RUN/stack.json")`; `WK=$(jq -r .worker.url "$RUN/stack.json")`.
=======
- Baseline from [README](./README.md), on a stack started with `--worker-fail-marker` and `--worker-failures 0`. `jq .worker.fail_marker "$RUN/stack.json"` prints `true`. Unmarked messages never fail, so `dup-*` and `retry-delivery` share the stack with every other feature.
- `C=chat-$(uuidgen | tr -d -)`; `K=dup-$C`; `OUT="$RUN/duplicate-and-retry"`;
  `PG=$(jq -r .postgres.url "$RUN/stack.json")`.
>>>>>>> theirs

- **Same command twice, two sites.** Run `$D --out "$OUT" send --site a --session "$S" --chat "$C" --text Once --key "$K"` and `$D --out "$OUT" send --site b --session "$S" --chat "$C" --text Once --key "$K"`. Both exit `0`, with `.body.outcome` `accepted`.
- **One reply.** Run `$D --out "$OUT" snapshot --site a --session "$S" --chat "$C" --wait-text Noted.`. The conversation is exactly `[Once, Noted.]`.
- **Conflicting reuse.** Run `$D --out "$OUT/conflict" send --site a --session "$S" --chat "$C" --text Different --key "$K"`. Exit `1`; `.body` is `{"outcome": "rejected", "message": "Coda couldn’t accept that message."}`.
<<<<<<< ours
- **Durable count.** Run `psql "$PG" -At -c "select c.status, (select count(*) from physician_runtime_chat_attempts a where a.chat = c.chat), json_array_length(c.state->'messages') from physician_runtime_conversations c where c.chat='$C'"`. Output: `completed|1|2`.
- **Duplicate delivery.** Build the push body for the same `{scope, input_id}` and send it to the worker:

  ```bash
  DATA=$(jq -cn --arg c "$C" --arg k "$K" '{kind:"chat_turn",scope:{practice:"roger-synthetic-practice",physician:"staging-physician",workspace:"pw-2a995cfc98bbfd3c4e6c1d7492ee6998",chat:$c},input_id:$k}' | base64 -w0)
  curl -sS -o /dev/null -w "%{http_code}\n" -X POST "$WK/pubsub/chat-turn" -H 'content-type: application/json' \
    -d "{\"message\":{\"data\":\"$DATA\",\"messageId\":\"duplicate-delivery\"},\"subscription\":\"projects/coda-local-physician/subscriptions/physician-chat-turns\"}" | tee "$OUT/duplicate-push.txt"
  ```

  It prints `204`. Re-running the **Durable count** query still prints `completed|1|2`, and the snapshot is still `[Once, Noted.]`.
- **Retried delivery.** Run `printf 1 > "$F"` before sending. Set `R=chat-$(python3 -c 'import uuid; print(uuid.uuid4().hex)')`. Run `$D --out "$OUT/retry" send --site a --session "$S" --chat "$R" --text 'Retry'`, then `$D --out "$OUT/retry" snapshot --site b --session "$S" --chat "$R" --wait-text Noted. --timeout 60`. Exit `0` after about one backoff (1–10 s), and the conversation is `[Retry, Noted.]`. Run `psql "$PG" -At -c "select c.status, (select count(*) from json_array_elements(c.state->'messages') m where m->>'role' = 'assistant'), (select json_agg(a.status order by a.position) from physician_runtime_chat_attempts a where a.chat = c.chat) from physician_runtime_conversations c where c.chat='$R'"`. Output: `completed|1|["failed", "completed"]` (one reply; the first attempt failed, the redelivery completed). `$D --out "$OUT/retry" dead-letters` has no entry whose `.scope.chat` is `$R`.
||||||| base
- **Durable count.** Run `psql "$PG" -At -c "select state->>'status', jsonb_array_length(state->'attempts'), jsonb_array_length(state->'messages') from physician_runtime_conversations where chat='$C'"`. Output: `completed|1|2`.
- **Duplicate delivery.** Build the push body for the same `{scope, input_id}` and send it to the worker:

  ```bash
  DATA=$(jq -cn --arg c "$C" --arg k "$K" '{scope:{practice:"roger-synthetic-practice",physician:"staging-physician",workspace:"pw-2a995cfc98bbfd3c4e6c1d7492ee6998",chat:$c},input_id:$k}' | base64 -w0)
  curl -sS -o /dev/null -w "%{http_code}\n" -X POST "$WK/pubsub/chat-turn" -H 'content-type: application/json' \
    -d "{\"message\":{\"data\":\"$DATA\",\"messageId\":\"duplicate-delivery\"},\"subscription\":\"projects/coda-local-physician/subscriptions/physician-chat-turns\"}" | tee "$OUT/duplicate-push.txt"
  ```

  It prints `204`. Re-running the **Durable count** query still prints `completed|1|2`, and the snapshot is still `[Once, Noted.]`.
- **Retried delivery.** Set `R=chat-$(uuidgen | tr -d -)`. Run `$D --out "$OUT/retry" send --site a --session "$S" --chat "$R" --text 'Retry [fail:1]'`, then `$D --out "$OUT/retry" snapshot --site b --session "$S" --chat "$R" --wait-text Noted. --timeout 60`. Exit `0` after about one backoff (1–10 s), and the conversation is `[Retry [fail:1], Noted.]`. Run `psql "$PG" -At -c "select state->>'status', jsonb_array_length(jsonb_path_query_array(state, '\$.messages[*] ? (@.role == \"assistant\")')), jsonb_path_query_array(state, '\$.attempts[*].status') from physician_runtime_conversations where chat='$R'"`. Output: `completed|1|["failed", "completed"]` (one reply; the first attempt failed, the redelivery completed). `$D --out "$OUT/retry" dead-letters` has no entry whose `.scope.chat` is `$R`.
=======
- **Durable count.** Run `psql "$PG" -At -c "select state->>'status', jsonb_array_length(state->'attempts'), jsonb_array_length(state->'messages') from physician_runtime_conversations where chat='$C'"`. Output: `completed|1|2`.
- **Duplicate delivery.** Run `$D --out "$OUT" announce --chat "$C" --key "$K"`. It sends the accepting transaction's `physician_work` notification once more and exits `0`. Wait 2 s (nothing observable changes when the worker ignores it), then re-run the **Durable count** query: it still prints `completed|1|2`, and the snapshot is still `[Once, Noted.]`.
- **Retried delivery.** Set `R=chat-$(uuidgen | tr -d -)`. Run `$D --out "$OUT/retry" send --site a --session "$S" --chat "$R" --text 'Retry [fail:1]'`, then `$D --out "$OUT/retry" snapshot --site b --session "$S" --chat "$R" --wait-text Noted. --timeout 60`. Exit `0` after one retry delay (`stack.json` `.worker.retry_seconds`, 1 s unless the stack names another), and the conversation is `[Retry [fail:1], Noted.]`. Run `psql "$PG" -At -c "select state->>'status', jsonb_array_length(jsonb_path_query_array(state, '\$.messages[*] ? (@.role == \"assistant\")')), jsonb_path_query_array(state, '\$.attempts[*].status'), jsonb_path_query_array(state, '\$.attempts[*].expires') from physician_runtime_conversations where chat='$R'" | tee "$OUT/retry/attempts.txt"`. Output: `completed|1|["failed", "completed"]|[<t1>, <t2>]` (one reply; the first attempt failed, the retry completed). Each `expires` is the attempt's claim time on the database clock plus the 60 s lease, so `t2 - t1` is the wait between the two attempts: at least the retry delay.
>>>>>>> theirs

## Gotchas

- The idempotency key is scoped to the signed-in physician, not to the site. Use one session on both sites to prove the cross-site duplicate.
- A rejected conflicting key answers HTTP 200. Assert `.body.outcome`, not the status code.
<<<<<<< ours
- The worker accepts unauthenticated pushes only because the stack sets `PUBSUB_EMULATOR_HOST`. Hosted push-token checks are not exercised here.
- A worker restart leaves the provider stub running and the conversation retains its attempt history.
||||||| base
- The worker accepts unauthenticated pushes only because the stack sets `PUBSUB_EMULATOR_HOST`. Hosted push-token checks are not exercised here.
- `[fail:K]` counts the attempts saved on the conversation's current message, so a worker restart or a second worker instance keeps the count. Each retry drive needs a new chat; a new message in the same chat starts at zero.
=======
- The worker serves no work route: `curl -s -o /dev/null -w "%{http_code}\n" -X POST "$(jq -r .worker.url "$RUN/stack.json")/pubsub/chat-turn"` prints `404`. Work reaches it only through Postgres.
- `[fail:K]` counts the attempts saved on the conversation's current message, so a worker restart or a second worker instance keeps the count. Each retry drive needs a new chat; a new message in the same chat starts at zero.
>>>>>>> theirs
- `--worker-failures N` still exists and still fails the first N provider calls of the process, whatever the message. Leave it at `0` when the stack also serves happy paths.
- `--key` must equal the key of the accepted command. An announcement naming another input is ignored as stale, which proves nothing about duplicates.
