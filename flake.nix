{
  description = "Grace Bobber's Website";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    gitignore.url = "github:hercules-ci/gitignore.nix";
  };

  outputs = { self, nixpkgs, flake-utils, gitignore }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        nodeEnv = pkgs.callPackage ./default.nix { };
        revision = "${self.lastModifiedDate}-${self.shortRev or "dirty"}";
        inherit (gitignore.lib) gitignoreSource;
      in rec {
        packages = with pkgs; {
          gracebobberdotcom = stdenv.mkDerivation {
            name = "gracebobberdotcom";
            src = gitignoreSource ./.;
            buildPhase = ''
              ${pkgs.exiftool}/bin/exiftool -Title="Grace Bobber Resume" ./images/Grace-Bobber-Resume.pdf
            '';
            installPhase = ''
              mkdir $out
              cp -r ./* $out/
            '';
          };

          ociImage = dockerTools.buildLayeredImage {
            name = "gracebobber.com";
            contents = [ pkgs.python3Minimal packages.gracebobberdotcom ];
            tag = revision;
            config = {
              Cmd = [
                "${pkgs.python3Minimal}/bin/python"
                "-m"
                "http.server"
                "8000"
              ];
            };
          };
        };

        devShell = pkgs.mkShell {
          buildInputs = with pkgs;
            with nodePackages; [
              nodejs
              node2nix
              nodeEnv.shell.nodeDependencies
              imagemagick
              libwebp
              jpegoptim
              optipng
              svgo
              fd
              exiftool
            ];
          shellHook = ''
            export NODE_PATH=${nodeEnv.shell.nodeDependencies}/lib/node_modules
          '';
        };
      });
}
