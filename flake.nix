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

        images_folder = "images";
        dist_images_folder = "dist/images";
        thumbnails_folder = "thumbnails";

        optimizejpg = pkgs.writeShellScriptBin "optimizejpg" ''
          for img in $(${pkgs.fd}/bin/fd -e jpg -e jpeg . ${images_folder});
          do
                  ${pkgs.jpegoptim}/bin/jpegoptim --dest="${dist_images_folder}" \
                          --overwrite \
                          --all-progressive \
                          --strip-all \
                          --max=90 \
                          --force \
                          $img
                  ${pkgs.jpegoptim}/bin/jpegoptim --dest=${thumbnails_folder} \
                          --overwrite \
                          --all-progressive \
                          --strip-all \
                          --size=10% \
                          --force \
                          $img
          done
        '';

        optimizepng = pkgs.writeShellScriptBin "optimizepng" ''
          for img in $( ${pkgs.fd}/bin/fd -e png . ${images_folder} );
          do
                  ${pkgs.optipng}/bin/optipng $img -dir ${dist_images_folder} -clobber
                  ${pkgs.optipng}/bin/optipng $img -dir ${thumbnails_folder} -o 7 -clobber
          done
        '';


      in
      rec {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs;
            with nodePackages; [
              nodejs
              fd
              exiftool
              nodePackages.prettier
              netlify-cli

              optimizepng
              optimizejpg

              rust-bin.stable."1.58.0".default
              openssl
              pkgconfig
              cargo-edit
              cargo-watch
            ];
        };
      });
}
