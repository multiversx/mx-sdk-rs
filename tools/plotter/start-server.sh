#!/bin/bash
set -e

./build-wasm.sh

cd www
npm install
npm start
