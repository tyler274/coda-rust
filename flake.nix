{
  description = "Development shell for the Coda monorepo";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  inputs.crane.url = "github:ipetkov/crane";

  outputs =
    { self, nixpkgs, rust-overlay, crane }:
    let
      system = "x86_64-linux";
      # aarch64-linux adds only the dev shells, for Linux containers on Apple
      # silicon. Release packages, images and CI stay on x86_64-linux.
      devSystems = [ system "aarch64-linux" ];
      pkgsFor = system: import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
        config.allowUnfree = true;
      };
      devShellsFor = system:
        let
          pkgs = pkgsFor system;
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
          # The same pin as the packages and the CI `ci` job, plus the std
          # source and a rust-analyzer of the same release, which editors
          # find on the shell's PATH through direnv.
          rustToolchain = (import ./nix/rust-toolchain.nix pkgs).override {
            extensions = [
              "rust-src"
              "rust-analyzer"
            ];
          };
        in
        {
          # No credential-loading shellHook in release builds or CI packaging.
          release = pkgs.mkShell {
            packages = with pkgs; [ nix git gh jq python3 docker-client skopeo actionlint cargo-deny postgres opentofu ];
          };
          # The CI observability job: collector config validation and the
          # OpenTofu roots it rides on, without the agent shell's toolchain.
          observability = pkgs.mkShell {
            packages = with pkgs; [ opentelemetry-collector-contrib opentofu ];
          };
          default = import ./nix/dev-shell.nix {
            inherit pkgs postgres wheelLibs chromiumBin rustToolchain;
          };
          agent = import ./nix/dev-shell.nix {
            inherit pkgs postgres wheelLibs chromiumBin rustToolchain;
            desktop = false;
            loadCredentials = false;
          };
        } // pkgs.lib.optionalAttrs (system == "x86_64-linux") {
          # The x86_64 emulator image needs KVM on an x86_64 host. Accepting
          # the SDK licence stays inside this shell's package set.
          android = import ./nix/android-shell.nix {
            inherit rustToolchain;
            pkgs = import nixpkgs {
              inherit system;
              overlays = [ rust-overlay.overlays.default ];
              config = { allowUnfree = true; android_sdk.accept_license = true; };
            };
          };
        };
    in
    {
      packages.${system} = import ./nix/packages.nix { pkgs = pkgsFor system; inherit self crane; };
      devShells = nixpkgs.lib.genAttrs devSystems devShellsFor;
    };
}
