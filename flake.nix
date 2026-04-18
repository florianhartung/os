{
  description = "TODO";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    devshell.url = "github:numtide/devshell";
    utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    {
      self,
      nixpkgs,
      devshell,
      utils,
      rust-overlay,
      treefmt-nix,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [
          devshell.overlays.default
          (import rust-overlay)
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        # Target for current system
        # rust-target = pkgs.pkgsStatic.stdenv.targetPlatform.rust.rustcTarget;
        rust-toolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
          targets = [
            "x86_64-unknown-none"
          ];
        };
        treefmtEval = treefmt-nix.lib.evalModule pkgs ./treefmt.nix;

      in
      {
        git.hooks = {
          enable = true;
          pre-commit.text = "nix flake check";
        };

        devShells.default = (
          pkgs.devshell.mkShell {
            name = "dev";
            packages = with pkgs; [
              stdenv.cc
              coreutils
              gnumake

              nasm
              grub2
              xorriso
              qemu

              rust-toolchain
              rust-analyzer
              cargo-expand
              cargo-show-asm
              cargo-outdated
              cargo-bootimage
            ];
          }
        );

        formatter = treefmtEval.config.build.wrapper;

        checks = {
          formatting = treefmtEval.config.build.check self;
        };
      }
    );
  # });
}
