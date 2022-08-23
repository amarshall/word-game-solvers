{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, nickel }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          (final: prev: { nickel = nickel.defaultPackage.${system}; })
        ];
      };
    in {
      devShell = pkgs.mkShell {
        packages = [
          pkgs.bashInteractive
          pkgs.cargo
          pkgs.clippy
          pkgs.oniguruma
          pkgs.pkg-config
          pkgs.rust-analyzer
          pkgs.rustfmt
        ];
        shellHook = ''
          export LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib"
          export RUSTONIG_SYSTEM_LIBONIG=y
        '';
      };
    });
}
