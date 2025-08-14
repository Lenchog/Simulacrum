{
  description = "Rust flake with nightly for bevy";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      flake-utils,
      rust-overlay,
      nixpkgs,
      naersk,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = (import nixpkgs) { inherit system overlays; };
        dioxus-cli = pkgs.callPackage ./dioxus.nix { };
        naerskLib = pkgs.callPackage naersk { };
        dependencies = with pkgs; [
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
          dioxus-cli
        ];
        rust = pkgs.rust-bin.selectLatestNightlyWith (
          toolchain:
          toolchain.default.override {
            extensions = [
              "rustc-codegen-cranelift-preview"
              "rust-src"
              "rust-analyzer"
            ];
            targets = [
              "x86_64-pc-windows-msvc"
              "wasm32-unknown-unknown"
            ];
          }
        );
      in
      with pkgs;
      {
        devShells.default = mkShell rec {
          buildInputs = dependencies;
          nativeBuildInputs = [ rust ];
          LD_LIBRARY_PATH = lib.makeLibraryPath dependencies;
          BEVY_ASSET_ROOT = ".";
        };
        packages.default =
          (naerskLib.override {
            cargo = rust;
            rustc = rust;
          }).buildPackage
            {
              src = ./.;
              buildInputs = dependencies;
              nativeBuildInputs = [ rust ];
              cargoBuildOptions = default: default ++ [ "--no-default-features" ];
            };
      }
    );
}
