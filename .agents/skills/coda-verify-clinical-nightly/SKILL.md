---
name: coda-verify-clinical-nightly
description: "Check a change to the clinical library's records or nightly failure rules against their spec: the crate's rule tests, then its Postgres suite on a local cluster. Use when a change touches the clinical-library crate, its schema, or the nightly spec."
---

# Verify the clinical library nightly

The spec is [verification/clinical-library/nightly/README.md](../../../verification/clinical-library/nightly/README.md).
It has no model yet: the types, the schema's constraints and the tests below hold its rules. The
harm when one breaks: a deferred night counts as a success and a stalled library stays quiet, a
broken nightly keeps retrying, a night that lost its slot overwrites the one that took it, or search
serves an unchecked or second version.

## When a change needs this

Run it when a change touches any of these:

- `crates/clinical-library/src/` (article states, night outcomes, the stop rule, the slot, versions
  and the store) and `crates/clinical-library/src/migration/` (the schema).
- `crates/clinical-library/tests/postgres.rs`, `crates/clinical-library/tests/pubmed.rs` and
  `crates/clinical-library/tests/fixtures/`.
- The nightly README.

## Run

From the repository root, in the [dev shell](../../../AGENTS.md#dev-shell), with a local Postgres
started as the [README](../../../README.md) describes for the database-backed tests:

```sh
export CLINICAL_LIBRARY_TEST_DATABASE_URL=postgresql://postgres@127.0.0.1:5544/postgres
cargo test -p clinical-library
```

It passes when the unit, Postgres and doc tests all pass, including the two `compile_fail` doc
tests. The Postgres suite fails, rather than skipping, when the variable isn't set.

## What holds each rule

| Rule | Held by |
| --- | --- |
| 1. Only a person, new content or a new reader moves an article out of needing a person or waiting | `ArticleState::apply`; `every_reachable_move_keeps_the_rules` applies every move to every reachable state |
| 2. A night tries only articles waiting to retry | `ArticleState::apply` refuses a failure on any other state; same test, and `item_states_keep_their_rules_in_storage` on Postgres |
| 3. Only a complete night is a success | `NightOutcome`, whose `Succeeded` only the crate can build (`compile_fail` doc test in `night.rs`); `only_complete_nights_succeed`; the nights table's checks (`each_night_records_one_outcome`) |
| 4. The same failure code twice stops | `stop_rule::decide`; `the_same_failure_twice_stops`, `taken_over_nights_count_as_failures`, `the_stop_rule_reads_the_stored_history` |
| 5. Three failed or idle nights in a row stop | `stop_rule::decide`; `three_different_failed_nights_in_a_row_stop`, `deferred_nights_that_moved_nothing_count`, `live_slot_holders_and_partial_nights` |
| 6. Takeover only of an ended night | `slot::takeover` and `Store::acquire`; `unknown_is_never_treated_as_ended`, `a_run_that_lost_its_slot_cannot_write` |
| 7. An unreadable holder alerts on the second night | `slot::owner_unreadable_needs_alert`; `an_unreadable_owner_alerts_on_the_second_night` |
| 8. A night that lost the slot changes nothing | `Store`'s writes take a `Lease`, checked in the same transaction; `a_run_that_lost_its_slot_cannot_write` |
| 9. One active version, and only a checked one | `Candidate`'s states (`compile_fail` doc test in `candidate.rs`); the versions table's checks and partial unique index (`one_checked_version_is_active`) |

PubMed's status, in `tests/pubmed.rs` against NCBI's recorded responses and a local stand-in for
NCBI:

| Behaviour | Held by |
| --- | --- |
| Stored status matches NLM, an unchanged record re-confirms without replacing, and the record is kept exactly as sent, with a hash the database checks | `stored_status_matches_nlm` |
| A changed record replaces the stored status and its links | `a_changed_record_replaces_the_stored_status` |
| A record without publication types is unreadable, not an empty status | `a_record_without_publication_types_is_unreadable` |
| Book records answer their PMIDs, and deleted citations answer nothing | `book_records_answer_their_pmids` (NCBI's recorded response), `deleted_citations_answer_nothing` |
| Every NLM link type from an article to a notice about it is read; the notice's own links aren't | `notices_follow_nlms_link_types` |
| An unreadable record is set aside, tells a person and keeps the last good status | `one_failing_article_is_set_aside_and_the_rest_finish`; `refresh::plan`'s unit tests |
| An outage sets articles aside and a night that moved nothing counts toward rule 5 | `an_outage_sets_articles_aside_without_losing_status` |
| A run that lost its slot records no status | `a_run_that_lost_its_slot_records_no_status` |
| Requests keep within NCBI's rate; throttling and server errors back off; a refusal is never retried and never carries the key | `requests_keep_within_ncbis_rate`, `throttling_and_server_errors_back_off` |
| A refused request fails the night | `a_refused_request_fails_the_night` |

These tests never call NCBI. Against NCBI itself, with the key from Doppler and a local database
the migrate binary has set up:

```sh
export CLINICAL_LIBRARY_DATABASE_URL=postgresql://postgres@127.0.0.1:5544/<database>
cargo run -p clinical-library --bin clinical-library-migrate
doppler run -- cargo run -p clinical-library --bin clinical-library-pubmed-status <pmids-file>
```

It exits 0 when the night succeeded, 2 when it deferred and 1 when it failed.

Only a qualified item takes moves, and a PubMed item's key is a PMID: `Store::qualify` and
`Store::apply`, held by `item_states_keep_their_rules_in_storage`, and `ItemKey`'s `compile_fail` doc
test in `source.rs`.

A fresh start (a success or a recorded resolution) is held by
`a_resolution_or_a_success_starts_the_count_over`. The schema's own rules, one outcome per night,
an append-only history and the retry bound matching `RETRY_LIMIT`, are held by
`each_night_records_one_outcome` and `item_states_keep_their_rules_in_storage`.

## Not held yet

- A failed night resuming from its saved progress, and the alert and its daily reminders: no
  test holds these yet.
- Each candidate building into storage only it uses, so only activation needs the slot: no test
  covers writes to object storage yet.
- Orderings across several nights, a crash and a person acting at once are checked only as far
  as the tests above enumerate them; there is no model of the run lifecycle.
