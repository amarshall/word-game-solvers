{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11";
    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, devshell, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ devshell.overlay ];
      };
    in {
      devShell = pkgs.devshell.mkShell {
        motd = "";
        devshell.packages = [
          pkgs.cargo
          pkgs.clippy
          pkgs.oniguruma
          pkgs.pkg-config
          pkgs.rust-analyzer
          pkgs.rustfmt
        ];
        env = let
          wordlist = pkgs.runCommand "words.txt" {} ''
            ${pkgs.glibc.bin}/bin/iconv -f iso8859-1 -t utf-8 ${pkgs.scowl}/share/dict/words.txt > $out
          '';
        in [
          { name = "LIBCLANG_PATH"; value = "${pkgs.llvmPackages.libclang.lib}/lib"; }
          { name = "RUSTONIG_SYSTEM_LIBONIG"; value = "y"; }
          { name = "WORDLIST"; value = "${wordlist}"; }
        ];
      };
    });
}
