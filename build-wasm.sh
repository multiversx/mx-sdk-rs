#!/bin/sh

# builds all wasm targets

ROOT=$(pwd)

SMART_CONTRACT_JSONS=$(find . -name "elrond.json")
for smart_contract_json in $SMART_CONTRACT_JSONS
do
    cd $ROOT
    smart_contract_folder=$(dirname $smart_contract_json)
    cd $smart_contract_folder/meta
    cargo run build
done
