with (import <nixpkgs> { });
mkShell {
  buildInputs = [
    rustup
    clang
    pkgconfig
    openssl
    cmake
    fontconfig
  ];
}
