{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    pkg-config
    cargo
    rustc
    gtk4
    gtk4-layer-shell
    gcc
  ];
}