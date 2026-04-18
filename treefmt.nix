{ ... }:
{
  # Used to find the project root
  projectRootFile = "flake.nix";

  programs.mdformat.enable = true;
  programs.nixfmt.enable = true;
  programs.rustfmt = {
    enable = true;
    edition = "2024";
  };
  programs.taplo.enable = true; # toml
}
