# Hard-stop recovery

When a worker process stops abruptly while it holds a turn, nothing hands the turn back.
Once its lease runs out, another worker claims it, its successor attempt names the
interrupted one as predecessor, and the physician sees exactly one reply. A rehearsal
turn marked `[crash:1]` makes the worker abort on purpose with the attempt still leased.
Owners: `Rehearsal` in `crates/physician-sample/src/hosting.rs`, `chat_attempt` and
`rehearsed_crash` in `crates/physician-sample/src/bin/physician-worker.rs`, and
`tests/instance_loss.rs`.

## Sub-features

- `hard-stop-hold` — a `[hold:S]` attempt keeps its lease past the lease length by renewing it.
- `hard-stop-receipt` — the worker logs `physician.worker.rehearsed_crash` with `process.pid`, `coda.run_id`, `coda.work_id` and `coda.lease_expires_unix`, then exits by SIGABRT.
- `hard-stop-takeover` — no successor starts before the lease runs out; after it, the other worker's successor has `predecessor` equal to the interrupted attempt and the same work.
- `hard-stop-one-reply` — attempts end `[interrupted, completed]` with one failure, and the conversation holds exactly one assistant reply.
- `hard-stop-real-ignored` — a turn in a real practice carrying the same markers answers on its first attempt, and its worker keeps running.

## How to get to it (user POV)

- A physician sees nothing unusual: the reply arrives later than usual, once.
- Only rehearsal practices on the `staging-preview` profile honour the markers, so the
  journey is driven by the test locally and by the rehearsal physician on staging.

## Driving it with physician-drive

The local scale stack runs one worker without a rehearsal practice, so `physician-drive`
does not drive this feature. Use the two-worker test locally and the drill on staging.

- **Locally.** With a disposable Postgres, run
  `CODA_REQUIRE_POSTGRES_TESTS=1 PHYSICIAN_DATABASE_URL=<url> cargo test -p physician-sample --test instance_loss -- --include-ignored --nocapture --test-threads=1 | tee "$RUN/hard-stop/instance_loss.txt"`.
  `a_rehearsal_crash_is_recovered_by_the_other_worker_after_its_lease` prints
  `rehearsed_crash: aborted_unix=… completed_unix=… predecessor=… successor=…` and the
  receipt line; `a_real_practice_ignores_hold_and_crash_markers` passes. The SIGKILL
  tests print `receipt pid=… signalled_unix=… lease_expires_unix=… taken_unix=…`.
- **Staging.** Only with the operator's approval for an exclusive window: run
  `workspace-tools observability-drill langfuse hard-stop --confirm`. It refuses while
  work is due, sends one `[hold:30][crash:1]` turn, and writes
  `langfuse-hard-stop-<stamp>.json` with both attempt ids, the predecessor link, the one
  reply and a closing normal turn. Read the crash receipt from Cloud Logging
  (`jsonPayload.event="physician.worker.rehearsed_crash"`) and open the chat in the
  browser as the rehearsal physician to confirm one reply.

## Gotchas

- The abort drops every attempt that worker instance was running, up to four. Each one
  recovers by lease expiry too, so run the staging drill only with nothing else due.
- With one worker instance, the successor waits for Cloud Run to start a replacement
  instance as well as for the lease, so recovery can take longer than the 60 s lease.
- `[fail:K]` and `[crash:K]` count saved attempts. A conversation reused for a second
  drill has those attempts already, so start a new chat each time.
