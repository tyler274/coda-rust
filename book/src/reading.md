# How to use this book

Linear owns product intent, scope and acceptance. This book maps that intent to
Rust owners, setup and proof. Code establishes what is implemented; it does not
turn an old implementation into an approved product decision. Dev's latest
explicit decisions take precedence. When an outcome and implementation disagree,
keep the gap visible and update the responsible owner as the work lands.

| Question | Canonical owner |
| --- | --- |
| What are we building and why? | [Duet](https://linear.app/coda-health/project/duet-ed7c815ed39b), [Coda Agents](https://linear.app/coda-health/project/coda-agents-d7d16847168e), [Connected care](https://linear.app/coda-health/project/connected-care-v1-9f745b9c6874), and their outcome tickets; [engineering scope and proof](https://linear.app/coda-health/project/coda-rust-platform-6ca86460e169) |
| How should Duet look and read? | [Design language](https://linear.app/coda-health/document/duet-design-language-foundations-patterns-and-current-care-e3d0bc28fa6d), with open choices in [ENG-168](https://linear.app/coda-health/issue/ENG-168) |
| Where is it implemented? | [Crate map](crate-map.md), then the owning source file and tests |
| How do I work here? | <a href="/AGENTS%2Emd">AGENTS.md</a>, [operating invariants](agent-contracts.md), and <a href="/README%2Emd">README setup</a> |
| What has been proved? | The scoped test/run evidence linked by each outcome; <a href="/verification/README%2Emd">verification</a> owns formal-design workflow; [release](release.md) and [GCP](gcp.md) own operational procedures |

## Current outcomes and implementation

| Outcome | Linear owner | Rust navigation |
| --- | --- | --- |
| Standalone identity, privacy, export and deletion | [ENG-161](https://linear.app/coda-health/issue/ENG-161) | [Backend](backend-architecture.md), [security](security-posture.md), [privacy recovery](privacy-recovery.md) |
| Continuous conversation and accurate memory | [ENG-26](https://linear.app/coda-health/issue/ENG-26) | [Backend](backend-architecture.md), [UI](ui-surface-map.md) |
| Source acquisition and exact evidence | [ENG-162](https://linear.app/coda-health/issue/ENG-162) | [Backend](backend-architecture.md) |
| Today, Plans, reminders and Check-ins | [ENG-34](https://linear.app/coda-health/issue/ENG-34) | [Backend](backend-architecture.md), [UI](ui-surface-map.md) |
| One invited practice, fixed consent at join, leave/discharge | [ENG-70](https://linear.app/coda-health/issue/ENG-70) | [Practice connection](front-door.md) |
| Consented evidence and documented care instructions | [ENG-39](https://linear.app/coda-health/issue/ENG-39), [ENG-45](https://linear.app/coda-health/issue/ENG-45) | [Practice connection](front-door.md) |
| Permanent removal of person-to-person Shared | [ENG-141](https://linear.app/coda-health/issue/ENG-141) | [UI](ui-surface-map.md) |
| MVP patient-owned doctor workspace: preparation, updates, review and saving | [ENG-169](https://linear.app/coda-health/issue/ENG-169) | [Backend](backend-architecture.md), [UI](ui-surface-map.md); connected sharing stays under ENG-70/ENG-39 |
| Post-MVP sharing with physicians outside Coda | [ENG-169](https://linear.app/coda-health/issue/ENG-169) | Later browser publication, no PDF download |
| Post-MVP jointly maintained live patient–physician shared space | [ENG-27](https://linear.app/coda-health/issue/ENG-27) | Background design; distinct from the patient-owned doctor workspace |
| Separate product authority over shared execution mechanics | [ENG-159](https://linear.app/coda-health/issue/ENG-159) | [Operating invariants](agent-contracts.md), [crate map](crate-map.md) |
| Physician experience and formal-design foundation | [ENG-185](https://linear.app/coda-health/issue/ENG-185) | [Physician site](physician-site.md), <a href="/verification/README%2Emd">verification</a>; design proof is separate from runtime and clinical qualification |

## Retained references

The original repositories remain read-only comparison material. Their pinned
contracts and release packets explain prior behavior and evidence; they cannot
override current Linear scope. Follow a ticket's specific source links when
porting a behavior. Do not recreate a chapter for every historical heading.

- [Original Duet documentation](https://github.com/Coda-Health/coda-app/tree/5a1dc47b8feacb8db3fcc71522982f76822c0b01/docs): behavior, clinical catalogs and licenses, privacy, historical iOS/TestFlight release packets, scenario and insight research.
- [Original physician documentation](https://github.com/Coda-Health/coda-agents/tree/5674d0f13895214e0a633fc81176107f2e28f79d/docs): architecture, runtime protocol, physician experience, care-team operations and readiness.
- [October 21 Rust design review](https://linear.app/coda-health/document/duet-rust-review-input-for-tylers-design-pass-a6562ebd1c45): dated observations, not a current implementation inventory or acceptance contract.
- <a href="/submodules/coda-gym/README%2Emd">Coda Gym</a> and <a href="/submodules/Coda-Computer-Use/README%2Emd">computer use</a> retain their own task packages, source evidence and operating instructions; neither has a Rust product crate here.

Fixtures, generated API contracts, published legal text and active verification
work are not obsolete merely because product scope changed. Remove a runtime
surface through its outcome with consumer and authority checks, not through a
documentation cleanup.
