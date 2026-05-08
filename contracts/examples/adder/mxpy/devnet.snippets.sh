#!/bin/bash

# ── Tool selection ─────────────────────────────────────────────────────────────
# sc-meta (new):
# TOOL=(sc-meta)
# or run from source:
TOOL=(cargo run --manifest-path ../../../../framework/meta/Cargo.toml --)
CONTRACT_CMD=tx

# mxpy (legacy):
# TOOL=(mxpy --verbose)
# CONTRACT_CMD=contract
# ──────────────────────────────────────────────────────────────────────────────

ALICE="../../../../sdk/core/src/test_wallets/alice.pem"
PROXY=https://devnet-gateway.multiversx.com
CHAIN=D
ADDRESS=$("${TOOL[@]}" data load --partition devnet --key=address-devnet)
OUTFILE="deploy-devnet.interaction.json"

export RUST_BACKTRACE=1

loadState() {
    ADDRESS=$("${TOOL[@]}" data parse --file="${OUTFILE}" --expression="data['contractAddress']" 2>/dev/null)
    DEPLOY_TRANSACTION=$("${TOOL[@]}" data parse --file="${OUTFILE}" --expression="data['emittedTransactionHash']" 2>/dev/null)
    "${TOOL[@]}" data store --partition devnet --key=address-devnet --value="${ADDRESS}" 2>/dev/null || true
    "${TOOL[@]}" data store --partition devnet --key=deployTransaction-devnet --value="${DEPLOY_TRANSACTION}" 2>/dev/null || true
}

deploy() {
    "${TOOL[@]}" ${CONTRACT_CMD} deploy \
        --bytecode "../output/adder.wasm" \
        --pem="${ALICE}" \
        --gas-limit=50000000 \
        --arguments 0 \
        --proxy="${PROXY}" \
        --chain="${CHAIN}" \
        --send \
        --outfile="${OUTFILE}" \
        || return

    loadState

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

add() {
    NUMBER=5
    "${TOOL[@]}" ${CONTRACT_CMD} call "${ADDRESS}" \
        --pem="${ALICE}" \
        --gas-limit=5000000 \
        --function="add" \
        --arguments "${NUMBER}" \
        --proxy="${PROXY}" \
        --chain="${CHAIN}" \
        --send
}

getSum() {
    "${TOOL[@]}" ${CONTRACT_CMD} query "${ADDRESS}" \
        --function="getSum" \
        --proxy="${PROXY}"
}
