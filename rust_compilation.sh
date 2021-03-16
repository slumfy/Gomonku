#!/bin/sh
cd gomoku_rust_lib
cargo build --release
cp target/release/gomoku_rust.dll ../
cd ../
mv gomoku_rust.dll gomoku_rust.pyd