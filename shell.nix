{ sources ? import ./nix/sources.nix
, pkgs ? import ./nix { inherit sources; } }:

pkgs.mkShell {
  name = "snowflake-shell";

  buildInputs = with pkgs; [
    latest.rustChannels.nightly.rust
    niv
  ];
}
