#!/bin/sh
cd gomoku_rust_lib
cargo build --release
cp target/release/string_sum.dll ../
cd ../
mv string_sum.dll string_sum.pyd