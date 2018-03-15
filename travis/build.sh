#!/bin/bash
set -ev

cargo build
cargo test

if [ "${TRAVIS_RUST_VERSION}" = "stable" ]; then
    rustup component add rustfmt-preview
    cargo install --force cargo-travis

    cargo fmt --all -- --write-mode=diff
    cargo coveralls
fi
