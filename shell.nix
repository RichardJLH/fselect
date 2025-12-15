{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  name = "rust-tui-dev-shell";

  buildInputs = with pkgs; [
    cargo
    clippy
    rustfmt
    python3
  ];
}
