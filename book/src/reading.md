# How to use this book

Linear owns product intent, scope and acceptance. This book documents Rust
owners, setup and proof. Code establishes what is implemented; it does not
turn an old implementation into an approved product decision. Dev's latest
explicit decisions take precedence. When an outcome and implementation disagree,
keep the gap visible and update the responsible owner as the work lands.

| Question | Canonical owner |
| --- | --- |
| What are we building and why? | [Duet](https://linear.app/coda-health/project/duet-ed7c815ed39b), [Coda Agents](https://linear.app/coda-health/project/coda-agents-d7d16847168e), [Connected care](https://linear.app/coda-health/project/connected-care-v1-9f745b9c6874), and their outcome tickets; [engineering scope and proof](https://linear.app/coda-health/project/coda-rust-platform-6ca86460e169) |
| How should Duet look and read? | [Design language](https://linear.app/coda-health/document/duet-design-language-foundations-patterns-and-current-care-e3d0bc28fa6d) |
| Where is it implemented? | [Crate map](crate-map.md), then the owning source file and tests |
| How do I work here? | <a href="/AGENTS%2Emd">AGENTS.md</a>, [operating invariants](agent-contracts.md), and <a href="/README%2Emd">README setup</a> |
| What has been proved? | The scoped test/run evidence linked by each outcome; <a href="/verification/README%2Emd">verification</a> owns formal-design workflow; [release](release.md) and [GCP](gcp.md) own operational procedures |

## Find the implementation

| Area | Navigation |
| --- | --- |
| Accounts, evidence, conversation and saved care | [Duet backend](backend-architecture.md), [security boundaries](security-posture.md), [privacy recovery](privacy-recovery.md) |
| Patient screens and design references | [Duet UI](ui-surface-map.md) |
| Invitations, consent and practice connections | [Accounts and practice connection](front-door.md) |
| Physician client, sign-in and execution | [Physician site](physician-site.md) |
| Shared execution and product boundaries | [Operating invariants](agent-contracts.md), [crate map](crate-map.md) |
| Statecharts, design decisions and formal proof | <a href="/verification/README%2Emd">Verification</a>, <a href="/verification/physician-experience/README%2Emd">physician experience</a> |
| Synthetic patients for running Coda | [Synthetic patients](patient-bank.md) |
| Builds, deployment and diagnostics | [Release](release.md), [GCP](gcp.md), [observability](observability.md) |

Keep ticket references, lists, status and acceptance tracking in Linear. This book
describes maintained source owners, implemented behavior and operating procedures;
it does not mirror the backlog. Product and runtime changes follow the agreed
design and verification workflow.

## Retained references

The original repositories remain read-only comparison material. Their pinned
contracts and release packets explain prior behavior and evidence; they cannot
override current product decisions. Follow the original source when comparing
or porting a behavior. Do not recreate a chapter for every historical heading.

- [Original Duet documentation](https://github.com/Coda-Health/coda-app/tree/5a1dc47b8feacb8db3fcc71522982f76822c0b01/docs): behavior, clinical catalogs and licenses, privacy, historical iOS/TestFlight release packets, scenario and insight research.
- [Original physician documentation](https://github.com/Coda-Health/coda-agents/tree/5674d0f13895214e0a633fc81176107f2e28f79d/docs): architecture, runtime protocol, physician experience, care-team operations and readiness.
- [October 21 Rust design review](https://linear.app/coda-health/document/duet-rust-review-input-for-tylers-design-pass-a6562ebd1c45): dated observations, not a current implementation inventory or acceptance contract.
- <a href="/submodules/coda-gym/README%2Emd">Coda Gym</a> and <a href="/submodules/Coda-Computer-Use/README%2Emd">computer use</a> retain their own task packages, source evidence and operating instructions; neither has a Rust product crate here.

Fixtures, generated API contracts, published legal text and active verification
work are not obsolete merely because product scope changed. Remove a runtime
surface through its outcome with consumer and authority checks, not through a
documentation cleanup.
