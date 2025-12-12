{
  description = "Rust flake with nightly for bevy";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    fenix.url = "github:nix-community/fenix/staging";
  };

  outputs =
    {
      self,
      flake-utils,
      nixpkgs,
      fenix,
      naersk,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) { inherit system; };
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
        ];
        fenixLib = fenix.packages."x86_64-linux";
        rust = fenixLib.latest.withComponents [
          "cargo"
          "clippy"
          "rust-src"
          "rustfmt"
          "rust-analyzer"
          "rustc-codegen-cranelift-preview"
        ];
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
