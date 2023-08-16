#!/bin/bash

# format test and lint mooving app
cd mooving
cargo fmt
cargo test
cargo clippy
cd ..

# format, lint and build the webassemnly module
cd wasm
cargo fmt
cargo component clippy
cargo component build --release
cd ..

cp wasm/target/wasm32-wasi/release/mooving.wasm mooving.wasm
