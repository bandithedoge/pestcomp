{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    naersk,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in rec {
      packages.default = naersk.lib.${system}.buildPackage {
        pname = "pestcomp";
        root = ./.;

        nativeBuildInputs = with pkgs; [rustc cargo pkg-config python3];
        buildInputs = with pkgs; [libGL xorg.libX11 xorg.libXcursor xorg.xcbutilwm];

        copyBins = false;
        copyLibs = true;
      };
      devshell = pkgs.mkShell {
        inherit (packages.default) nativeBuildInputs buildInputs;
      };
    });
}
