#!/bin/sh
set -e # exit on error
set -x # show commands executed
cargo build --all-features
cargo test --all-features
cargo clippy --all-features -- -D warnings
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --no-deps --all-features
cargo +nightly fmt
cargo rdme --force
