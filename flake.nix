{
  description = "Grace Bobber's Website";

  inputs = {
    unstable.url = "github:NixOS/nixpkgs/nixos-unstable";
    stable.url = "github:NixOS/nixpkgs/nixos-20.09";
  };

  outputs = inputs:
    let
      system = "x86_64-linux";
      pkgs = inputs.unstable.legacyPackages.${system};
      nodeEnv = pkgs.callPackage ./default.nix { };
    in {
      devShell."${system}" = pkgs.mkShell {
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
          exiftool -title="Grace Bobber Resume" images/Grace-Bobber-Resume.pdf
        '';
      };
    };
}
