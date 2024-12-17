{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    avrdude
    pkgsCross.avr.buildPackages.gcc
    ravedude
    rustup
    cargo-generate
  ];
}
