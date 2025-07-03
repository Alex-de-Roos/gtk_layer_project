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

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

}