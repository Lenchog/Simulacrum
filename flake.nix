{
  description = "Rust flake with nightly for bevy";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, rust-overlay, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = (import nixpkgs) { inherit system overlays; };
      in with pkgs; {
        devShells.default = mkShell rec {
          nativeBuildInputs = [
            (rust-bin.selectLatestNightlyWith (toolchain:
              toolchain.default.override {
                extensions = [
                  "rustc-codegen-cranelift-preview"
                  "rust-src"
                  "rust-analyzer"
                ];
                targets = [ "x86_64-pc-windows-msvc" "wasm32-unknown-unknown" ];
              }))
          ];
          buildInputs = [
            udev
            alsa-lib
            vulkan-loader
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            libxkbcommon
            wayland
            pkg-config
            clang
            mold
            cargo-xwin
            wasm-bindgen-cli
            binaryen
          ];
          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        };
      });
}
