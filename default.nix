with import <nixpkgs> {};

pkgs.mkShell {
    buildInputs = [ stdenv.cc pkgconfig nodejs-13_x sqlite ];
}
