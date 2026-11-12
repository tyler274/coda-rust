# The token broker: a `systemd --user` service of services.coda-sandbox.user
# that hands the agent container its tokens over a Unix socket. It reads
# them from the user's Secret Service keyring with `secret-tool`, so no
# token sits in a file; `coda-sandbox broker store <item>` puts them there.
#
# The socket's directory is made by root, owned by the user and grouped to
# the container's `users` group, and container.nix binds it into the
# container at the same path. The directory gates traversal; the broker's
# SO_PEERCRED check gates the rest.
#
# The polkit rule lets the same user start, stop and restart the sandbox's
# system units this host defines, without sudo; nothing else, and never a
# transient unit under one of their names.
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.services.coda-sandbox;
  hostUsersGid = cfg.uidBase + 100;
  socketDir = builtins.dirOf cfg.broker.socket;
  # Only units this host defines. A name with no unit file is one the user
  # could otherwise create as a transient unit running anything as root.
  defined =
    kind: names: builtins.filter (name: config.systemd.${kind} ? ${name}) names;
  units =
    map (name: "${name}.service") (
      defined "services" [
        "container@${cfg.container.name}"
        "coda-ci"
        "coda-ci-seed"
      ]
    )
    ++ map (name: "${name}.timer") (defined "timers" [ "coda-ci" ]);
  # systemd asks polkit for `manage-units` with the same `unit` and `verb`
  # (`start`) for StartUnit and StartTransientUnit; only the message
  # detail tells them apart. Granting only these exact job messages lets
  # start, stop and restart through and nothing else.
  jobMessages = {
    start = "Authentication is required to start '$(unit)'.";
    stop = "Authentication is required to stop '$(unit)'.";
    restart = "Authentication is required to restart '$(unit)'.";
  };
  transientMessage = "Authentication is required to start transient unit '$(unit)'.";
in
{
  options.services.coda-sandbox.broker = {
    socket = lib.mkOption {
      type = lib.types.str;
      default = "/run/coda-sandbox/agent/broker.sock";
      readOnly = true;
      description = "The agent socket, at the same path on the host and in the container.";
    };

    ghFallback = lib.mkOption {
      type = lib.types.bool;
      default = false;
      description = ''
        Let `gh auth token` stand in for a `github-agent` token the keyring
        does not hold. That token reaches every repository the user can, so
        doctor warns while this is on.
      '';
    };

    keyringPackage = lib.mkOption {
      type = lib.types.package;
      default = pkgs.libsecret;
      defaultText = lib.literalExpression "pkgs.libsecret";
      description = "Provides `bin/secret-tool` on the broker's PATH; the VM check swaps in a stub.";
    };
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = [ cfg.broker.keyringPackage ];

    systemd.tmpfiles.rules = [
      "d /run/coda-sandbox 0755 root root -"
      "d ${socketDir} 0750 ${cfg.user} ${toString hostUsersGid} -"
    ];

    systemd.user.services.coda-sandbox-broker = {
      description = "Coda sandbox token broker";
      wantedBy = [ "default.target" ];
      unitConfig.ConditionUser = cfg.user;
      path = [
        cfg.broker.keyringPackage
        pkgs.gh
      ];
      serviceConfig = {
        ExecStart = "${lib.getExe' cfg.package "coda-sandbox"} broker serve";
        ExecReload = "${pkgs.coreutils}/bin/kill -HUP $MAINPID";
        Restart = "on-failure";
        RestartSec = "5s";
        # A core dump would put the cached tokens in a file.
        LimitCORE = 0;
      };
    };

    security.polkit.extraConfig = ''
      // services.coda-sandbox: its user starts, stops and restarts the
      // sandbox's units without sudo, and never creates a transient unit
      // under their names.
      polkit.addRule(function (action, subject) {
        if (action.id != "org.freedesktop.systemd1.manage-units" ||
            subject.user != ${builtins.toJSON cfg.user} ||
            ${builtins.toJSON units}.indexOf(action.lookup("unit")) < 0) {
          return polkit.Result.NOT_HANDLED;
        }
        if (action.lookup("polkit.message") == ${builtins.toJSON transientMessage}) {
          return polkit.Result.NO;
        }
        var messages = ${builtins.toJSON jobMessages};
        var verb = action.lookup("verb");
        if (messages.hasOwnProperty(verb) &&
            action.lookup("polkit.message") == messages[verb]) {
          return polkit.Result.YES;
        }
        return polkit.Result.NOT_HANDLED;
      });
    '';
  };
}
