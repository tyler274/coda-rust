{ pkgs, self, crane }:
let
  inherit (pkgs) lib;
  # rust-toolchain.toml pins this toolchain and its WASM std.
  toolchain = import ./rust-toolchain.nix pkgs;
  craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
  # Keep crate-local inputs intact: HTML, SQL, JSON, images and generated data
  # are compile-time inputs as well as Rust, including clinical Markdown.
  # Root-level runtime skills and patient ledgers have separate derivations.
  src = lib.fileset.toSource {
    root = ../.;
    fileset = lib.fileset.unions [
      ../Cargo.toml
      ../Cargo.lock
      ../crates
      ../skills
      ../platform/ios/build_bridge.rs
    ];
  };
  cargoVendorDir = craneLib.vendorCargoDeps { inherit src; };
  common = {
    version = "0.1.0";
    inherit src cargoVendorDir;
    # Registry checksums and Git revisions come from Cargo.lock. Both stages
    # build frozen, offline, in the normal Nix sandbox.
    cargoExtraArgs = "--frozen";
    CARGO_BUILD_TARGET = "x86_64-unknown-linux-gnu";
    cargoProfile = "release";
    nativeBuildInputs = [ pkgs.pkg-config ];
    buildInputs = [ pkgs.openssl ];
    OPENSSL_NO_VENDOR = "1";
    # Functional/browser/database proofs remain separate mandatory CI jobs.
    doCheck = false;
    strictDeps = true;
  };

  # One locked release build compiles the shared dependency graph once for
  # every server, migration and job binary.
  serverPackages = [ "physician-sample" "duet-api" "local-product" "physician-auth" "front-door" "availability-probe" "observability-relay" ];
  serverArgs = common // {
    pname = "coda-servers";
    cargoExtraArgs = "--frozen --bins " + lib.concatMapStringsSep " " (p: "-p ${p}") serverPackages;
  };
  serverDeps = craneLib.buildDepsOnly serverArgs;
  servers = craneLib.buildPackage (serverArgs // {
    cargoArtifacts = serverDeps;
  });
  # Each image references exactly one executable, so the others never enter
  # its closure. Copying keeps the store references (glibc, openssl) intact.
  native = binary: pkgs.runCommand binary { } ''
    install -Dm755 ${servers}/bin/${binary} $out/bin/${binary}
  '';

  patientBankArgs = common // {
    pname = "patient-bank";
    cargoExtraArgs = "--frozen -p patient-bank --bin patient-bank";
  };
  patientBankDeps = craneLib.buildDepsOnly patientBankArgs;
  patientBankTool = craneLib.buildPackage (patientBankArgs // {
    cargoArtifacts = patientBankDeps;
  });
  # Committed ledgers and seeds enter the offline chart build. The migration
  # image carries only the rendered charts, practice calendar and manifest,
  # never the toolchain.
  patientBankInputs = lib.fileset.toSource {
    root = ../patients;
    fileset = lib.fileset.fileFilter
      (file: file.name == "seed.json" || file.name == "ledger.json") ../patients;
  };
  physicianCharts = pkgs.runCommand "physician-synthetic-charts" {
    nativeBuildInputs = [ patientBankTool ];
    PATIENT_BANK_DATA = lib.fileset.toSource {
      root = ../crates/patient-bank/data;
      fileset = ../crates/patient-bank/data;
    };
    PATIENT_BANK_PATIENTS = patientBankInputs;
  } ''
    patient-bank build --out bank
    mkdir -p $out
    cp -r bank/patients bank/calendar bank/manifest.json $out/
  '';

  webPackages = [ "physician-app" "duet-app" ];
  webArgs = common // {
    pname = "coda-web";
    CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
    cargoExtraArgs = "--frozen "
      + lib.concatMapStringsSep " " (p: "-p ${p}") webPackages
      + " --no-default-features --features "
      + lib.concatMapStringsSep "," (p: "${p}/web") webPackages;
    preBuild = ''
      # Ambient credentials and API URLs never enter browser artifacts.
      unset DUET_API_URL DUET_BEARER DUET_API_BEARER PHYSICIAN_API_URL PHYSICIAN_API_BEARER
    '';
  };
  webDeps = craneLib.buildDepsOnly webArgs;
  webBuild = craneLib.buildPackage (webArgs // {
    cargoArtifacts = webDeps;
    nativeBuildInputs = common.nativeBuildInputs ++ [ pkgs.wasm-bindgen-cli_0_2_129 ];
    installPhase = ''
      runHook preInstall
      for package in ${lib.concatStringsSep " " webPackages}; do
        wasm-bindgen target/wasm32-unknown-unknown/release/$package.wasm \
          --target web --out-dir $out/$package --out-name $package
        cp crates/$package/index.html $out/$package/index.html
      done
      runHook postInstall
    '';
  });
  web = package: pkgs.runCommand "${package}-web" { } ''
    cp -r ${webBuild}/${package} $out
  '';

  # Production processes start under `doppler run --` (AGENTS.md). This is the
  # same release, URL and SHA-256 that `workspace-tools install-doppler`
  # installs on runners: one pin, read from crates/workspace-tools/src/doppler.rs,
  # verified by fetchurl. The upstream binary is static, so it adds no
  # libraries to the closure.
  dopplerPin =
    let
      source = builtins.readFile ../crates/workspace-tools/src/doppler.rs;
      pin = name: builtins.head (builtins.match ".*pub const ${name}: &str =[[:space:]]*\"([^\"]+)\";.*" source);
    in
    { version = pin "VERSION"; sha256 = pin "SHA256"; url = pin "URL"; };
  doppler = pkgs.stdenvNoCC.mkDerivation {
    pname = "doppler-cli";
    version = lib.removePrefix "v" dopplerPin.version;
    src = pkgs.fetchurl {
      name = "doppler-${dopplerPin.version}.tar.gz";
      inherit (dopplerPin) url sha256;
    };
    dontUnpack = true;
    dontFixup = true;
    installPhase = ''
      tar -xzf $src doppler
      install -Dm755 doppler $out/bin/doppler
    '';
  };

  revision = self.rev or self.dirtyRev or "uncommitted";
  # Named non-root account for libraries that resolve the current uid.
  passwd = pkgs.dockerTools.fakeNss.override {
    extraPasswdLines = [ "nonroot:x:65532:65532:nonroot:/tmp:/var/empty/nologin" ];
    extraGroupLines = [ "nonroot:x:65532:" ];
  };
  zoneinfo = "${pkgs.tzdata}/share/zoneinfo";
  image = { name, binary, executable, role, assets ? null, environment ? [ ], secrets ? true }:
    pkgs.dockerTools.buildLayeredImage {
      inherit name;
      tag = revision;
      created = "1970-01-01T00:00:01Z";
      # No shell, package manager or toolchain: the binary, its libraries,
      # Doppler, CA roots, zoneinfo and the passwd/group files.
      contents = [ binary pkgs.dockerTools.caCertificates pkgs.tzdata passwd ]
        ++ lib.optional secrets doppler
        ++ lib.optional (assets != null) assets;
      fakeRootCommands = ''
        mkdir -p tmp
        chmod 1777 tmp
      '';
      config = {
        User = "65532:65532";
        WorkingDir = "/tmp";
        # Secrets arrive only at start: Cloud Run mounts DOPPLER_TOKEN and
        # Doppler injects the config. No fallback file is written or read, so
        # no secret copy lands on the container filesystem. Cloud Run's PORT
        # wins over any PORT value stored in Doppler. Infrastructure telemetry
        # settings and the image's immutable version also win over Doppler.
        # A secret-free job (secrets = false) starts its binary directly.
        Entrypoint = if !secrets then [ "${binary}/bin/${executable}" ] else [
          "${doppler}/bin/doppler" "run"
          "--no-check-version" "--no-fallback" "--forward-signals"
          "--preserve-env=PORT,DD_VERSION,DD_ENV,OTEL_EXPORTER_OTLP_ENDPOINT,CODA_LOG_FORMAT,CODA_TELEMETRY_CONTENT"
          "--" "${binary}/bin/${executable}"
        ];
        Env = [
          "HOME=/tmp"
          "TMPDIR=/tmp"
          "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
          "TZDIR=${zoneinfo}"
          # Immutable image version, preserved across Doppler injection.
          "DD_VERSION=${revision}"
        ] ++ lib.optional (role == "runtime") "PORT=8080" ++ environment;
        Labels = {
          "org.opencontainers.image.revision" = revision;
          "org.opencontainers.image.source" = "https://github.com/Coda-Health/coda-rust";
          # Packaging current applications is NOT the durable hosted runtime gate.
          "com.coda.runtime-readiness" = "packaging-only-not-staging-ready";
          "com.coda.role" = role;
          "com.coda.binary" = "${binary}/bin/${executable}";
        };
      };
    };

  physician = native "physician-sample";
  physicianWorker = native "physician-worker";
  duet = native "duet-api";
  # Product entrypoints compose their owned chains; future conversation owners
  # extend these binaries, not the image wrapper or runtime startup.
  physicianMigration = native "physician-migrate";
  duetMigration = native "duet-migrate";
  duetPrivacyDelivery = native "duet-privacy-delivery";
  physicianWeb = web "physician-app";
  duetWeb = web "duet-app";
  # The packaged clinical skill catalog the admitting physician site loads once at
  # startup (PHYSICIAN_SKILLS_DIR). Only skills/ enters this store path;
  # being in the build source alone would not put it in the image.
  physicianSkills = pkgs.runCommand "physician-skills" { } ''
    cp -r ${lib.fileset.toSource { root = ../skills; fileset = ../skills; }} $out
  '';
in
{
  coda-servers = servers;
  coda-servers-deps = serverDeps;
  coda-web = webBuild;
  coda-web-deps = webDeps;
  patient-bank = patientBankTool;
  patient-bank-deps = patientBankDeps;
  doppler-cli = doppler;
  physician-sample = physician;
  physician-worker = physicianWorker;
  physician-skills = physicianSkills;
  physician-synthetic-charts = physicianCharts;
  duet-api = duet;
  physician-web = physicianWeb;
  duet-web = duetWeb;
  physician-migrate = physicianMigration;
  duet-migrate = duetMigration;
  duet-privacy-delivery = duetPrivacyDelivery;
  # Standalone chain binaries for explicit operator runs; the product
  # migration images already compose them.
  physician-auth-migrate = native "physician-auth-migrate";
  front-door-migrate = native "front-door-migrate";
  physician-runtime-migrate = native "physician-runtime-migrate";

  physician-sample-image = image {
    name = "coda-physician-sample";
    role = "runtime";
    binary = physician;
    executable = "physician-sample";
    assets = physicianWeb;
    environment = [
      "PHYSICIAN_SKILLS_DIR=${physicianSkills}"
      "PHYSICIAN_SYNTHETIC_CHARTS=${physicianCharts}"
      "PHYSICIAN_SITE_DIR=${physicianWeb}"
      # Hosted physician chat is API-backed only and sessions are required.
      "PHYSICIAN_AUTH=required"
      "CODA_LOCAL_MODEL_EXECUTION=api"
    ];
  };
  physician-worker-image = image {
    name = "coda-physician-worker";
    role = "runtime";
    binary = physicianWorker;
    executable = "physician-worker";
    # Hosted startup requires sessions, exactly as on the site.
    environment = [
      "PHYSICIAN_AUTH=required"
      "CODA_LOCAL_MODEL_EXECUTION=api"
    ];
  };
  duet-api-image = image {
    name = "coda-duet-api";
    role = "runtime";
    binary = duet;
    executable = "duet-api";
    assets = duetWeb;
    environment = [ "DUET_SITE_DIR=${duetWeb}" ];
  };
  physician-migration-image = image {
    name = "coda-physician-migrate";
    role = "migration";
    binary = physicianMigration;
    executable = "physician-migrate";
    assets = physicianCharts;
    environment = [ "PHYSICIAN_SYNTHETIC_CHARTS=${physicianCharts}" ];
  };
  duet-migration-image = image {
    name = "coda-duet-migrate";
    role = "migration";
    binary = duetMigration;
    executable = "duet-migrate";
  };
  # Authenticated /readyz probe job. It reads no secrets: its identity is the
  # Cloud Run Job service account, so the image has no Doppler entrypoint.
  availability-probe-image = image {
    name = "coda-availability-probe";
    role = "job";
    binary = native "availability-probe";
    executable = "availability-probe";
    secrets = false;
  };
  # Staging hop from public-site to the private Grafana load
  # balancer. Cloud Run IAP authenticates its callers and it reads no
  # secrets, so the image has no Doppler entrypoint.
  observability-relay-image = image {
    name = "coda-observability-relay";
    role = "runtime";
    binary = native "observability-relay";
    executable = "observability-relay";
    secrets = false;
  };
  # Bounded Luna privacy receipt delivery job. Not a migration; no scheduler.
  duet-privacy-delivery-image = image {
    name = "coda-duet-privacy-delivery";
    role = "job";
    binary = duetPrivacyDelivery;
    executable = "duet-privacy-delivery";
  };
}
