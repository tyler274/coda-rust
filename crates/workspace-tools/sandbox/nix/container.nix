# The coda-agent container: where coding agents (and you, over Remote-SSH)
# work on coda-rust worktrees. nspawn.nix says what every sandbox guarantees.
#
# Paths inside the container equal the host paths, so GC roots the host Nix
# daemon records for a worktree's dev shell stay valid from the host's view.
{ config, lib, ... }:
let
  cfg = config.services.coda-sandbox;
  name = cfg.container.name;
  brokerDir = builtins.dirOf cfg.broker.socket;
in
lib.mkIf (cfg.enable && cfg.container.enable) {
  agentSandbox.containers.${name} = {
    inherit (cfg)
      uidBase
      hostAddress
      localAddress
      memoryMax
      memoryHigh
      authorizedKeys
      ;
    autoStart = true;
    requires = cfg.requires ++ [ "coda-ci-spool.service" ];
    binds = {
      ${cfg.paths.src} = {
        hostPath = cfg.paths.src;
        ownedByAgent = true;
      };
      ${cfg.paths.build} = {
        hostPath = cfg.paths.build;
        ownedByAgent = true;
      };
      # The spool is made by coda-ci-spool.service (ci.nix): the agent may
      # drop requests there but cannot list or remove anyone else's. Bound
      # at the host's path, so the config's [ci] spool names it on both
      # sides; the rest of paths.ci (seed, jobs) stays out of the container.
      "${cfg.paths.ci}/spool/requests" = {
        hostPath = "${cfg.paths.ci}/spool/requests";
      };
      "${cfg.paths.ci}/spool/results" = {
        hostPath = "${cfg.paths.ci}/spool/results";
        readOnly = true;
      };
      # Bundles of the host worktrees written by coda-carry. Kept off the
      # agent-owned tree, so the host never writes where the agent can plant
      # a symlink.
      "${cfg.paths.root}/carry" = {
        hostPath = cfg.carry.dir;
        readOnly = true;
      };
      # The token broker's socket directory (broker.nix), at the host's
      # path, so the config's [broker] socket names it on both sides.
      # Binding the directory keeps a re-created socket visible; connecting
      # to a socket needs no writable mount.
      ${brokerDir} = {
        hostPath = brokerDir;
        readOnly = true;
      };
    };
    config = {
      imports = [
        (import ./container-guest.nix {
          inherit (cfg.paths) repo;
          inherit (cfg) package;
        })
        cfg.container.config
      ];
      environment.systemPackages = [ cfg.package ];
      environment.etc."coda-sandbox/config.toml".source = cfg.configFile;
    };
  };

  systemd.tmpfiles.rules = [
    "d ${cfg.carry.dir} 0755 root root -"
  ];

  environment.etc."coda-agent/uid".text = toString (cfg.uidBase + 1000);
}
