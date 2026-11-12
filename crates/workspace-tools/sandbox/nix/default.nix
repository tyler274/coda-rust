# services.coda-sandbox: the Coda agent sandbox on a NixOS host.
#
# One option set declares the coda-agent container (container.nix on the
# hardened nspawn module nspawn.nix), its token broker (broker.nix), the
# Firecracker CI (ci.nix), the coda-carry bundler, and
# /etc/coda-sandbox/config.toml, which the
# coda-sandbox binary reads by default. The flake passes its packages and
# the microvm module in, so a host needs only this module.
{ packages, microvmModule }:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.services.coda-sandbox;
  toml = pkgs.formats.toml { };

  # The binary's schema: crates/workspace-tools/sandbox/src/config.rs.
  configFile = toml.generate "coda-sandbox-config.toml" {
    version = 1;
    container = {
      name = cfg.container.name;
      unit = "container@${cfg.container.name}.service";
      ssh_host = cfg.container.sshHost;
      agent_uid = cfg.uidBase + 1000;
      repo = cfg.paths.repo;
    };
    build.root = cfg.paths.build;
    ci = {
      spool = "${cfg.paths.ci}/spool";
      timer = "coda-ci.timer";
      service = "coda-ci.service";
    };
    broker = {
      socket = cfg.broker.socket;
      gh_fallback = if cfg.broker.ghFallback then "on" else "off";
    }
    // lib.optionalAttrs cfg.ci.enable {
      ci = {
        socket = cfg.broker.ciSocket;
        user = "coda-ci";
      };
    };
  };

  carry = pkgs.callPackage ./carry-package.nix {
    inherit (cfg) user;
    repo = cfg.carry.sourceRepo;
    out = cfg.carry.dir;
  };
in
{
  imports = [
    ./nspawn.nix
    ./container.nix
    ./broker.nix
    (import ./ci.nix { inherit microvmModule; })
  ];

  options.services.coda-sandbox = {
    enable = lib.mkEnableOption "the Coda agent sandbox: the coda-agent container, its CI spool and coda-carry";

    package = lib.mkOption {
      type = lib.types.package;
      default = packages.${pkgs.stdenv.hostPlatform.system}.coda-sandbox;
      defaultText = lib.literalMD "the `coda-sandbox` package of this flake";
      description = "The coda-sandbox binary, installed on the host and in the container.";
    };

    configFile = lib.mkOption {
      type = lib.types.path;
      default = configFile;
      readOnly = true;
      internal = true;
      description = "The generated config.toml, installed on the host and in the container.";
    };

    user = lib.mkOption {
      type = lib.types.str;
      example = "luluco";
      description = "The host user who works with the sandbox and owns the host checkout coda-carry bundles.";
    };

    requires = lib.mkOption {
      type = lib.types.listOf lib.types.str;
      default = [ ];
      example = [ "agents-pool.service" ];
      description = ''
        Units that make the paths under `paths.root` ready, such as a late pool
        import. The container, the CI spool and the CI units require them, so
        a missing disk fails only those units and never boot.
      '';
    };

    uidBase = lib.mkOption {
      type = lib.types.ints.u32;
      default = 655360;
      description = ''
        First host UID of the container's 65536-UID range; a non-zero multiple
        of 65536. The agent is host UID uidBase + 1000.
      '';
    };

    hostAddress = lib.mkOption {
      type = lib.types.str;
      default = "10.233.1.1";
      description = "The host end of the container's veth pair.";
    };

    localAddress = lib.mkOption {
      type = lib.types.str;
      default = "10.233.1.2";
      description = "The container end of its veth pair; the SSH entry's HostName.";
    };

    memoryMax = lib.mkOption {
      type = lib.types.str;
      default = "24G";
      description = "Hard memory limit of the container unit.";
    };

    memoryHigh = lib.mkOption {
      type = lib.types.nullOr lib.types.str;
      default = null;
      description = "Throttle point below memoryMax; defaults to memoryMax.";
    };

    authorizedKeys = lib.mkOption {
      type = lib.types.listOf lib.types.str;
      default = [ ];
      description = "SSH public keys that may log in as the container's agent; sshd runs only when there is one.";
    };

    container = {
      enable = lib.mkOption {
        type = lib.types.bool;
        default = true;
        description = "Declare the agent container. Off leaves the CI spool, CI and coda-carry.";
      };
      name = lib.mkOption {
        type = lib.types.str;
        default = "coda-agent";
        description = "The container's name; its unit is container@<name>.service.";
      };
      sshHost = lib.mkOption {
        type = lib.types.str;
        default = "coda-agent";
        description = "The `Host` alias the user's SSH config gives the container.";
      };
      config = lib.mkOption {
        type = lib.types.deferredModule;
        default = { };
        description = "Extra NixOS configuration for the container.";
      };
    };

    paths = {
      root = lib.mkOption {
        type = lib.types.str;
        default = "/agents/coda";
        description = ''
          Holds src, build and ci. Paths inside the container equal the host
          paths, so GC roots the host Nix daemon records for a worktree's dev
          shell stay valid from the host's view. build must support reflinks.
        '';
      };
      src = lib.mkOption {
        type = lib.types.str;
        default = "${cfg.paths.root}/src";
        defaultText = lib.literalExpression ''"''${paths.root}/src"'';
        description = "Agent-owned source trees.";
      };
      build = lib.mkOption {
        type = lib.types.str;
        default = "${cfg.paths.root}/build";
        defaultText = lib.literalExpression ''"''${paths.root}/build"'';
        description = "Agent-owned build output; must support reflinks.";
      };
      ci = lib.mkOption {
        type = lib.types.str;
        default = "${cfg.paths.root}/ci";
        defaultText = lib.literalExpression ''"''${paths.root}/ci"'';
        description = "The CI spool, seed and job directories.";
      };
      repo = lib.mkOption {
        type = lib.types.str;
        default = "${cfg.paths.src}/coda-rust";
        defaultText = lib.literalExpression ''"''${paths.src}/coda-rust"'';
        description = "The coda-rust checkout inside the container.";
      };
    };

    carry = {
      dir = lib.mkOption {
        type = lib.types.str;
        default = "/var/lib/coda-carry";
        description = "Where coda-carry installs its bundle; bound read-only into the container.";
      };
      sourceRepo = lib.mkOption {
        type = lib.types.str;
        default = "/home/${cfg.user}/code/coda/coda-rust";
        defaultText = lib.literalExpression ''"/home/''${user}/code/coda/coda-rust"'';
        description = "The host checkout whose worktrees coda-carry bundles.";
      };
    };
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = [
      cfg.package
      carry
    ];
    environment.etc."coda-sandbox/config.toml".source = cfg.configFile;
  };
}
