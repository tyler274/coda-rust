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
    # Crates come only from Cargo.lock checksums (fixed-output vendor); the
    # build itself runs in the sandbox without network.
    cargoLock.lockFile = ../Cargo.lock;
    nativeBuildInputs = [ pkgs.pkg-config ];
    buildInputs = [ pkgs.openssl ];
    OPENSSL_NO_VENDOR = "1";
    # Functional/browser/database proofs remain separate mandatory CI jobs.
    doCheck = false;
    strictDeps = true;
  };

  # One locked release build compiles the shared dependency graph once for
  # every server, migration and job binary.
  serverPackages = [ "physician-sample" "duet-api" "local-product" "physician-auth" "front-door" ];
  servers = rustPlatform.buildRustPackage (common // {
    pname = "coda-servers";
    cargoBuildFlags = [ "--locked" "--bins" ] ++ lib.concatMap (p: [ "-p" p ]) serverPackages;
    installPhase = ''
      runHook preInstall
      mkdir -p $out/bin
      find target/x86_64-unknown-linux-gnu/release -maxdepth 1 -type f -executable \
        -exec install -m755 -t $out/bin {} +
      runHook postInstall
    '';
  });
  # Each image references exactly one executable, so the others never enter
  # its closure. Copying keeps the store references (glibc, openssl) intact.
  native = binary: pkgs.runCommand binary { } ''
    install -Dm755 ${servers}/bin/${binary} $out/bin/${binary}
  '';

  webPackages = [ "physician-app" "duet-app" ];
  webBuild = rustPlatform.buildRustPackage (common // {
    pname = "coda-web";
    nativeBuildInputs = common.nativeBuildInputs ++ [ pkgs.wasm-bindgen-cli_0_2_129 ];
    buildPhase = ''
      runHook preBuild
      # Ambient credentials and API URLs never enter browser artifacts.
      unset DUET_API_URL DUET_BEARER DUET_API_BEARER PHYSICIAN_API_URL PHYSICIAN_API_BEARER
      cargo build --frozen --release --target wasm32-unknown-unknown \
        ${lib.concatMapStringsSep " " (p: "-p ${p}") webPackages} \
        --no-default-features --features ${lib.concatMapStringsSep "," (p: "${p}/web") webPackages}
      runHook postBuild
    '';
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
  # same release, URL and SHA-256 that scripts/gcp/install_doppler.sh installs
  # on runners: one pin, read from that script, verified by fetchurl. The
  # upstream binary is static, so it adds no libraries to the closure.
  dopplerPin =
    let
      script = builtins.readFile ../scripts/gcp/install_doppler.sh;
      pin = name: builtins.head (builtins.match ".*readonly ${name}=\"([^\"]+)\".*" script);
    in
    { version = pin "DOPPLER_VERSION"; sha256 = pin "DOPPLER_SHA256"; };
  doppler = pkgs.stdenvNoCC.mkDerivation {
    pname = "doppler-cli";
    version = lib.removePrefix "v" dopplerPin.version;
    src = pkgs.fetchurl {
      name = "doppler-${dopplerPin.version}.tar";
      url = "https://cli.doppler.com/download?os=linux&arch=amd64&format=tar&version=${dopplerPin.version}";
      inherit (dopplerPin) sha256;
    };
    dontUnpack = true;
    dontFixup = true;
    installPhase = ''
      tar -xf $src doppler
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
  image = { name, binary, executable, role, assets ? null, environment ? [ ] }:
    pkgs.dockerTools.buildLayeredImage {
      inherit name;
      tag = revision;
      created = "1970-01-01T00:00:01Z";
      # No shell, package manager or toolchain: the binary, its libraries,
      # Doppler, CA roots, zoneinfo and the passwd/group files.
      contents = [ binary doppler pkgs.dockerTools.caCertificates pkgs.tzdata passwd ]
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
        # wins over any PORT value stored in Doppler.
        Entrypoint = [
          "${doppler}/bin/doppler" "run"
          "--no-check-version" "--no-fallback" "--forward-signals" "--preserve-env=PORT"
          "--" "${binary}/bin/${executable}"
        ];
        Env = [
          "HOME=/tmp"
          "TMPDIR=/tmp"
          "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
          "TZDIR=${zoneinfo}"
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
  duet = native "duet-api";
  # Product entrypoints compose their owned chains; future conversation owners
  # extend these binaries, not the image wrapper or runtime startup.
  physicianMigration = native "physician-migrate";
  duetMigration = native "duet-migrate";
  duetPrivacyDelivery = native "duet-privacy-delivery";
  physicianWeb = web "physician-app";
  duetWeb = web "duet-app";
in
{
  coda-servers = servers;
  coda-web = webBuild;
  doppler-cli = doppler;
  physician-sample = physician;
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
      "PHYSICIAN_SITE_DIR=${physicianWeb}"
      # Hosted physician chat is API-backed only and sessions are required.
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
  };
  duet-migration-image = image {
    name = "coda-duet-migrate";
    role = "migration";
    binary = duetMigration;
    executable = "duet-migrate";
  };
  # Bounded Luna privacy receipt delivery job. Not a migration; no scheduler.
  duet-privacy-delivery-image = image {
    name = "coda-duet-privacy-delivery";
    role = "job";
    binary = duetPrivacyDelivery;
    executable = "duet-privacy-delivery";
  };
}
