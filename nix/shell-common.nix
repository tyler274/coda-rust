# Definitions every repository shell shares, so their native Rust builds agree.
{ pkgs }:
{
  # The Pub/Sub emulator backs the physician chat-turn queue tests
  # (crates/physician-sample/tests/worker.rs). It is a Java server, and
  # crates/pubsub/src/testing.rs starts it with `gcloud beta`.
  gcloud = (pkgs.google-cloud-sdk.withExtraComponents [ pkgs.google-cloud-sdk.components.pubsub-emulator ]).overrideAttrs (old: {
    # The component check writes gcloud configuration. Keep it in the build
    # directory so root builds without sandboxing cannot create /homeless-shelter.
    buildCommand = ''
      export CLOUDSDK_CONFIG="$TMPDIR/gcloud"
      ${old.buildCommand}
    '';
  });

  env = {
    # Do not inherit host linker search paths (notably ElyLD's -B prefix).
    # ElyLD placed httparse's AVX constant at a 16-byte rather than 32-byte
    # boundary, faulting on vmovaps in the physician HTTP proxy unit test.
    NIX_CFLAGS_LINK = "";
    # Select the existing stdenv BFD linker for native Rust links. Cargo
    # fingerprints this target-specific flag, unlike NIX_CFLAGS_LINK, so
    # entering the repaired shell also replaces cached mislinked binaries.
    # Do not send GCC linker flags to wasm32 or other cross targets.
    CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS = "-C link-arg=-fuse-ld=bfd";
    OPENSSL_NO_VENDOR = "1";
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  };

  glibcHook = ''
    export C_INCLUDE_PATH="${pkgs.glibc.dev}/include''${C_INCLUDE_PATH:+:}$C_INCLUDE_PATH"
    export LIBRARY_PATH="${pkgs.glibc}/lib''${LIBRARY_PATH:+:}$LIBRARY_PATH"
  '';
}
