# Complete CI runtime, including the shell's setup-hook exports and shellHook.
# Tag packages.postgres-ci-image.drvPath, NOT the shell's drvPath: changing this
# recipe or its environment renderer must invalidate the immutable image tag.
# No evaluation, Nix daemon, or mutable second image layer is needed at runtime.
{ pkgs, shell }:
let
  fhsLibs = pkgs.lib.makeLibraryPath [ pkgs.stdenv.cc.cc.lib pkgs.glibc ];
  # Reuse the shell derivation's inputs, env and stdenv initialization. Capture
  # after setup (as print-dev-env does), but emit only runtime exports. This
  # avoids copying compiler flags/package setup hooks into the image recipe.
  capturedEnv = shell.overrideAttrs (_: {
    name = "postgres-ci-runtime-env";
    buildPhase = ''
      ${pkgs.python314}/bin/python ${../scripts/nix-cache/postgres_image_env.py} > "$out"
    '';
  });
  runtimeEnv = pkgs.runCommand "postgres-ci-runtime" { } ''
    mkdir -p $out/etc/coda
    cp ${capturedEnv} $out/etc/coda/postgres-ci.env
  '';
in
pkgs.dockerTools.streamLayeredImage {
  name = "postgres-ci";
  tag = "latest";
  created = "1970-01-01T00:00:01Z";
  maxLayers = 100;
  # The captured runtime exports root the tools, compiler headers/libraries and
  # their transitive closures. inputDerivation would additionally retain build
  # hooks and documentation that aren't referenced by the activated environment.
  contents = [
    runtimeEnv
    pkgs.bashInteractive
    pkgs.coreutils
    pkgs.findutils
    pkgs.gnugrep
    pkgs.gnused
    pkgs.gnutar
    pkgs.gzip
    pkgs.zstd
    # Checkout/cache need Git's HTTPS transport, not git-send-email, gitweb,
    # Python helpers or manuals. Keep the nixpkgs-maintained minimal variant.
    pkgs.gitMinimal
    pkgs.procps
    pkgs.dockerTools.caCertificates
    pkgs.dockerTools.fakeNss
    pkgs.dockerTools.usrBinEnv
    pkgs.dockerTools.binSh
  ];
  extraCommands = ''
    mkdir -p tmp lib64 etc/coda
    chmod 1777 tmp
    ln -s ${pkgs.nix-ld}/libexec/nix-ld lib64/ld-linux-x86-64.so.2
  '';
  config = {
    Labels."org.opencontainers.image.source" = "https://github.com/Coda-Health/coda-rust";
    Env = [
      "PATH=/bin:/usr/bin"
      "NIX_LD=${pkgs.glibc}/lib/ld-linux-x86-64.so.2"
      "NIX_LD_LIBRARY_PATH=${fhsLibs}"
      "SSL_CERT_FILE=/etc/ssl/certs/ca-bundle.crt"
      "BASH_ENV=/etc/coda/postgres-ci.env"
    ];
  };
}
