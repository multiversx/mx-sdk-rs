#!/bin/sh

# cleans all wasm targets

set -e
SMART_CONTRACT_JSONS=$(find . -name "elrond.json")
for smart_contract_json in $SMART_CONTRACT_JSONS
do
    smart_contract_folder=$(dirname $smart_contract_json)
    echo ""
    (set -x; erdpy --verbose contract clean $smart_contract_folder)
done

# not wasm, but worth cleaning from time to time

cargo clean
cd ..
cd framework/wasm-output
cargo clean
cd ../..
