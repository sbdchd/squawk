{
  description = "Linter for PostgreSQL, focused on migrations";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ overlay ];
        };
        overlay = (final: prev:
          let
            inherit (prev) lib;
          in
          {
            squawk = final.rustPlatform.buildRustPackage {
              pname = "squawk";
              version = "2.68.0";

              cargoLock = {
                lockFile = ./Cargo.lock;
              };

              src = ./.;

              nativeBuildInputs = with final; [
                pkg-config
                rustPlatform.bindgenHook
              ];

              buildInputs = with final; [
                libiconv
                openssl
              ];

              meta = with lib; {
                description = "Linter for PostgreSQL, focused on migrations";
                homepage = "https://github.com/sbdchd/squawk";
                license = with licenses; [ asl20 mit ];
                platforms = platforms.all;
              };
            };
          });
      in
      {
        packages = {
          default = pkgs.squawk;
          squawk = pkgs.squawk;
        };
        checks.squawk = pkgs.squawk;

        devShells.default = pkgs.mkShell {
          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
          packages = with pkgs; [
            cargo
            rustc
            rustfmt
            clippy
            cargo-insta
            pkg-config
            rustPlatform.bindgenHook
            libiconv
            openssl
          ];
        };
      });
}
