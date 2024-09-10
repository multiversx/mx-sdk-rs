#!/bin/bash

cleanup() {
    echo "Stopping all processes..."
    kill $(jobs -p)
    exit 0
}

trap cleanup SIGINT

BASE_DIR=$(pwd)

cd $BASE_DIR/api/src/services/sc-query-service
cargo run &

cd $BASE_DIR/api/src/services/tx-executor-service
cargo run &

cd $BASE_DIR/api
cargo run &

cd $BASE_DIR/frontend/src
wasm-pack build

cd $BASE_DIR/frontend/www
npm run start &

wait