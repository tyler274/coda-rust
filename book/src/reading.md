# How to use this book

This book is navigation. Each page points at the file that owns a fact: a crate's entry file in this workspace, or the canonical document in a product repository. It is not a roadmap, and it does not copy contracts. [Agent contracts, hoisted](agent-contracts.md) is the one deliberate exception, and <a href="/AGENTS%2Emd"><code>AGENTS.md</code></a> routes to it. When a page here disagrees with its source, the source wins and the page needs fixing.

The book has four parts:

| Part | Use it to find |
| --- | --- |
| **Setup** | How to clone, enter the shell, run tests, and where each kind of change goes: [Set up the workspace](setup.md) |
| **Product behavior** | What Duet and the physician product must do, from their product repositories |
| **Ownership** | Which crate, script, or repository implements each part: start at the [crate map](crate-map.md) |
| **Release** | CI, Doppler, the GCP roots, TestFlight, and Android: start at [Build, CI, and release](release.md) |

## Authority

Product direction lives in [Linear](https://linear.app/coda-health/team/ENG). The ticket for a piece of work carries Dev's direction. coda-app added this to the authority order in its `docs/README.md` in [5a1dc47b](https://github.com/Coda-Health/coda-app/commit/5a1dc47b8feacb8db3fcc71522982f76822c0b01), the same change that retired the formal model. The coda-app commit this workspace pins is older than that change. When a ticket conflicts with a product document, follow the ticket and amend the document in the same change. Workspace routing stays in <a href="/AGENTS%2Emd"><code>AGENTS.md</code></a>.

## Product behavior: Duet

Duet's reading order is <a href="/submodules/coda-app/docs/README%2Emd">Duet documentation (<code>docs/README.md</code>)</a>. The chapters follow its sections:

- <a href="/submodules/coda-app/docs/README%2Emd#authority-order">Authority order</a>: [North Star](north-star.md), [Behaviour contract](behaviour-contract.md), [Backend architecture](backend-architecture.md), and [iOS verification](ios-verification.md). The [formal model](formal-model.md) is retired upstream.
- <a href="/submodules/coda-app/docs/README%2Emd#current-architecture-and-proof">Current architecture and proof</a>: [UI surface map](ui-surface-map.md), [Personal-agent program](personal-agent-program.md), [Privacy recovery](privacy-recovery.md), [Security posture](security-posture.md), [Clinical catalog](clinical-catalog-migration.md), [Terminology licenses](clinical-terminology-license-audit.md), [Medication and vaccines](medication-vaccine-catalog-program.md), [Sharing threat model](sharing-threat-model.md), [Sharing privacy disclosure](sharing-privacy-disclosure.md), and [Insight visual contract](visual-contract.md).
- <a href="/submodules/coda-app/docs/README%2Emd#release-packets-and-retained-proof">Release packets and retained proof</a>: listed under **Release**.
- <a href="/submodules/coda-app/docs/README%2Emd#product-research-and-calibration">Product research and calibration</a>: [Scenario lab](scenario-lab.md) and [Insight lab](insight-lab.md).
- <a href="/submodules/coda-app/docs/README%2Emd#change-discipline">Change discipline</a> stays in coda-app.

## Product behavior: physician product

coda-agents owns the physician product. Its <a href="/submodules/coda-agents/README%2Emd#document-map">document map</a> answers "which document owns this?". The main owners are the <a href="/submodules/coda-agents/docs/architecture%2Emd">architecture</a>, <a href="/submodules/coda-agents/docs/physician-experience%2Emd">physician experience</a>, the <a href="/submodules/coda-agents/docs/ai-care-team-operating-contract%2Emd">care-team operating contract</a>, the <a href="/submodules/coda-agents/docs/runtime-protocol%2Emd">runtime protocol</a>, and <a href="/submodules/coda-agents/docs/readiness%2Emd">readiness</a>. [Physician site](physician-site.md) maps those documents to the Rust crates.

## The other checkouts

Gym and computer use have no Rust crate yet. [Coda Gym](coda-gym.md) and [Computer use](computer-use.md) link their repositories.
