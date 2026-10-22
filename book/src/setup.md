# Set up the workspace

This page points at the files that own setup. It does not repeat them.

## Clone, shell, and first test

- <a href="/README%2Emd#start"><code>README.md</code>: Start</a> covers the clone with submodules, `nix develop`, and `cargo test --workspace`.
- <a href="/README%2Emd#layout"><code>README.md</code>: Layout</a> says what `crates/`, `platform/`, `skills/`, `book/`, and `submodules/` hold. The [crate map](crate-map.md) cites each crate's entry file.
- <a href="/README%2Emd#physician-chat-codex-cli"><code>README.md</code>: Physician chat</a> covers `codex login`, `cargo run -p physician-sample`, and the record and replay cassettes.
- <a href="/README%2Emd#dev-container"><code>README.md</code>: Dev container</a> covers the VS Code container, its Postgres on port 5433, and its storage.

## Where work goes and how the shell is configured

- <a href="/AGENTS%2Emd#where-work-goes"><code>AGENTS.md</code>: Where work goes</a> routes each kind of change to the contract that owns it. [Agent contracts, hoisted](agent-contracts.md) restates the operative rules of the four product repositories.
- <a href="/AGENTS%2Emd#dev-shell"><code>AGENTS.md</code>: Dev shell</a> covers Doppler (`dev`, `stg`, `prd`), `act`, `actionlint`, and the boundary on applying `infra/gcp`.
- [`flake.nix`](/flake.nix) is the toolchain owner. [`.envrc`](/.envrc) enters it through direnv. [`doppler.yaml`](/doppler.yaml) selects the local Doppler config.
- [`.githooks/pre-commit`](/.githooks/pre-commit) runs rustfmt and clippy. The shell hook installs it.
- Workspace rules for agents live in `.cursor/rules`: <a href="/.cursor/rules/rust-repo-first%2Emdc"><code>rust-repo-first.mdc</code></a> (submodules are read-only), <a href="/.cursor/rules/nix-dev-shell%2Emdc"><code>nix-dev-shell.mdc</code></a>, and <a href="/.cursor/rules/worktree-per-plan%2Emdc"><code>worktree-per-plan.mdc</code></a>.

## Product repositories

Each product repository owns its own setup and proof. This workspace reads them and does not edit them.

| Repository | Setup and verification |
| --- | --- |
| coda-app (Duet) | <a href="/submodules/coda-app/README%2Emd#local-verification">Local verification</a>, <a href="/submodules/coda-app/README%2Emd#canonical-owners">Canonical owners</a> |
| coda-agents (physician product) | <a href="/submodules/coda-agents/README%2Emd#run-and-verify">Run and verify</a>, <a href="/submodules/coda-agents/README%2Emd#document-map">Document map</a> |
| coda-gym | <a href="/submodules/coda-gym/README%2Emd#local-verification">Local Verification</a> |
| Coda-Computer-Use | <a href="/submodules/Coda-Computer-Use/README%2Emd#run">Run</a> |

Product direction lives in [Linear](https://linear.app/coda-health/team/ENG). Where a ticket and a document disagree, the ticket carries Dev's direction (see [How to use this book](reading.md#authority)).

## Reading and checking this book

```sh
nix develop
mdbook build book                 # writes target/book
python3 -m http.server            # from the repository root; open /target/book/reading.html
scripts/check-book-links.sh       # builds the book, then checks every link offline
```

`mdbook serve` cannot resolve `/crates` or `/submodules`, so serve the repository root instead. [`scripts/check-book-links.sh`](/scripts/check-book-links.sh) resolves absolute links against the repository root, and it checks `#fragment` links against headings. Links into a submodule that is not checked out are skipped with a notice.
