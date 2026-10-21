# Duet Backend Architecture

Account moments are the slice in [`duet-api`](/crates/duet-api/src/lib.rs). Request bytes with no schema are [`http`](/crates/http/src/lib.rs). The rest of this architecture is still the Python service under `submodules/coda-app`.

- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#duet-backend-architecture">Duet Backend Architecture</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#architecture-boundaries">Architecture boundaries</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#execution-isolation">Execution isolation</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#execution-inventory">Execution inventory</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#composition-and-release-contract">Composition and release contract</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#for-my-doctor">For my doctor</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#bounded-maintenance">Bounded maintenance</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#acquisition-worker-design">Acquisition worker design</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#composition-and-access">Composition and access</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#bounded-execution-and-recovery">Bounded execution and recovery</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#worker-capacity-and-admission">Worker capacity and admission</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#capacity-and-acceptance">Capacity and acceptance</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#time-decisions">Time decisions</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#request-path">Request path</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#minimum-supported-iphone-build">Minimum supported iPhone build</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#model-cost-controls">Model cost controls</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#connector-path">Connector path</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#canonical-evidence-reads">Canonical evidence reads</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#calculation-ownership">Calculation ownership</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#incremental-apple-health-acquisition">Incremental Apple Health acquisition</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#canonical-evidence-reads-1">Canonical evidence reads</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#bounded-client-reads">Bounded client reads</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#agent-and-intelligence-path">Agent and intelligence path</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#model-facing-views-and-vocabulary">Model-facing views and vocabulary</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#measurement-reads">Measurement reads</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#conversation-context-and-understanding-lifecycle">Conversation context and understanding lifecycle</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#ahead-of-need-continuation-integration">Ahead-of-need continuation integration</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#unified-discovery-and-exact-recall">Unified discovery and exact recall</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#account-event-streams">Account event streams</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#accepted-follow-through">Accepted follow-through</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#reply-feedback">Reply feedback</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#scheduled-work">Scheduled work</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#privacy">Privacy</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#patient-isolation">Patient isolation</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#data-lifecycle">Data lifecycle</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#account-export-and-deletion">Account export and deletion</a>
- <a href="/submodules/coda-app/docs/backend-architecture%2Emd#schema-changes">Schema changes</a>

## Rust database-client versions and proof

[PR #36](https://github.com/Coda-Health/coda-rust/pull/36), merged as
`ee8ae88c9675bd008ff91e45dc799c88f13d5f24`, aligned `duet-api` with SQLx 0.9
(locked to 0.9.0) and the `physician-sample` test dependency with SeaORM 2
(locked to 2.0.4), matching the existing physician-auth/front-door stack.
This is a **Rust dependency/API migration**, not a database schema or data migration.
It removes the old SQLx 0.8 / SeaORM 1 stack and `webpki-roots` 0.26 from the
lockfile; the resolved TLS dependencies include `rustls-webpki` 0.103.16 and
`webpki-roots` 1.0.10. It does not combine Duet and physician accounts, clinical
models or runtime authority, or add a database to the [public site](public-site.md).

The integration repair uses `query_all_raw` for the preconstructed SQL `Statement`
in the [migration proof](/crates/physician-sample/tests/migrations_postgres.rs).
It changes no SQL, persistence semantics, schema history or TLS trust policy.
Upstream main supplied the migration-name/front-door persistence fixes and the
fresh-Postgres CI job; the public-site PR did not introduce those repairs.

Default workspace tests can skip database-gated checks. Use the existing
[Postgres entrypoint](/scripts/check-postgres.sh) against a **disposable PostgreSQL
16 instance with pgvector** to prove real persistence. Set `MIGRATION_DATABASE_URL`,
`PHYSICIAN_DATABASE_URL` and `FRONT_DOOR_DATABASE_URL` to that isolated server;
the tests create/drop databases and must never point at a shared or production
instance. The script requires all three URLs, sets `CODA_REQUIRE_POSTGRES_TESTS=1`
and explicitly runs the ignored fresh migration-chain test before the three store
suites. Run from the workspace root:

```bash
nix develop --command python scripts/check-sql.py
nix develop --command bash scripts/check-postgres.sh
```

The SQL check covers tracked authored `.sql` files outside submodules (three
Duet queries at this merge), not SeaQuery-generated DDL or arbitrary SQL embedded
in Rust. The final merged-tree local run passed the migration-chain test (both
chains alone and together, with two no-op reruns), seven physician-auth tests,
eight front-door tests and two physician-sample front-door tests: **18 total**.
The earlier pre-integration failures were resolved by upstream fixes and are not
outstanding database blockers. The
[final PR CI run](https://github.com/Coda-Health/coda-rust/actions/runs/39521784174)
also passed its fresh PostgreSQL job. The original regression/negative proofs
remain tracked in [ENG-163](https://linear.app/coda-health/issue/ENG-163).

Passing cargo-deny on this tree is separate from dependency remediation: the
inherited ZIP 8.6.0 exceptions are unchanged and remain open under
[ENG-108](https://linear.app/coda-health/issue/ENG-108).
