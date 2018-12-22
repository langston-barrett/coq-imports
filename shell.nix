{ pkgs ? import <nixpkgs> { } }:

with pkgs; (import ./default.nix { }).overrideAttrs (oldAttrs: {
  buildInputs = oldAttrs.buildInputs ++ [ emacs git racer zsh ];
})
