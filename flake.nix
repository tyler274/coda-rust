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
      ];
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
          # FHS-wrapped Cursor so extensions work without nix-specific patches.
          # Keep this ahead of the host profile on PATH: the host's older
          # 3.19.13 build SIGTRAPs under the dev shell's LD_LIBRARY_PATH.
          code-cursor-fhs
        ];

        env.OPENSSL_NO_VENDOR = "1";
        env.PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

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
          export LD_LIBRARY_PATH="${swiftLibs}:${wheelLibs}''${LD_LIBRARY_PATH:+:}$LD_LIBRARY_PATH"
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
          export CODA_PATIENT_ENCODER_PYTHON="$PWD/submodules/coda-agents/.venv/bin/python3"
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
