#!/bin/bash

## To use the complete wat build, install wasm2wat and wasm2c and add to PATH.

CONTRACT_PATH=$1

CONTRACT_NAME=${CONTRACT_PATH##*/}


rm -f ${CONTRACT_PATH}/output/${CONTRACT_NAME}.wasm
rm -f ${CONTRACT_PATH}/output/${CONTRACT_NAME}-dbg.wasm
rm -f ${CONTRACT_PATH}/output/${CONTRACT_NAME}-dbg.c
rm -f ${CONTRACT_PATH}/output/${CONTRACT_NAME}-dbg.wat

erdpy contract build "${CONTRACT_PATH}" 
erdpy contract build "${CONTRACT_PATH}" --wasm-symbols --wasm-name "${CONTRACT_NAME}-dbg.wasm"
wasm2wat \
    ${CONTRACT_PATH}/output/${CONTRACT_NAME}-dbg.wasm \
    -o \
    ${CONTRACT_PATH}/output/${CONTRACT_NAME}-dbg.wat

wasm2c \
    ${CONTRACT_PATH}/output/${CONTRACT_NAME}-dbg.wasm \
    -o \
    ${CONTRACT_PATH}/output/${CONTRACT_NAME}-dbg.c
