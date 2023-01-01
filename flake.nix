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

      drv = pkgs':
        pkgs'.stdenv.mkDerivation {
          pname = "PestComp";
          version = "2.0.0";
          src = ./.;

          installPhase = ''
            cp -r bin $out
          '';

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs';
            []
            ++ (pkgs'.lib.optionals pkgs'.stdenv.targetPlatform.isLinux (with pkgs'; [xorg.libX11]))
            ++ (pkgs'.lib.optionals pkgs'.stdenv.targetPlatform.isWindows (with pkgs'; [windows.pthreads]));
        };
    in {
      packages = {
        default = drv pkgs;
        windows = drv pkgs.pkgsCross.mingwW64;
      };

      devShells.default = pkgs.mkShell {
        nativeBuildInputs = (drv pkgs).nativeBuildInputs ++ (with pkgs; [bear clang-tools]);

        inherit (drv pkgs) buildInputs;
      };
    });
}
