{
  description = "Development shell for the Coda monorepo";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

  outputs =
    { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
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
      packages.${system} = import ./nix/packages.nix { inherit pkgs self; };
      # No credential-loading shellHook in release builds or CI packaging.
      devShells.${system} = {
      release = pkgs.mkShell {
        packages = with pkgs; [ nix git gh jq python3 docker-client skopeo actionlint cargo-deny postgres opentofu ];
      };
      default = import ./nix/dev-shell.nix {
        inherit pkgs postgres wheelLibs chromiumBin;
      };
      agent = import ./nix/dev-shell.nix {
        inherit pkgs postgres wheelLibs chromiumBin;
        desktop = false;
        loadCredentials = false;
      };
      };
    };
}
