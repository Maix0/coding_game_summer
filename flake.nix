{
  description = "Flake utils demo";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    cglocal-jar-raw = {
      url = "https://github.com/jmerle/cg-local-app/releases/download/1.3.0/cg-local-app-1.3.0.jar";
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    cglocal-jar-raw,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [(import rust-overlay)];
        };
        cglocal-jar = pkgs.stdenv.mkDerivation {
          name = "cglocal-jar";
          src = cglocal-jar-raw;
          dontUnpack = true;
          doBuild = false;
          doConfigure = false;
          installPhase = ''
            mkdir -p "$out"
            cp ${cglocal-jar-raw} "$out/cglocal.jar"
          '';
        };
        cglocal = pkgs.writeShellApplication {
          name = "cglocal";
          text = ''
            ${pkgs.jre}/bin/java -jar "${cglocal-jar}/cglocal.jar" "$@" >/dev/null 2>/dev/null &
          '';
        };
      in {
        devShell = pkgs.mkShell {
          packages = with pkgs; [
            rust-bin.stable.latest.default
            cglocal
          ];
        };
      }
    );
}
