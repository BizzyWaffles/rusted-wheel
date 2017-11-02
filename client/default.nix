{ mkDerivation, base, bizzlelude, containers, ghcjs-base, ghcjs-dom, reflex-dom, stdenv, text, transformers }:
mkDerivation {
  pname = "rusted-wheel";
  version = "0.0.1";
  src = ./src;
  isLibrary = true;
  isExecutable = true;
  buildDepends = [
    base
    bizzlelude
    containers
    ghcjs-base
    ghcjs-dom
    reflex-dom
    text
    transformers
  ];
  postInstall = stdenv.lib.optionalString (ghc.isGhcjs or false) ''
    rm "$out/bin/client" || true # This is not designed to be run from node, so don't let it be
  '';
  license = stdenv.lib.licenses.bsd3;
}
