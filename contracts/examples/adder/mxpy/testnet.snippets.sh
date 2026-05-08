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
PROXY=https://testnet-gateway.multiversx.com
CHAIN=T
ADDRESS=$("${TOOL[@]}" data load --partition testnet --key=address-testnet)
OUTFILE="deploy-testnet.interaction.json"

export RUST_LOG=trace

loadState() {
    ADDRESS=$("${TOOL[@]}" data parse --file="${OUTFILE}" --expression="data['contractAddress']" 2>/dev/null)
    DEPLOY_TRANSACTION=$("${TOOL[@]}" data parse --file="${OUTFILE}" --expression="data['emittedTransactionHash']" 2>/dev/null)
    "${TOOL[@]}" data store --partition testnet --key=address-testnet --value="${ADDRESS}" 2>/dev/null || true
    "${TOOL[@]}" data store --partition testnet --key=deployTransaction-testnet --value="${DEPLOY_TRANSACTION}" 2>/dev/null || true
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
