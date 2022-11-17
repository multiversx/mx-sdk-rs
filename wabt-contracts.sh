#!/bin/bash

export PATH=$HOME/tools/wabt/bin:$PATH

./wat-build.sh contracts/examples/adder
# ./wat-build.sh contracts/examples/erc20
# ./wat-build.sh contracts/examples/crowdfunding-erc20
# ./wat-build.sh contracts/feature-tests/composability/proxy-test-first
# ./wat-build.sh contracts/feature-tests/composability/forwarder
# ./wat-build.sh contracts/benchmarks/managed-data-copy
# ./wat-build.sh contracts/feature-tests/basic-features
