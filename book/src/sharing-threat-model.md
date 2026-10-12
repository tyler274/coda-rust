# Duet Sharing V1 threat model

The Rust client reads sharing status through [`DuetClient`](/crates/duet-client/src/lib.rs) and renders it in `shared_root`. Keys, grants, and the threat model stay in the Duet sharing service.

- <a href="/submodules/coda-app/docs/sharing-threat-model%2Emd#duet-sharing-v1-threat-model">Duet Sharing V1 threat model</a>
- <a href="/submodules/coda-app/docs/sharing-threat-model%2Emd#security-goals">Security goals</a>
- <a href="/submodules/coda-app/docs/sharing-threat-model%2Emd#data-flow-and-trust-boundaries">Data flow and trust boundaries</a>
- <a href="/submodules/coda-app/docs/sharing-threat-model%2Emd#protected-assets">Protected assets</a>
- <a href="/submodules/coda-app/docs/sharing-threat-model%2Emd#cryptographic-construction">Cryptographic construction</a>
- <a href="/submodules/coda-app/docs/sharing-threat-model%2Emd#threats-and-controls">Threats and controls</a>
- <a href="/submodules/coda-app/docs/sharing-threat-model%2Emd#active-server-and-identity-limitation">Active-server and identity limitation</a>
- <a href="/submodules/coda-app/docs/sharing-threat-model%2Emd#key-loss-and-deletion">Key loss and deletion</a>
- <a href="/submodules/coda-app/docs/sharing-threat-model%2Emd#release-gates">Release gates</a>
