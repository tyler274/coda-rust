# Duet behaviour model (TLA+)

> **Retired upstream.** coda-app removed `formal/` and the backend trace check in [5a1dc47b](https://github.com/Coda-Health/coda-app/commit/5a1dc47b8feacb8db3fcc71522982f76822c0b01), "Retire the formal model and trace check; the rules page stays". The [behaviour contract](behaviour-contract.md) is still the rules page. The same change made coda-app's authority order name [Linear](https://linear.app/coda-health/team/ENG) as the home of product direction (see [How to use this book](reading.md#authority)). Do not start new work from this model.

The links below resolve only while this workspace pins coda-app at a commit before the retirement. When the submodule moves past 5a1dc47b, the book link check fails on them, and this page should be removed in the same change.

The model checked the behaviour contract against the Python backend. No Rust crate ran that trace.

- <a href="/submodules/coda-app/formal/README%2Emd#duet-behaviour-model-tla">Duet behaviour model (TLA+)</a>
- <a href="/submodules/coda-app/formal/README%2Emd#1-what-the-model-covers">1. What the model covers</a>
- <a href="/submodules/coda-app/formal/README%2Emd#actors">Actors</a>
- <a href="/submodules/coda-app/formal/README%2Emd#the-state-in-product-words">The state, in product words</a>
- <a href="/submodules/coda-app/formal/README%2Emd#what-is-abstracted-away">What is abstracted away</a>
- <a href="/submodules/coda-app/formal/README%2Emd#2-results-one-row-per-rule">2. Results, one row per rule</a>
- <a href="/submodules/coda-app/formal/README%2Emd#3-assumptions">3. Assumptions</a>
- <a href="/submodules/coda-app/formal/README%2Emd#4-counterexamples-none-remain">4. Counterexamples: none remain</a>
- <a href="/submodules/coda-app/formal/README%2Emd#4a-the-design-fix-from-the-first-revision-kept">4a. The design fix from the first revision (kept)</a>
- <a href="/submodules/coda-app/formal/README%2Emd#5-fairness-assumptions-all-of-them">5. Fairness assumptions (all of them)</a>
- <a href="/submodules/coda-app/formal/README%2Emd#6-how-the-checker-was-run">6. How the checker was run</a>
- <a href="/submodules/coda-app/formal/README%2Emd#7-what-a-code-owner-must-provide">7. What a code owner must provide</a>
- <a href="/submodules/coda-app/formal/README%2Emd#files">Files</a>
