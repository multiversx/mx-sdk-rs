#!/bin/bash
set -e

mkdir -p www/pkg

./build-wasm.sh

cd www
npm install
npm start
