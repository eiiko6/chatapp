{
  description = "chatapp monorepo";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.stable.latest.default;
        openssl = pkgs.openssl;
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "chatapp";
          version = "1.0.0";

          # Point to the backend subdirectory
          src = ./backend;
          cargoLock = {
            lockFile = ./backend/Cargo.lock;
          };

          nativeBuildInputs = [ rust pkgs.pkg-config ];
          buildInputs = [ openssl ];

          OPENSSL_LIB_DIR = "${openssl.out}/lib";
          OPENSSL_INCLUDE_DIR = "${openssl.dev}/include";
          PKG_CONFIG_PATH = "${openssl.dev}/lib/pkgconfig";
        };

        devShells.default = pkgs.mkShell {
          packages = [ rust pkgs.cargo pkgs.rust-analyzer pkgs.pkg-config openssl ];
          shellHook = ''
            export OPENSSL_DIR=${openssl.dev}
            export PKG_CONFIG_PATH=${openssl.dev}/lib/pkgconfig
          '';
        };
      });
}
