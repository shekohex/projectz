{
  description = "Game development environment using Bevy Engine";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    # Rust
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        lib = pkgs.lib;
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      {
        devShells.default = pkgs.mkShell {
          name = "projectz";
          nativeBuildInputs = [
            pkgs.pkg-config
            pkgs.clang
            pkgs.libclang.lib
            pkgs.rustPlatform.bindgenHook
            # Mold Linker for faster builds (only on Linux)
            (lib.optionals pkgs.stdenv.isLinux pkgs.mold)
            (lib.optionals pkgs.stdenv.isLinux pkgs.om4)
            # lld Linker for faster builds (only on Darwin)
            (lib.optionals pkgs.stdenv.isDarwin pkgs.lld)
            (lib.optionals pkgs.stdenv.isDarwin pkgs.darwin.apple_sdk.frameworks.SystemConfiguration)
            (lib.optionals pkgs.stdenv.isDarwin pkgs.darwin.apple_sdk.frameworks.AudioUnit)
            (lib.optionals pkgs.stdenv.isDarwin pkgs.darwin.apple_sdk.frameworks.CoreAudio)
            (lib.optionals pkgs.stdenv.isDarwin pkgs.darwin.apple_sdk.frameworks.AppKit)
          ];
          buildInputs = [
            # We want the unwrapped version, wrapped comes with nixpkgs' toolchain
            pkgs.rust-analyzer-unwrapped
            # Nix language server
            pkgs.nixd
            # Finally the toolchain
            toolchain
            pkgs.taplo
          ];
          packages = [
            pkgs.cargo-nextest
            pkgs.cargo-machete
            pkgs.cargo-expand
            pkgs.cargo-watch
            pkgs.just
          ];
          # Environment variables
          RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
          LD_LIBRARY_PATH = lib.makeLibraryPath [ pkgs.libclang pkgs.openssl.dev pkgs.stdenv.cc.cc ];
          RUST_LOG = builtins.concatStringsSep "," [
            "wgpu=error"
            "bevy_render=info"
            "bevy_ecs=info"
            "project_z=trace"
          ];
        };
      });
}
