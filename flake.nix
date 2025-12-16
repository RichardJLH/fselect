{
  description = "fselect - a TUI fuzzy selector";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      packages.default = pkgs.rustPlatform.buildRustPackage {
        pname = "fselect";
        version = "0.1.0";

        src = self;

        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        # Optional but recommended metadata
        meta = with pkgs.lib; {
          description = "A simple TUI fuzzy selector";
          license = licenses.mit; # adjust if needed
          mainProgram = "fselect";
        };
      };
    });
}
