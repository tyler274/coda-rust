{
  description = "Development shell for the Coda monorepo";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        config.allowUnfree = true;
      };
      postgres = pkgs.postgresql_16.withPackages (ps: [ ps.pgvector ]);
      # Manylinux wheels (NumPy, Torch, Playwright) link glibc-style libraries
      # that are not on NixOS's default loader path.
      wheelLibs = pkgs.lib.makeLibraryPath [
        pkgs.stdenv.cc.cc.lib
        pkgs.libgcc.lib
        pkgs.zlib
        pkgs.libz
        pkgs.stdenv.cc.cc
      ];
      # Swift.org's Linux toolchain is a glibc binary. curl stays off this
      # path: the xtool AppImage ships its own OpenSSL, and Nix curl demands
      # a newer one.
      swiftLibs = pkgs.lib.makeLibraryPath [
        pkgs.ncurses
        pkgs.sqlite
        pkgs.icu
        pkgs.xz
        pkgs.bzip2
        pkgs.libedit
        pkgs.gmp
        pkgs.e2fsprogs
        pkgs.krb5
        pkgs.keyutils
        # The toolchain lld loads libxml2; without it a rustc wasm link
        # under a stripped-down environment aborts at exit 127. This nixpkgs
        # pin's current libxml2 is 2.15, whose soname no longer matches the
        # binary, so pin the 2.13 ABI the toolchain links against.
        pkgs.libxml2_13
      ];
      # Chromium's own NSS build pairs with its own sqlite; the shell's
      # sqlite (newer glibc) must not shadow it when Chromium loads. The
      # shellHook resolves the exact store paths by reading chromium's
      # dynamic links at activation time.
      chromiumBin = "${pkgs.chromium}/bin/chromium";
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          rustc
          cargo
          rustfmt
          clippy
          cargo-deny
          lld
          pkg-config
          openssl
          dioxus-cli
          openapi-generator-cli
          nodejs_22
          python314
          uv
          poppler-utils
          postgres
          gcc
          gnumake
          jq
          gnupg
          unzip
          zip
          doppler
          google-cloud-sdk
          act
          actionlint
          mdbook
          opentofu
          wasm-bindgen-cli_0_2_129
          # The proof tests drive this through CDP; CHROMIUM points at it.
          chromium
          # FHS-wrapped Cursor so extensions work without nix-specific patches.
          # Keep this ahead of the host profile on PATH: the host's older
          # 3.19.13 build SIGTRAPs under the dev shell's LD_LIBRARY_PATH.
          code-cursor-fhs
        ];

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
          cat > "$coda_bin/cursor" <<CURSOR
#!/bin/sh
exec env -u LD_LIBRARY_PATH "${pkgs.code-cursor-fhs}/bin/cursor" "\$@"
CURSOR
          chmod +x "$coda_bin/cursor"
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
          if doppler me >/dev/null 2>&1; then
            while IFS= read -r line; do
              [ -n "$line" ] && export "$line"
            done < <(doppler secrets download --no-file --format env-no-quotes --project coda-rust --config dev 2>/dev/null || true)
          fi
          # Git hooks live in .githooks (pre-commit runs rustfmt + clippy).
          # Idempotent: only set when not already configured, so a local
          # override survives.
          if [ "$(git rev-parse --git-dir 2>/dev/null)" ]; then
            git config core.hooksPath .githooks
          fi
        '';
      };
    };
}
