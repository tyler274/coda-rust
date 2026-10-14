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
      # a newer one. Swift itself wants libxml2.so.2, which the Swiftly
      # toolchain wrapper supplies.
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
          if [ -f "$HOME/.local/share/swiftly/env.sh" ]; then
            . "$HOME/.local/share/swiftly/env.sh"
          fi
          export LD_LIBRARY_PATH="${swiftLibs}:${wheelLibs}''${LD_LIBRARY_PATH:+:}$LD_LIBRARY_PATH"
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
        '';
      };
    };
}
