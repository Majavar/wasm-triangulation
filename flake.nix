{
  description = "Delaunay and Voronoi";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config = {allowUnfree = true;};
        };
        rust = pkgs.rust-bin.stable."1.85.0".minimal.override {
          extensions = [ "rustfmt" "clippy" "rust-src" "rust-analyzer" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
        rustPlatform = pkgs.makeRustPlatform { cargo = rust; rustc = rust; };
        dioxus-cli = rustPlatform.buildRustPackage rec {
          pname = "dioxus-cli";
          version = "0.6.2";
          src = pkgs.fetchCrate {
            inherit pname version;
            sha256 = "sha256-jUS/it2N5o5D7Jon0fKHWEt3f0wdtVgNIkqSNc7u830=";
          };
          cargoLock = {
            lockFileContents = (builtins.readFile ./.nix/dioxus-cli.lock);
          };
          postPatch = ''
            rm Cargo.lock
            ln -s ${./.nix/dioxus-cli.lock} Cargo.lock
          '';

          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [
            pkgs.cacert
            pkgs.openssl
          ];

          checkFlags = [
          # requires network access, thanks nixpkgs for figuring this out
            "--skip=serve::proxy::test"
            "--skip=wasm_bindgen::test"
          ];

          # Tell openssl-sys to use the system's provided openssl.
          OPENSSL_NO_VENDOR = 1;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell.override { stdenv = stdenvNoCC; } {
          buildInputs = [
            openssl
            pkg-config
            dioxus-cli
            rust
            tailwindcss
            vscode
          ];
          TMPDIR = "/tmp";
        };
      }
    );
}
