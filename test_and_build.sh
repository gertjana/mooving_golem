#!/bin/bash
cd mooving
cargo fmt
cargo test
cargo clippy
cd ..

cd wasm
cargo fmt
cargo component build --release
cargo component clippy
cd ..

cp wasm/target/wasm32-wasi/release/mooving.wasm ../mooving.wasm
