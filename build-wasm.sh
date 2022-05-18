#!/bin/sh

# builds all wasm targets

set -e
SMART_CONTRACT_JSONS=$(find . -name "elrond.json")
CARGO_TARGET_DIR=$(realpath ./target)
for smart_contract_json in $SMART_CONTRACT_JSONS
do
    smart_contract_folder=$(dirname $smart_contract_json)
    echo ""
    (set -x; erdpy --verbose contract build --cargo-target-dir=${CARGO_TARGET_DIR} --ignore-eei-checks $smart_contract_folder)
done
