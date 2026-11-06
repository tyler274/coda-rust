# The CI Postgres job's shell: scripts/check-sql.py and scripts/check-postgres.sh.
# The agent shell also carries Chromium, Codex, Claude Code, rust-docs and
# other tools these steps never run; fetching them cost most of the job's shell
# setup. Browser and wasm journeys in these suites are #[ignore]d here, so lld
# (550 MiB of LLVM, used only for wasm links) is omitted too: native links use
# BFD through the rustflags below.
#
# The compiler, rustflags and native-library variables match the agent shell
# so a target/ directory built in either shell stays fresh in the other.
{ pkgs, postgres, rustToolchain }:
let
  # Same channel and component store paths as rust-toolchain.toml's default
  # profile, without docs, the wasm32 std, rust-src, clippy or rustfmt.
  pin = (pkgs.lib.importTOML ../rust-toolchain.toml).toolchain;
  rust = pkgs.rust-bin.fromRustupToolchain { inherit (pin) channel; profile = "minimal"; };
  # crates/pubsub/src/testing.rs starts the emulator with `gcloud beta`.
  gcloud = (pkgs.google-cloud-sdk.withExtraComponents [ pkgs.google-cloud-sdk.components.pubsub-emulator ]).overrideAttrs (old: {
    buildCommand = ''
      export CLOUDSDK_CONFIG="$TMPDIR/gcloud"
      ${old.buildCommand}
    '';
  });
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
    gcloud
    jre_headless
    valkey
  ];
  env.NIX_CFLAGS_LINK = "";
  env.CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS = "-C link-arg=-fuse-ld=bfd";
  env.OPENSSL_NO_VENDOR = "1";
  env.PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  shellHook = ''
    export C_INCLUDE_PATH="${pkgs.glibc.dev}/include''${C_INCLUDE_PATH:+:}$C_INCLUDE_PATH"
    export LIBRARY_PATH="${pkgs.glibc}/lib''${LIBRARY_PATH:+:}$LIBRARY_PATH"
  '';
}
