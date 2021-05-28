#!/bin/sh
cd gomoku_rust_lib
cargo make --makefile Makefile.toml build-release
RET=$?
if [ $RET != 0 ]
then
	exit 1
else
	cp target/release/gomoku_rust.dll ../
	cd ../
	mv gomoku_rust.dll gomoku_rust.pyd
	exit 0
fi