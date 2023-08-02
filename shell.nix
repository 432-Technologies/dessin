with (import <nixpkgs> { });
mkShell {
  buildInputs = [
    rustup
    clang
    pkg-config
    openssl
    cmake
    fontconfig
  ];
}
