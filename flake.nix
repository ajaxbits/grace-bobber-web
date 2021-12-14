{
  description = "Grace Bobber's Website";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        nodeEnv = pkgs.callPackage ./default.nix { };
        revision = "${self.lastModifiedDate}-${self.shortRev or "dirty"}";
      in rec {
        # packages = {
        #   ociImage = pkgs.dockerTools.buildLayeredImage {
        #     name = "gracebobber.com";
        #     contents = [ pkgs.python3Minimal ];
        #     tag = revision;
        #     config = {
        #       Cmd = [
        #         "${pkgs.python3Minimal}/bin/python"
        #         "-m"
        #         "http.server"
        #         "8000"
        #       ];
        #     };
        #   };
        # };

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
