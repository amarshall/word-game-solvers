{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
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
        overlays = [ devshell.overlays.default ];
      };
    in {
      devShells.default = pkgs.devshell.mkShell {
        motd = "";
        devshell.packages = [
          pkgs.cargo
          pkgs.clippy
          pkgs.gcc
          pkgs.oniguruma.dev
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
          { name = "PKG_CONFIG_PATH"; eval = "$DEVSHELL_DIR/lib/pkgconfig"; }
          { name = "RUSTONIG_SYSTEM_LIBONIG"; value = "1"; }
          { name = "WORDLIST"; value = "${wordlist}"; }
        ];
      };
    });
}
