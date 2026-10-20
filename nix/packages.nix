{ pkgs, self }:
let
  inherit (pkgs) lib;
  # The overlay revision in flake.lock pins this toolchain and its WASM std.
  toolchain = pkgs.rust-bin.stable.latest.default.override {
    targets = [ "wasm32-unknown-unknown" ];
  };
  rustPlatform = pkgs.makeRustPlatform { cargo = toolchain; rustc = toolchain; };
  src = lib.fileset.toSource {
    root = ../.;
    fileset = lib.fileset.unions [ ../Cargo.toml ../Cargo.lock ../crates ../skills ];
  };
  common = {
    version = "0.1.0";
    inherit src;
    cargoLock.lockFile = ../Cargo.lock;
    nativeBuildInputs = [ pkgs.pkg-config ];
    buildInputs = [ pkgs.openssl ];
    OPENSSL_NO_VENDOR = "1";
    # Functional/browser/database proofs remain separate mandatory CI jobs.
    doCheck = false;
    strictDeps = true;
  };
  native = name: package: binary: rustPlatform.buildRustPackage (common // {
    pname = name;
    cargoBuildFlags = [ "--locked" "-p" package "--bin" binary ];
    installPhase = ''
      runHook preInstall
      install -Dm755 target/x86_64-unknown-linux-gnu/release/${binary} $out/bin/${binary}
      runHook postInstall
    '';
  });
  web = package: rustPlatform.buildRustPackage (common // {
    pname = "${package}-web";
    nativeBuildInputs = common.nativeBuildInputs ++ [ pkgs.wasm-bindgen-cli_0_2_129 ];
    buildPhase = ''
      runHook preBuild
      # Ambient credentials and API URLs never enter browser artifacts.
      unset DUET_API_URL DUET_BEARER DUET_API_BEARER PHYSICIAN_API_URL PHYSICIAN_API_BEARER
      cargo build --frozen --release --target wasm32-unknown-unknown -p ${package} --no-default-features --features web
      runHook postBuild
    '';
    installPhase = ''
      runHook preInstall
      mkdir -p $out
      wasm-bindgen target/wasm32-unknown-unknown/release/${package}.wasm --target web --out-dir $out --out-name ${package}
      cp crates/${package}/index.html $out/index.html
      runHook postInstall
    '';
  });
  revision = self.rev or self.dirtyRev or "uncommitted";
  skills = pkgs.runCommand "coda-skills" { } ''
    mkdir -p $out/share/coda
    cp -r ${../skills} $out/share/coda/skills
  '';
  image = { name, binary, executable, assets ? null, environment ? [], extra ? [] }:
    let
      # Doppler is bootstrapped only at runtime; never during construction.
      entrypoint = pkgs.writeShellScriptBin "start-${name}" ''
        set -eu
        export PHYSICIAN_LISTEN_ADDR="0.0.0.0:''${PORT:-8080}"
        export DUET_LISTEN_ADDR="0.0.0.0:''${PORT:-8080}"
        exec ${pkgs.doppler}/bin/doppler run --no-cache -- ${binary}/bin/${executable} "$@"
      '';
    in pkgs.dockerTools.buildLayeredImage {
      inherit name;
      tag = revision;
      created = "1970-01-01T00:00:01Z";
      contents = [ pkgs.cacert entrypoint binary ] ++ extra ++ lib.optional (assets != null) assets;
      fakeRootCommands = ''
        mkdir -p tmp
        chmod 1777 tmp
      '';
      config = {
        User = "65532:65532";
        WorkingDir = "/tmp";
        Entrypoint = [ "${entrypoint}/bin/start-${name}" ];
        Env = [ "HOME=/tmp" "TMPDIR=/tmp" "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt" ] ++ environment;
        Labels = {
          "org.opencontainers.image.revision" = revision;
          "org.opencontainers.image.source" = "https://github.com/Coda-Health/coda-rust";
          # Packaging current applications is NOT the durable hosted runtime gate.
          "com.coda.runtime-readiness" = "packaging-only-not-staging-ready";
          "com.coda.binary" = "${binary}/bin/${executable}";
        };
      };
    };
  physician = native "physician-sample" "physician-sample" "physician-sample";
  duet = native "duet-api" "duet-api" "duet-api";
  physicianMigration = native "physician-auth-migrate" "physician-auth" "physician-auth-migrate";
  frontDoorMigration = native "front-door-migrate" "front-door" "front-door-migrate";
  physicianWeb = web "physician-app";
  duetWeb = web "duet-app";
in {
  physician-sample = physician;
  duet-api = duet;
  physician-web = physicianWeb;
  duet-web = duetWeb;
  physician-auth-migrate = physicianMigration;
  front-door-migrate = frontDoorMigration;
  physician-sample-image = image {
    name = "coda-physician-sample";
    binary = physician;
    executable = "physician-sample";
    assets = physicianWeb;
    extra = [ skills ];
    environment = [
      "PHYSICIAN_SITE_DIR=${physicianWeb}"
      "CODA_SKILLS_DIR=${skills}/share/coda/skills"
      "PHYSICIAN_AUTH=required"
      "CODA_LOCAL_MODEL_EXECUTION=api"
    ];
  };
  duet-api-image = image {
    name = "coda-duet-api";
    binary = duet;
    executable = "duet-api";
    assets = duetWeb;
    environment = [ "DUET_SITE_DIR=${duetWeb}" ];
  };
  # These jobs cover only existing workspace schemas, not future chat/account migrations.
  physician-auth-migration-image = image {
    name = "coda-physician-auth-migrate";
    binary = physicianMigration;
    executable = "physician-auth-migrate";
  };
  front-door-migration-image = image {
    name = "coda-front-door-migrate";
    binary = frontDoorMigration;
    executable = "front-door-migrate";
  };
}
