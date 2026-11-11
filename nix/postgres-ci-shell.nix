# The CI Postgres job's shell: scripts/check-sql.py and scripts/check-postgres.sh.
# The agent shell also carries Chromium, Codex, Claude Code, rust-docs and
# other tools these steps never run; fetching them cost most of the job's shell
# setup. Browser and wasm journeys in these suites are #[ignore]d here, so lld
# (550 MiB of LLVM, used only for wasm links) is omitted too: native links use
# BFD through the shared rustflags.
#
# The compiler, rustflags and native-library variables come from the same
# definitions as the agent shell. Build scripts that fingerprint other
# variables (aws-lc-sys tracks PKG_CONFIG_PATH, which package setup hooks
# extend) still rebuild once in a new package set, so the Postgres job keys
# its target/ cache on this file and shell-common.nix.
{ pkgs, postgres, rustToolchain }:
let
  common = import ./shell-common.nix { inherit pkgs; };
  # Same channel and component store paths as rust-toolchain.toml's default
  # profile, without docs, the wasm32 std, rust-src, clippy or rustfmt.
  pin = (pkgs.lib.importTOML ../rust-toolchain.toml).toolchain;
  rust = pkgs.rust-bin.fromRustupToolchain { inherit (pin) channel; profile = "minimal"; };
in
assert rust.version == rustToolchain.version;
pkgs.mkShell {
  packages = with pkgs; [
    rust
    pkg-config
    openssl
    python314
    postgres
    pgformatter
    common.gcloud
    jre_headless
    valkey
  ];
  inherit (common) env;
  shellHook = common.glibcHook;
}
