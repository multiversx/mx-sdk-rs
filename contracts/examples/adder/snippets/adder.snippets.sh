#!/bin/bash

NETWORK=chain-simulator      # choose: devnet, testnet, mainnet, chain-simulator
TOOL_VARIANT=source # choose: sc-meta, source, mxpy

# ── Tool selection ─────────────────────────────────────────────────────────────
case "${TOOL_VARIANT}" in
    sc-meta) BASE=(sc-meta)                                                              ; TX_CMD=tx       ;;
    source)  BASE=(cargo run --manifest-path ../../../../framework/meta/Cargo.toml --)   ; TX_CMD=tx       ;;
    mxpy)    BASE=(mxpy --verbose)                                                       ; TX_CMD=contract ;;
esac

TX_TOOL=("${BASE[@]}" "${TX_CMD}")
DATA_TOOL=("${BASE[@]}" data)
# ──────────────────────────────────────────────────────────────────────────────

ALICE="../../../../sdk/core/src/test_wallets/alice.pem"

case "${NETWORK}" in
    devnet)           PROXY=https://devnet-gateway.multiversx.com;  CHAIN=D ;;
    testnet)          PROXY=https://testnet-gateway.multiversx.com; CHAIN=T ;;
    mainnet)          PROXY=https://gateway.multiversx.com;         CHAIN=1 ;;
    chain-simulator)  PROXY=http://localhost:8085;                  CHAIN=chain  ;;
esac

ADDRESS=$("${DATA_TOOL[@]}" load --partition "${NETWORK}" --key="address-${NETWORK}")
OUTFILE_DEPLOY="deploy-${NETWORK}.interaction.json"
OUTFILE_CALL="call-${NETWORK}.interaction.json"

export RUST_BACKTRACE=1

deploy() {
    "${TX_TOOL[@]}" deploy \
        --bytecode "../output/adder.wasm" \
        --pem="${ALICE}" \
        --gas-limit=50000000 \
        --arguments 0 \
        --proxy="${PROXY}" \
        --chain="${CHAIN}" \
        --send \
        --outfile="${OUTFILE_DEPLOY}" \
        || return

    ADDRESS=$("${DATA_TOOL[@]}" parse --file="${OUTFILE_DEPLOY}" --expression="data['contractAddress']" 2>/dev/null)
    DEPLOY_TRANSACTION=$("${DATA_TOOL[@]}" parse --file="${OUTFILE_DEPLOY}" --expression="data['emittedTransactionHash']" 2>/dev/null)
    "${DATA_TOOL[@]}" store --partition "${NETWORK}" --key="address-${NETWORK}"           --value="${ADDRESS}"            2>/dev/null || true
    "${DATA_TOOL[@]}" store --partition "${NETWORK}" --key="deployTransaction-${NETWORK}" --value="${DEPLOY_TRANSACTION}" 2>/dev/null || true

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

add() {
    NUMBER=5
    "${TX_TOOL[@]}" call "${ADDRESS}" \
        --pem="${ALICE}" \
        --gas-limit=5000000 \
        --function="add" \
        --arguments "${NUMBER}" \
        --proxy="${PROXY}" \
        --chain="${CHAIN}" \
        --send \
        --outfile="${OUTFILE_CALL}" \
        || return
}

getSum() {
    "${TX_TOOL[@]}" query "${ADDRESS}" \
        --function="getSum" \
        --proxy="${PROXY}" \
        || return
}
