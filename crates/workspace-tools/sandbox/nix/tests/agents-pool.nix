# Test fixture: the late-import unit for an encrypted `agents` ZFS pool, as
# a host declares it (on Cyrene, hosts/cyrene/zfs/agents.nix in the nixos
# repository). The module never creates or imports a pool; a host passes
# this unit in services.coda-sandbox.requires.
#
# The pool must never delay or fail boot, so it is not in
# boot.zfs.extraPools and has no fileSystems entries. If the disk or the key
# is missing, only this unit and the units that require it fail.
{ disk }:
{ config, pkgs, ... }:
let
  keyFile = "/etc/zfs/keys/agents-coda.key";
  mounts = {
    "agents/coda/src" = "/agents/coda/src";
    "agents/coda/build" = "/agents/coda/build";
    "agents/coda/ci" = "/agents/coda/ci";
  };
in
{
  systemd.services.agents-pool = {
    description = "Import, unlock and mount the agents ZFS pool";
    wantedBy = [ "multi-user.target" ];
    after = [
      "zfs-import.target"
      "zfs-mount.service"
      "local-fs.target"
    ];
    unitConfig.ConditionPathExists = disk;
    path = [
      config.boot.zfs.package
      pkgs.util-linux
    ];
    serviceConfig = {
      Type = "oneshot";
      RemainAfterExit = true;
      TimeoutStartSec = "90s";
    };
    script = ''
      if ! zpool list -H agents >/dev/null 2>&1; then
        zpool import -N -d "$(dirname ${disk})" -o cachefile=none agents
      fi
      if [ "$(zfs get -H -o value keystatus agents/coda)" != available ]; then
        zfs load-key -L file://${keyFile} agents/coda
      fi
    ''
    + pkgs.lib.concatStrings (
      pkgs.lib.mapAttrsToList (dataset: mountpoint: ''
        if [ "$(findmnt -rn -M ${mountpoint} -o SOURCE)" != ${dataset} ]; then
          zfs mount ${dataset}
        fi
        test "$(findmnt -rn -M ${mountpoint} -o SOURCE)" = ${dataset}
      '') mounts
    );
  };
}
