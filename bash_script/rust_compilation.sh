#!/bin/sh
cd gomoku_rust_lib
cargo make --makefile Makefile.toml build
RET=$?
if [ $RET != 0 ]
then
	exit 1
else
	cp target/debug/gomoku_rust.dll ../
	cd ../
	mv gomoku_rust.dll gomoku_rust.pyd
	exit 0
fi