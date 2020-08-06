with import <nixpkgs> {};

pkgs.mkShell {
    buildInputs = [ stdenv.cc pkgconfig nodejs-14_x ];
}
