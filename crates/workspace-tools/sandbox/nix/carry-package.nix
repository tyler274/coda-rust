# `coda-carry`: bundle the host's coda-rust worktrees for the sandbox
# (crates/workspace-tools/sandbox/README.md, "Bringing host worktrees over"). The module's values
# become the script's defaults; the CODA_CARRY_* variables still override them.
{
  lib,
  writeShellApplication,
  git,
  coreutils,
  gnugrep,
  gnused,
  util-linux,
  user ? null,
  repo ? null,
  out ? null,
}:
let
  default =
    var: value:
    lib.optionalString (value != null) ''
      ${var}=''${${var}:-${lib.escapeShellArg value}}
    '';
in
writeShellApplication {
  name = "coda-carry";
  runtimeInputs = [
    git
    coreutils
    gnugrep
    gnused
    util-linux
  ];
  text =
    default "CODA_CARRY_USER" user
    + default "CODA_CARRY_REPO" repo
    + default "CODA_CARRY_OUT" out
    + builtins.readFile ./coda-carry.sh;
}
