with import <nixpkgs> {};

stdenv.mkDerivation {
    name="rust-env";
    nativeBuildInputs = [
        rustc
        cargo
        clippy
        rustfmt
        bacon
        pkg-config
        openssl
    ];
    buildInputs = [];

    RUST_BACKTRACE = 1;
}
