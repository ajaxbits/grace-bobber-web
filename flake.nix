{
  description = "Grace Bobber's Website";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";
    flake-utils.inputs.nixpkgs.follows = "nixpkgs";
    gitignore.url = "github:hercules-ci/gitignore.nix";
    gitignore.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, gitignore, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        nodeEnv = pkgs.callPackage ./nix { };
        revision = "${self.lastModifiedDate}-${self.shortRev or "dirty"}";
        inherit (gitignore.lib) gitignoreSource;

        htmlSource = pkgs.stdenv.mkDerivation {
          name = "gracebobberdotcom";
          src = gitignoreSource ./.;
          preferLocalBuild = true;
          buildInputs = [ pkgs.exiftool ];
          buildPhase = ''
            exiftool -Title="Grace Bobber Resume" -q -P -overwrite_original_in_place ./images/Grace-Bobber-Resume.pdf
          '';
          installPhase = ''
            mkdir $out
            cp -r ./* $out/
          '';
        };

        node2nix = with pkgs; writeShellScriptBin "node2nix" ''
          ${nodePackages.node2nix}/bin/node2nix \
            --development \
            -l package-lock.json \
            -c ./nix/default.nix \
            -o ./nix/node-packages.nix \
            -e ./nix/node-env.nix
        '';


      in
      rec {
        packages = {
          ociImage = pkgs.dockerTools.buildLayeredImage {
            name = "gracebobber.com";
            contents = [ pkgs.python3Minimal htmlSource ];
            tag = revision;
            config = {
              Cmd = [
                "${pkgs.python3Minimal}/bin/python"
                "-m"
                "http.server"
                "8000"
                "--directory"
                "${htmlSource}"
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

              rust-bin.stable."1.58.0".default
              openssl
              pkgconfig
              cargo-edit
              cargo-watch
            ];
          shellHook = ''
            export NODE_PATH=${nodeEnv.shell.nodeDependencies}/lib/node_modules
          '';
        };
      });
}
