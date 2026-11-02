# How to use this book

The <a href="/intent/README%2Emd">intent specs</a> own product intent: principles, product specs and Dev's
decisions. Linear tracks scope, work and acceptance. This book documents Rust
owners, setup and proof. Code establishes what is implemented; it does not
turn an old implementation into an approved product decision. Dev's latest
explicit decisions take precedence. When an outcome and implementation disagree,
Linear tracks the gap, and the responsible owner changes as the work lands.

| Question | Canonical owner |
| --- | --- |
| What are we building and why? | <a href="/intent/README%2Emd">Intent specs</a>: principles, then the physician, Duet, connected-care and harness specs, then decisions; Linear tracks the work: [Duet](https://linear.app/coda-health/project/duet-ed7c815ed39b), [Coda Agents](https://linear.app/coda-health/project/coda-agents-d7d16847168e), [Connected care](https://linear.app/coda-health/project/connected-care-v1-9f745b9c6874), and their outcome tickets; [engineering scope and proof](https://linear.app/coda-health/project/coda-rust-platform-6ca86460e169) |
| How should Duet look and read? | [Design language](https://linear.app/coda-health/document/duet-design-language-foundations-patterns-and-current-care-e3d0bc28fa6d) |
| Where is it implemented? | [Crate map](crate-map.md), then the owning source file and tests |
| How do I work here? | <a href="/AGENTS%2Emd">AGENTS.md</a> and <a href="/README%2Emd">README setup</a> |
| What has been proved? | The scoped test/run evidence linked by each outcome; <a href="/verification/README%2Emd">verification</a> owns formal-design workflow; [release](release.md) and [GCP](gcp.md) own operational procedures |

## Find the implementation

| Area | Navigation |
| --- | --- |
| Accounts, evidence, conversation and saved care | [Duet backend](backend-architecture.md), [security boundaries](security-posture.md), [privacy recovery](privacy-recovery.md) |
| Patient screens and design references | [Duet UI](ui-surface-map.md) |
| Invitations, consent and practice connections | [Accounts and practice connection](front-door.md) |
| Physician client, sign-in and execution | [Physician site](physician-site.md) |
| The physician clinical skill catalog | [`skills/`](/skills), read by `agent-skills` for `harness`; [skills in hosted chat](physician-site.md#skills-in-hosted-chat) |
| Public marketing site | [Public site](public-site.md) |
| Gym cases, promotion and runs | <a href="/submodules/coda-gym/AGENTS%2Emd">coda-gym AGENTS.md</a> |
| Shared execution and product boundaries | <a href="/intent/harness%2Emd">Harness spec</a>, [crate map](crate-map.md) |
| Feature models, design decisions and their checks | <a href="/verification/README%2Emd">Verification</a>, <a href="/verification/physician/sign-in/README%2Emd">physician sign-in</a> |
| Synthetic patients for running Coda | [Synthetic patients](patient-bank.md) |
| Builds, deployment and diagnostics | [Release](release.md), [GCP](gcp.md), [observability](observability.md) |
| Dev container setup, ports and storage | <a href="/README%2Emd#dev-container">Dev container</a> |

Keep ticket references, lists, status and acceptance tracking in Linear. This book
describes maintained source owners, implemented behavior and operating procedures;
it does not mirror the backlog. Product and runtime changes follow the agreed
design and verification workflow.

## Read the physician product as one workflow

Start with what the physician should experience, then follow that promise through
the app and its runtime. These documents answer different questions:

| Read in order | What it establishes |
| --- | --- |
| <a href="/intent/physician%2Emd">Physician spec</a> and its decisions | The agreed behavior: Today, the patient page, conversations, readings, work and communication, including capabilities not built yet. |
| <a href="/intent/harness%2Emd">Harness spec</a> and [physician site](physician-site.md) | The guarantees left to execution, then the Rust owners and execution paths that implement parts of them. Recovery and scaling support the experience; they do not establish coverage of every product workflow. |
| <a href="/verification/README%2Emd">Verification</a> and the <a href="/.agents/skills/coda-verify-physician/SKILL%2Emd">physician verification recipe</a> | The boundary between graph/model checks, implementation checks and running-app readback. The current physician recipe exercises chat across a local scale stack with a fixed provider. |
| [GCP staging](gcp.md#rust-staging-bootstrap-and-deployment-gate) and [release](release.md) | Deployment. A deployed revision is not itself an accepted physician journey. |

For the next implementation slice, use Linear's current outcome and the physician
spec together: choose a complete physician journey, identify missing server data
and runtime guarantees, then verify the result through the app. Do not infer the
next product priority from the latest infrastructure merge.

Connected-care behavior belongs to the
<a href="/intent/connected-care%2Emd">connected-care spec</a>. It does not merge the
physician's and Duet's accounts, clinical state or runtime owners.

## Retained references

The original repositories remain read-only comparison material. Their pinned
contracts and release packets explain prior behavior and evidence; they cannot
override current product decisions. Follow the original source when comparing
or porting a behavior. Do not recreate a chapter for every historical heading.

- [Original Duet documentation](https://github.com/Coda-Health/coda-app/tree/5a1dc47b8feacb8db3fcc71522982f76822c0b01/docs): behavior, clinical catalogs and licenses, privacy, historical iOS/TestFlight release packets, scenario and insight research.
- [Original physician documentation](https://github.com/Coda-Health/coda-agents/tree/5674d0f13895214e0a633fc81176107f2e28f79d/docs): architecture, runtime protocol, physician experience, care-team operations and readiness.
- <a href="/submodules/coda-gym/README%2Emd">Coda Gym</a> and <a href="/submodules/Coda-Computer-Use/README%2Emd">computer use</a> retain their own task packages, source evidence and operating instructions; neither has a Rust product crate here.

Fixtures, generated API contracts, published legal text and active verification
work are not obsolete merely because product scope changed. Remove a runtime
surface through its outcome with consumer and authority checks, not through a
documentation cleanup.
