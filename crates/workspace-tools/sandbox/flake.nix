{
  description = "coda-sandbox: set up, check and run the Coda agent sandbox";

  # Its own flake on purpose: a host builds and deploys this tool without the
  # product flake's toolchain overlay, dev shells or packages.
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    # The CI's Firecracker guest.
    microvm = {
      url = "github:microvm-nix/microvm.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, crane, microvm }:
    let
      inherit (nixpkgs) lib;
      systems = [ "x86_64-linux" "aarch64-linux" ];
      forEachSystem = f: lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});

      # coda-sandbox and coda-swarm (the swarm supervisor the container runs)
      # are members of the repository's root Cargo workspace. Each build
      # takes the root manifest and lockfile but none of the other crates:
      # the manifest's members narrow to that crate, and the lockfile narrows
      # to the packages it reaches, so only its own dependencies are vendored.
      repo = ../../..;
      lock = lib.importTOML (repo + "/Cargo.lock");
      byName = lib.groupBy (package: package.name) lock.package;
      # A lockfile dependency is "name" when one version is locked, otherwise
      # "name version" or "name version (source)".
      locked = dependency:
        let
          parts = lib.splitString " " dependency;
          candidates = byName.${builtins.head parts};
        in
        if builtins.length parts == 1 then builtins.head candidates
        else lib.findFirst (package: package.version == builtins.elemAt parts 1)
          (throw "Cargo.lock has no ${dependency}") candidates;
      entry = package: { key = "${package.name} ${package.version}"; inherit package; };

      build = pkgs: { pname, member, fileset, nativeCheckInputs ? [ ] }:
        let
          craneLib = crane.mkLib pkgs;
          reached = builtins.genericClosure {
            startSet = [ (entry (locked pname)) ];
            operator = item: map (dependency: entry (locked dependency)) (item.package.dependencies or [ ]);
          };
          cargoLockParsed = lock // { package = map (item: item.package) reached; };
          manifest = lib.recursiveUpdate (lib.importTOML (repo + "/Cargo.toml")) {
            workspace.members = [ member ];
          };
          src = lib.fileset.toSource {
            root = repo;
            fileset = lib.fileset.unions ([ (repo + "/Cargo.toml") ] ++ fileset);
          };
          common = {
            inherit src cargoLockParsed pname;
            strictDeps = true;
            postPatch = "cp ${(pkgs.formats.toml { }).generate "Cargo.toml" manifest} Cargo.toml";
            # --offline, not --locked: Cargo compares lockfiles as text, and
            # the narrowed one is rewritten. Offline against a vendor
            # directory of exactly the locked packages, Cargo cannot resolve
            # anything the root Cargo.lock does not pin.
            cargoExtraArgs = "--offline -p ${pname}";
          };
          cargoArtifacts = craneLib.buildDepsOnly common;
        in
        {
          package = craneLib.buildPackage (common // {
            inherit cargoArtifacts;
            doCheck = false;
            meta.mainProgram = pname;
          });
          checks = {
            "${pname}-test" = craneLib.cargoTest (common // { inherit cargoArtifacts nativeCheckInputs; });
            "${pname}-clippy" = craneLib.cargoClippy (common // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- -D warnings";
            });
            "${pname}-fmt" = craneLib.cargoFmt (common // { cargoExtraArgs = "-p ${pname}"; });
          };
        };

      crates = pkgs: {
        # Tests read config/cyrene.toml, so it enters the source with the
        # crate. The integration tests drive git in scratch repositories.
        coda-sandbox = build pkgs {
          pname = "coda-sandbox";
          member = "crates/workspace-tools/sandbox";
          fileset = [ ./Cargo.toml ./src ./config ];
          nativeCheckInputs = [ pkgs.git ];
        };
        # The registry test reads models.example.json.
        coda-swarm = build pkgs {
          pname = "coda-swarm";
          member = "crates/workspace-tools/swarm";
          fileset = [ ../swarm/Cargo.toml ../swarm/src ../swarm/tests ../swarm/models.example.json ];
        };
      };

      sandboxModule = import ./nix {
        packages = self.packages;
        microvmModule = microvm.nixosModules.microvm;
      };

      # The VM tests need KVM (coda-firecracker nested KVM), so hosted CI
      # skips them; run them on the sandbox host.
      vmChecks = pkgs: {
        agent-sandbox = pkgs.testers.runNixOSTest (import ./nix/tests/agent-sandbox.nix { inherit sandboxModule; });
        coda-firecracker = pkgs.testers.runNixOSTest (import ./nix/tests/coda-firecracker.nix { inherit sandboxModule; });
      };

      carryCheck = pkgs:
        let carry = pkgs.callPackage ./nix/carry-package.nix { };
        in pkgs.runCommand "coda-carry-test" { nativeBuildInputs = [ pkgs.git ]; } ''
          bash ${./nix/tests/coda-carry.sh} ${carry}/bin/coda-carry
          touch $out
        '';
    in
    {
      packages = forEachSystem (pkgs:
        let built = lib.mapAttrs (_: crate: crate.package) (crates pkgs);
        in built // { default = built.coda-sandbox; });

      nixosModules.default = sandboxModule;

      checks = forEachSystem (pkgs:
        lib.concatMapAttrs (name: crate: crate.checks // { ${name} = crate.package; }) (crates pkgs)
        // { coda-carry = carryCheck pkgs; }
        // lib.optionalAttrs (pkgs.stdenv.hostPlatform.system == "x86_64-linux") (vmChecks pkgs));

      # The CI guest on its own, for building or inspecting it:
      #   nix build ./crates/workspace-tools/sandbox#nixosConfigurations.coda-ci-guest.config.microvm.declaredRunner
      nixosConfigurations.coda-ci-guest = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        modules = [
          microvm.nixosModules.microvm
          ./nix/ci-guest.nix
        ];
      };

      devShells = forEachSystem (pkgs: {
        default = pkgs.mkShell {
          # lld: Cargo also reads the repository's .cargo/config.toml, which
          # links x86_64 Linux through it.
          packages = with pkgs; [ cargo rustc clippy rustfmt rust-analyzer lld ];
        };
      });
    };
}
