#!/bin/bash

wat2wasm tiny.wat
stat -f%z tiny.wasm
mx-scenario-go run scenarios/tiny.scen.json
wasm-opt -O3 tiny.wasm -o tiny.wasm
