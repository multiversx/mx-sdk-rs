#!/bin/bash

## To use the complete wat build, install wasm2wat and wasm2c and add to PATH.

CONTRACT_PATH=$1

CONTRACT_NAME=${CONTRACT_PATH##*/}

rm -f ${CONTRACT_PATH}/output/${CONTRACT_NAME}.wasm
rm -f ${CONTRACT_PATH}/output/${CONTRACT_NAME}-dbg.wasm
rm -f ${CONTRACT_PATH}/output/${CONTRACT_NAME}-dbg.c
rm -f ${CONTRACT_PATH}/output/${CONTRACT_NAME}-dbg.wat

cd ${CONTRACT_PATH}/meta
cargo run build
cargo run build --wasm-symbols --wasm-name "${CONTRACT_NAME}-dbg.wasm"

cd ../output
wasm2wat \
    ${CONTRACT_NAME}-dbg.wasm \
    -o \
    ${CONTRACT_NAME}-dbg.wat

wasm2c \
    ${CONTRACT_NAME}-dbg.wasm \
    -o \
    ${CONTRACT_NAME}-dbg.c

# Twiggy helps us investigate where the size/functions come from
twiggy top -n 200 ${CONTRACT_NAME}-dbg.wasm > twiggy-top-${CONTRACT_NAME}.txt
twiggy paths ${CONTRACT_NAME}-dbg.wasm > twiggy-paths-${CONTRACT_NAME}.txt
twiggy monos ${CONTRACT_NAME}-dbg.wasm > twiggy-monos-${CONTRACT_NAME}.txt
twiggy dominators ${CONTRACT_NAME}-dbg.wasm > twiggy-dominators-${CONTRACT_NAME}.txt
