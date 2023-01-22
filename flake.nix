{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs { inherit system; };
    in {
      devShell = pkgs.mkShell {
        packages = [
          pkgs.cargo
          pkgs.clippy
          pkgs.oniguruma
          pkgs.pkg-config
          pkgs.rust-analyzer
          pkgs.rustfmt
        ];
        shellHook = let
          wordlist = pkgs.runCommand "words.txt" {} ''
            ${pkgs.glibc.bin}/bin/iconv -f iso8859-1 -t utf-8 ${pkgs.scowl}/share/dict/words.txt > $out
          '';
        in ''
          export LIBCLANG_PATH=${pkgs.llvmPackages.libclang.lib}/lib
          export RUSTONIG_SYSTEM_LIBONIG=y
          export WORDLIST=${wordlist}
        '';
      };
    });
}
