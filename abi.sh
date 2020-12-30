#!/bin/sh

# creates an ABI for a contract and places it in the /output directory
# this is a temporary script until erdpy can do it instead

cd $1/abi
cargo run > ../output/$(basename $1).abi.json
