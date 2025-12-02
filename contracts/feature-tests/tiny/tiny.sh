#!/bin/bash

wat2wasm tiny.wat                           
stat -f%z tiny.wasm                         
mx-scenario-go run scenarios/empty.scen.json
wasm-opt -O3 tiny.wasm -o tiny.wasm                
stat -f%z tiny.wasm
