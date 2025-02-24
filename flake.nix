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
      in
      with pkgs;
      {
        devShells.default = mkShell.override { stdenv = stdenvNoCC; } {
          buildInputs = [
            dioxus-cli
            rust
            tailwindcss
            vscode
            wasm-bindgen-cli
          ];
          shellHook = ''
            alias tw="tailwindcss -i ./input.css -o ./assets/tailwind.css --watch"
            alias clippy="cargo clippy --target wasm32-unknown-unknown"

            echo "Welcome to the dioxus-cli development environment!"
            echo "To start the tailwindcss watcher, run 'tw'"
            echo "To serve the website, run 'dx serve'"
            echo "To launch clippy, run 'clippy'"
          '';
        };
      }
    );
}
