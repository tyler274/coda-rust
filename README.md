# coda-rust

This is the monorepo for Coda's Rust workspace. It holds the shared Duet client, the physician Dioxus client, and the crates they use. The product repositories are checked out here as submodules. Their READMEs own product behavior, setup, and proof. React and SwiftUI remain the shipping clients.

## Layout

- `crates/` is one Cargo workspace. Duet is `duet-app` and `duet-api`. The physician page is `physician-app`. `physician-sample` serves that page and a recorded projection of the synthetic practice, and it does not run a model. `ui` and `http` are the only crates the two products share.
- `platform/` holds the iPhone and Android bindings for the shared Duet client.
- `skills/` is the clinical skill catalog, loaded by `harness`.
- `book/` is the navigation book over the product documents. Build with `mdbook build book`, serve this repository with `python3 -m http.server` from `nix develop`, and open `/target/book/reading.html`. `mdbook serve` cannot see `/crates` or `/submodules`.
- `submodules/` checks out the product repositories. Read each README there for that product: [coda-app](submodules/coda-app/README.md), [coda-agents](submodules/coda-agents/README.md), [coda-gym](submodules/coda-gym/README.md), and [Coda-Computer-Use](submodules/Coda-Computer-Use/README.md).

Where a change belongs, and how the dev shell, Doppler, and CI fit, is [AGENTS.md](AGENTS.md).

## Start

```sh
git clone --recurse-submodules git@github.com:Coda-Health/coda-rust.git
cd coda-rust
nix develop
cargo test --workspace
```

`nix develop` is the shell for this checkout. Tests and the sample server do not need a Doppler account.
