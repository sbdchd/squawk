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
              version = "2.20.0";

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
              ] ++ lib.optionals final.stdenv.isDarwin (with final.darwin.apple_sdk.frameworks; [
                CoreFoundation
                Security
              ]);

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
          squawk = pkgs.squawk;
        };
        defaultPackage = self.packages.${system}.squawk;
        checks = self.packages;

        # for debugging
        inherit pkgs;

        devShell = pkgs.squawk.overrideAttrs (old: {
          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;

          nativeBuildInputs = old.nativeBuildInputs ++ (with pkgs; [
            cargo-insta
            clippy
            rustfmt
          ]);
        });
      });
}
