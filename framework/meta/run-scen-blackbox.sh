#!/bin/bash

echo "Running scen-blackbox for order-book/pair..."
cargo run -- scen-blackbox --overwrite --path ../../contracts/examples/order-book/pair

echo "Done!"
