#!/bin/bash
set -e

CONFIG=release

rustup target add wasm32-unknown-unknown

if [ -z "$(cargo install --list | grep wasm-pack)" ]
then
	cargo install wasm-pack
fi

if [ "${CONFIG}" = "release" ]
then
    wasm-pack build
else 
    wasm-pack build --release
fi
