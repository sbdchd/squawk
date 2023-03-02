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

            # The query parser produces a slightly different AST between major versions
            # and Squawk is not capable of handling >=14 correctly yet.
            libpg_query13 = final.libpg_query.overrideAttrs (_: rec {
              version = "13-2.1.0";
              src = final.fetchFromGitHub {
                owner = "pganalyze";
                repo = "libpg_query";
                rev = version;
                hash = "sha256-DpvPmBvpx5pWDlx6T3Kp82ALi6FjOO549Exd8tWXDIk=";
              };
            });
          in
          {
            squawk = final.rustPlatform.buildRustPackage {
              pname = "squawk";
              version = "0.21.0";

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

              LIBPG_QUERY_PATH = libpg_query13;

              meta = with lib; {
                description = "Linter for PostgreSQL, focused on migrations";
                homepage = "https://github.com/sbdchd/squawk";
                license = licenses.gpl3Only;
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
