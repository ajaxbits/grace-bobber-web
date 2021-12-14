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
      in rec {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; with nodePackages; [
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
      }
    );
}
