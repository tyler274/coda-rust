# Shared repository toolchain for interactive and unattended development.
# The unattended profile omits desktop tooling and Doppler credential loading.
{ pkgs, postgres, wheelLibs, chromiumBin, rustToolchain, desktop ? true, loadCredentials ? true }:
pkgs.mkShell {
  packages = with pkgs; [
    # rustc, cargo, rustfmt, clippy and the wasm32 std from rust-toolchain.toml.
    rustToolchain
    cargo-deny
    lld
    pkg-config
    openssl
    dioxus-cli
    openapi-generator-cli
    nodejs_22
    # Model-backed browser checks use the statechart skill's pinned TLC and Java.
    mise
    python314
    uv
    poppler-utils
    postgres
    pgformatter
    gcc
    gnumake
    jq
    gnupg
    unzip
    zip
    doppler
    # Local physician chat and the local-review and review-pr reviewers run
    # these. Each developer signs in once: `codex login`, `claude auth login`.
    codex
    claude-code
    # The Pub/Sub emulator backs the physician chat-turn queue tests
    # (crates/physician-sample/tests/worker.rs). It is a Java server.
    # kubectl reaches the GKE observability cluster through
    # gke-gcloud-auth-plugin, which `gcloud container clusters
    # get-credentials` writes into the kubeconfig.
    ((google-cloud-sdk.withExtraComponents (with google-cloud-sdk.components; [
      pubsub-emulator
      kubectl
      gke-gcloud-auth-plugin
    ])).overrideAttrs (old: {
      # The component check writes gcloud configuration. Keep it in the build
      # directory so root builds without sandboxing cannot create /homeless-shelter.
      buildCommand = ''
        export CLOUDSDK_CONFIG="$TMPDIR/gcloud"
        ${old.buildCommand}
      '';
    }))
    jre_headless
    # Redis-compatible server for the physician event bus tests
    # (crates/physician-bus/tests, physician-sample/tests/fanout.rs).
    valkey
    act
    actionlint
    oha
    mdbook
    # Book link check (scripts/check-book-links.sh). CI installs the
    # same release from GitHub; keep the two versions together.
    lychee
    opentofu
    # `otelcol-contrib validate` checks dev/observability/collector.yaml,
    # which the compose stack and infra/gcp/observability both run.
    opentelemetry-collector-contrib
    wasm-bindgen-cli_0_2_129
    # The proof tests drive this through CDP; CHROMIUM points at it.
    chromium
    # FHS-wrapped Cursor so extensions work without nix-specific patches.
    # Keep this ahead of the host profile on PATH: the host's older
    # 3.19.13 build SIGTRAPs under the dev shell's LD_LIBRARY_PATH.
  ] ++ pkgs.lib.optionals desktop [ pkgs.code-cursor-fhs ];

  # Do not inherit host linker search paths (notably ElyLD's -B prefix).
  # ElyLD placed httparse's AVX constant at a 16-byte rather than 32-byte
  # boundary, faulting on vmovaps in the physician HTTP proxy unit test.
  env.NIX_CFLAGS_LINK = "";
  # Select the existing stdenv BFD linker for native Rust links. Cargo
  # fingerprints this target-specific flag, unlike NIX_CFLAGS_LINK, so
  # entering the repaired shell also replaces cached mislinked binaries.
  # Do not send GCC linker flags to wasm32 or other cross targets.
  env.CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS = "-C link-arg=-fuse-ld=bfd";
  # rust-analyzer loads std from the toolchain's rust-src component.
  env.RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
  env.OPENSSL_NO_VENDOR = "1";
  env.PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  # The browser proof tests (crates/mobile/tests, physician-sample/tests)
  # look this name up when CHROMIUM is unset.
  env.CHROMIUM = "${pkgs.chromium}/bin/chromium";

  shellHook = ''
    export COREPACK_HOME="$HOME/.cache/coda-dev/corepack"
    export COREPACK_ENABLE_DOWNLOAD_PROMPT=0
    coda_bin="$HOME/.cache/coda-dev/bin"
    mkdir -p "$COREPACK_HOME" "$coda_bin"
    export PATH="$coda_bin:$PATH"
    # The dev shell's LD_LIBRARY_PATH (Swift and wheel libs, including
    # Nix's sqlite) breaks Electron apps that pick those libraries up:
    # host-profile Cursor 3.19.13 SIGTRAPs in sqlite within a minute.
    # Strip the loader path for Cursor only; the shell keeps it for
    # Swift and the Python wheels. $coda_bin is first on PATH, so the
    # shim also wins over any older host-profile cursor.
    ${pkgs.lib.optionalString desktop ''
    cat > "$coda_bin/cursor" <<CURSOR
#!/bin/sh
exec env -u LD_LIBRARY_PATH "${pkgs.code-cursor-fhs}/bin/cursor" "\$@"
CURSOR
    chmod +x "$coda_bin/cursor"
    ''}
    if [ -f "$HOME/.local/share/swiftly/env.sh" ]; then
      . "$HOME/.local/share/swiftly/env.sh"
    fi
    # env.sh puts the swiftly shims first on PATH. rustc's wasm32
    # target has no bundled rust-lld and resolves `lld` on PATH, so
    # the shim routes it to the Swift toolchain's lld, which fails
    # without libxml2.so.2 when the environment is cleared (the
    # mobile web test re-invokes cargo that way). Keep $coda_bin on
    # top and link the nix lld into it.
    export PATH="$coda_bin:$PATH"
    for coda_lld_name in lld ld.lld wasm-ld; do
      coda_lld="$coda_bin/$coda_lld_name"
      if [ ! -e "$coda_lld" ] || [ "$(readlink -f "$coda_lld")" != "${pkgs.lld}/bin/$coda_lld_name" ]; then
        ln -sf "${pkgs.lld}/bin/$coda_lld_name" "$coda_lld"
      fi
    done
    export LD_LIBRARY_PATH="${wheelLibs}''${LD_LIBRARY_PATH:+:}$LD_LIBRARY_PATH"
    # Swift toolchain binaries (lld, clang) need libxml2.so.2, which
    # swiftly's env.sh does not put on the loader path. Append at the
    # tail so it cannot shadow the Chromium NSS/sqlite pair resolved
    # below.
    export LD_LIBRARY_PATH="''${LD_LIBRARY_PATH:+$LD_LIBRARY_PATH:}${pkgs.libxml2.out}/lib"
    # Headless Chromium aborts at startup when the NSS it loads cannot
    # resolve its sqlite (a glibc-mismatched copy would also break).
    # Resolve the exact nss + sqlite pair the binary links against and
    # put them ahead of everything else on the library path.
    coda_chromium_real="$(sed -n 's/^exec "\([^"]*\)".*/\1/p' ${chromiumBin})"
    coda_nss_dir="$(ldd "$coda_chromium_real" 2>/dev/null | grep 'libnss3\.so' | awk '{print $3}' | head -1 | xargs dirname | xargs dirname)/lib"
    if [ -d "$coda_nss_dir" ]; then
      coda_sqlite_dir="$(ldd "$coda_nss_dir/libsoftokn3.so" 2>/dev/null | grep 'libsqlite3\.so' | awk '{print $3}' | head -1 | xargs dirname)"
      export LD_LIBRARY_PATH="$coda_sqlite_dir:$coda_nss_dir:$LD_LIBRARY_PATH"
    fi
    # The browser tests look for CHROMIUM, then PATH. Chromium is in
    # the closure for the NSS resolution above; export it so the
    # tests do not depend on it also being on PATH.
    if [ -z "''${CHROMIUM:-}" ]; then
      export CHROMIUM="${chromiumBin}"
    fi
    export C_INCLUDE_PATH="${pkgs.glibc.dev}/include''${C_INCLUDE_PATH:+:}$C_INCLUDE_PATH"
    export LIBRARY_PATH="${pkgs.glibc}/lib''${LIBRARY_PATH:+:}$LIBRARY_PATH"
    # Nix's PostgreSQL puts its Unix socket in /run/postgresql, which a
    # non-root user cannot write on NixOS or a stock Linux host. The test
    # suite's harness starts its own clusters with pg_ctl and passes no
    # socket directory, so when the default is unwritable, point
    # CODA_TEST_POSTGRES_BIN at a wrapper whose pg_ctl adds -k. The
    # snapshot cache hashes `postgres --version` from this directory, so
    # pass initdb and postgres through unchanged.
    coda_pg_bin="$HOME/.cache/coda-dev/pg16-bin"
    coda_pg_socks="''${XDG_RUNTIME_DIR:-/tmp}/coda-pg-sockets"
    if ! [ -w /run/postgresql ]; then
      mkdir -p "$coda_pg_bin" "$coda_pg_socks"
      ln -sf "${postgres}/bin/initdb" "$coda_pg_bin/initdb"
      ln -sf "${postgres}/bin/postgres" "$coda_pg_bin/postgres"
      cat > "$coda_pg_bin/pg_ctl" <<PGCTL
#!/bin/sh
exec ${postgres}/bin/pg_ctl "\$@" -o "-k $coda_pg_socks"
PGCTL
      chmod +x "$coda_pg_bin/pg_ctl"
      export CODA_TEST_POSTGRES_BIN="$coda_pg_bin"
    else
      export CODA_TEST_POSTGRES_BIN="${postgres}/bin"
    fi
    export CODA_PATIENT_ENCODER_MODEL="$HOME/code/coda/coda-local-data/models/bge-small-en-v1.5"
    # The BGE venv is provisioned outside the submodule (it is
    # gitignored there), so prefer the submodule copy and fall back to
    # the sibling checkout a worktree shares the monorepo with.
    if [ -x "$PWD/submodules/coda-agents/.venv/bin/python3" ]; then
      export CODA_PATIENT_ENCODER_PYTHON="$PWD/submodules/coda-agents/.venv/bin/python3"
    elif [ -x "$HOME/code/coda/coda-agents/.venv/bin/python3" ]; then
      export CODA_PATIENT_ENCODER_PYTHON="$HOME/code/coda/coda-agents/.venv/bin/python3"
    else
      export CODA_PATIENT_ENCODER_PYTHON="python3"
    fi
    export CODA_PATIENT_ENCODER_BIN="$PWD/target/debug/encoder"
    export DUET_DISCOVERY_ENCODER_MODEL="$CODA_PATIENT_ENCODER_MODEL"
    export DUET_DISCOVERY_ENCODER_BIN="$CODA_PATIENT_ENCODER_BIN"
    if ! "$coda_bin/pnpm" --version 2>/dev/null | grep -qx "10.33.0"; then
      corepack enable --install-directory "$coda_bin"
      corepack prepare pnpm@10.33.0 --activate
    fi
    ${pkgs.lib.optionalString loadCredentials ''
    if doppler me >/dev/null 2>&1; then
      while IFS= read -r line; do
        [ -n "$line" ] && export "$line"
      done < <(doppler secrets download --no-file --format env-no-quotes --project coda-rust --config dev 2>/dev/null || true)
    fi
    ''}
    # A host ElyLD override can survive `nix develop` in this wrapper
    # variable. ElyLD 1.0.0 misaligns AVX2 constants in unoptimized Rust
    # binaries (CRC32 and ChaCha both fault). Remove only that foreign
    # linker-prefix override, including GCC's joined/separate -B forms.
    # Keep all other wrapper flags and NIX_LDFLAGS: Nix uses those for
    # library paths, hardening and runtime search paths. Do this after
    # sourcing host tooling and Doppler so they cannot reintroduce it.
    coda_foreign_linker='(^|[[:space:]])-B[[:space:]]*/nix/store/[^/[:space:]]+-elyld-ld/ld-prefix/?([[:space:]]|$)'
    while [[ "''${NIX_CFLAGS_LINK-}" =~ $coda_foreign_linker ]]; do
      NIX_CFLAGS_LINK="''${NIX_CFLAGS_LINK/"''${BASH_REMATCH[0]}"/ }"
    done
    if [[ -v NIX_CFLAGS_LINK ]]; then
      export NIX_CFLAGS_LINK
    fi
    unset coda_foreign_linker
    # Git hooks live in .githooks (pre-commit runs rustfmt + clippy).
    # Idempotent: only set when not already configured, so a local
    # override survives.
    if [ "$(git rev-parse --git-dir 2>/dev/null)" ] && ! git config --get core.hooksPath >/dev/null; then
      git config core.hooksPath .githooks
    fi
    # Set last, after every PATH and linker fix above, so a hook that
    # sees it knows it runs in the complete shell. IN_NIX_SHELL is not
    # enough: any nix shell, or a host swiftly PATH on top, sets it too.
    export CODA_DEV_SHELL=1
  '';
}
