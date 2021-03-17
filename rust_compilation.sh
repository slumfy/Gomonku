#!/bin/sh
cd gomoku_rust_lib
cargo make --makefile Makefile.toml build
cp target/debug/gomoku_rust.dll ../
cd ../
mv gomoku_rust.dll gomoku_rust.pyd