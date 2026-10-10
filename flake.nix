{
  description = "Reproducible shell for the Coda Rust client workspace";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      postgres = pkgs.postgresql_16;
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          rustc
          cargo
          rustfmt
          clippy
          lld
          pkg-config
          openssl
          dioxus-cli
          openapi-generator-cli
          postgres
          jq
        ];

        env.OPENSSL_NO_VENDOR = "1";
        env.PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

        shellHook = ''
          export CODA_TEST_POSTGRES_BIN="${postgres}/bin"
        '';
      };
    };
}
