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
        hyperfine
        cargo-flamegraph
    ];
    buildInputs = [];

    RUST_BACKTRACE = 1;
}
