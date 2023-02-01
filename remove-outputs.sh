#!/bin/sh

# Only clears contract output folders

set -e
SMART_CONTRACT_JSONS=$(find . -name "multiversx.json")
for smart_contract_json in $SMART_CONTRACT_JSONS
do
    smart_contract_folder=$(dirname $smart_contract_json)
    rm -rf $smart_contract_folder/output
done
