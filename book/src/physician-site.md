# The physician site and the Coda Agents harness

The shipping physician client is the React site in `submodules/coda-agents/web`. The Rust page is the Dioxus client in [`physician-app`](/crates/physician-app/src/lib.rs), mounting into `#main`, speaking the same projections through [`physician-wire`](/crates/physician-wire). [`physician-sample`](/crates/physician-sample/src/lib.rs) serves the page and a recorded projection of the synthetic practice, and forwards physician routes to the Node server through `PHYSICIAN_UPSTREAM`. Model execution is owned by [`local-product`](/crates/local-product/src/lib.rs); `api` mode is opt-in and never runs without an authorized key. [`harness`](/crates/harness/src/lib.rs) loads the clinical skill catalog in [`skills/`](/skills) once through [`agent-skills`](/crates/agent-skills/src/lib.rs), the [`encoder`](/crates/encoder/src/lib.rs) runs the offline patient-text inference, and [`patient-work`](/crates/patient-work/src/lib.rs) models the dispatcher's grouping window.

- <a href="/submodules/coda-agents/docs/architecture%2Emd#system-boundary">Architecture: the system boundary and model roles</a>
- <a href="/submodules/coda-agents/docs/architecture%2Emd#external-clinical-evidence">Architecture: external clinical evidence</a>
- <a href="/submodules/coda-agents/docs/runtime-protocol%2Emd">Runtime protocol</a>
- <a href="/submodules/coda-agents/docs/physician-experience%2Emd#physician-product-direction">Physician experience</a>
- <a href="/submodules/coda-agents/docs/ai-care-team-operating-contract%2Emd">Care-team operating contract</a>
- <a href="/submodules/coda-agents/docs/patient-messaging%2Emd">Messaging contract</a>
- <a href="/submodules/coda-agents/docs/readiness%2Emd#current-local-product-work">Readiness: current work</a>
- <a href="/submodules/coda-agents/docs/testing%2Emd#proof-by-boundary">Testing: proof by boundary</a>
- <a href="/submodules/coda-agents/README%2Emd#run-and-verify">Run and verify</a>
