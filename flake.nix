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
          # libpg_query has systems = x86_64, which is probably a bug
          config.allowUnsupportedSystem = true;
          overlays = [ overlay ];
        };
        overlay = (final: prev:
          let inherit (prev) lib;
          in
          {
            squawk = final.rustPlatform.buildRustPackage {
              pname = "squawk";
              version = "0.13.2";

              cargoLock = {
                lockFile = ./Cargo.lock;
                outputHashes = {
                  "libpg_query-sys-0.2.0" = "sha256-Txllwr/aZFrrRC+Me+ofX/B9l7JpX6MBLEs2NX2FFqw=";
                };
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

              LIBPG_QUERY = final.libpg_query;

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
        });
      });
}
