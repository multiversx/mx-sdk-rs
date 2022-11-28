#!/bin/bash

# builds all wasm targets

root=$(pwd)

set -e
SMART_CONTRACT_JSONS=$(find . -name "elrond.json")
for smart_contract_json in $SMART_CONTRACT_JSONS
do
    cd $root
    smart_contract_folder=$(dirname $smart_contract_json)
    meta_folder="$smart_contract_folder/meta"

    if [ -f "$meta_folder/src/main.rs" ]; then
        echo "$meta_folder running ..."
        cd $meta_folder
        cargo run
    else
        echo "$meta_folder MISSING!!!!!."
    fi

done
