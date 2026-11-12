# The token broker: a `systemd --user` service of services.coda-sandbox.user
# that hands the agent container its tokens over a Unix socket. It reads
# them from the user's Secret Service keyring with `secret-tool`, so no
# token sits in a file; `coda-sandbox broker store <item>` puts them there.
#
# Each socket's directory is made by root, owned by the user and grouped to
# the one caller it is for: the container's `users` group for the agent
# socket, which container.nix binds into the container at the same path,
# and `coda-ci` for the CI socket, where the controller of ci.nix fetches
# its GitHub token. The directory gates traversal; the broker's SO_PEERCRED
# check gates the rest.
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
  ciSocketDir = builtins.dirOf cfg.broker.ciSocket;
  doorDirs = [ socketDir ] ++ lib.optional cfg.ci.enable ciSocketDir;
  # tmpfiles makes the directories in the system manager, which a user unit
  # cannot order itself after; on the switch that first enables the broker
  # it can start before they exist.
  waitForDoors = pkgs.writeShellScript "coda-sandbox-broker-doors" ''
    for dir in ${lib.escapeShellArgs doorDirs}; do
      tries=0
      until [ -d "$dir" ]; do
        tries=$((tries + 1))
        if [ "$tries" -gt 100 ]; then
          echo "$dir is missing; systemd-tmpfiles makes it" >&2
          exit 1
        fi
        ${pkgs.coreutils}/bin/sleep 0.1
      done
    done
  '';
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

    ciSocket = lib.mkOption {
      type = lib.types.str;
      default = "/run/coda-sandbox/ci/broker.sock";
      readOnly = true;
      description = "The CI socket, served while `ci.enable` is set and reachable only by `coda-ci`.";
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
    ]
    ++ lib.optional cfg.ci.enable "d ${ciSocketDir} 0750 ${cfg.user} coda-ci -";

    systemd.user.services.coda-sandbox-broker = {
      description = "Coda sandbox token broker";
      wantedBy = [ "default.target" ];
      unitConfig.ConditionUser = cfg.user;
      path = [
        cfg.broker.keyringPackage
        pkgs.gh
      ];
      serviceConfig = {
        ExecStartPre = waitForDoors;
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
