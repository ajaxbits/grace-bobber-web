# This file has been generated by node2nix 1.9.0. Do not edit!

{nodeEnv, fetchurl, fetchgit, nix-gitignore, stdenv, lib, globalBuildInputs ? []}:

let
  sources = {
    "headroom.js-0.11.0" = {
      name = "headroom.js";
      packageName = "headroom.js";
      version = "0.11.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/headroom.js/-/headroom.js-0.11.0.tgz";
        sha512 = "yI4ciZRD1WH22wa5uJDg2kMtRvhJwUJWo2l41Eby0BoAD+lzXL98lf5jDFxP4Q5W3HmlrpfItSfmqc3jCtasbw==";
      };
    };
  };
  args = {
    name = "grace-website";
    packageName = "grace-website";
    version = "1.0.0";
    src = ./.;
    dependencies = [
      sources."headroom.js-0.11.0"
    ];
    buildInputs = globalBuildInputs;
    meta = {
      description = "Grace Bobber's website";
      homepage = "https://github.com/ajaxbits/grace-bobber-web#readme";
      license = "ISC";
    };
    production = true;
    bypassCache = true;
    reconstructLock = false;
  };
in
{
  args = args;
  sources = sources;
  tarball = nodeEnv.buildNodeSourceDist args;
  package = nodeEnv.buildNodePackage args;
  shell = nodeEnv.buildNodeShell args;
  nodeDependencies = nodeEnv.buildNodeDependencies (lib.overrideExisting args {
    src = stdenv.mkDerivation {
      name = args.name + "-package-json";
      src = nix-gitignore.gitignoreSourcePure [
        "*"
        "!package.json"
        "!package-lock.json"
      ] args.src;
      dontBuild = true;
      installPhase = "mkdir -p $out; cp -r ./* $out;";
    };
  });
}