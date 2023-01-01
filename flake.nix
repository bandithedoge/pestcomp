{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in rec {
      packages.default = pkgs.stdenv.mkDerivation {
        pname = "PestComp";
        version = "2.0.0";
        src = ./.;

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        buildInputs = with pkgs; (lib.optionals stdenv.targetPlatform.isLinux (with pkgs; [xorg.libX11]));

        makeFlags = ["PREFIX=$(out)"];

        enableParallelBuilding = true;
      };

      devShells.default = pkgs.mkShell {
        inherit (packages.default) buildInputs;
        nativeBuildInputs = packages.default.nativeBuildInputs ++ (with pkgs; [bear clang-tools]);
      };
    });
}
