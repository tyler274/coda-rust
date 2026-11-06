# The Rust toolchain from rust-toolchain.toml, for the packages and dev shells.
# Its `components` line is for rustup, which installs only listed components
# when the toolchain already exists. The default profile already includes
# them, and passing them here would change every cached dependency derivation.
pkgs:
let
  pin = (pkgs.lib.importTOML ../rust-toolchain.toml).toolchain;
in
pkgs.rust-bin.fromRustupToolchain { inherit (pin) channel profile targets; }
