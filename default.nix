with import <nixpkgs> {};

pkgs.mkShell {
    buildInputs = [ stdenv.cc pkgconfig ];
}
