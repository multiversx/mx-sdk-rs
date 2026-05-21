#!/bin/bash

NETWORK=devnet       # choose: devnet, testnet, mainnet, chain-simulator
TOOL_VARIANT=sc-meta # choose: sc-meta, mxpy

# ── Tool selection ─────────────────────────────────────────────────────────────
case "${TOOL_VARIANT}" in
    sc-meta) BASE=(sc-meta)        ; TX_CMD=tx       ;;
    mxpy)    BASE=(mxpy --verbose) ; TX_CMD=contract ;;
esac

TX_TOOL=("${BASE[@]}" "${TX_CMD}")
DATA_TOOL=("${BASE[@]}" data)
# ──────────────────────────────────────────────────────────────────────────────

if [[ "${TOOL_VARIANT}" == "sc-meta" ]]; then
    sc-meta wallet test-wallet --name alice
fi
ALICE="alice.pem"

case "${NETWORK}" in
    devnet)           PROXY=https://devnet-gateway.multiversx.com;  CHAIN=D ;;
    testnet)          PROXY=https://testnet-gateway.multiversx.com; CHAIN=T ;;
    mainnet)          PROXY=https://gateway.multiversx.com;         CHAIN=1 ;;
    chain-simulator)  PROXY=http://localhost:8085;                  CHAIN=chain  ;;
esac

ADDRESS=$("${DATA_TOOL[@]}" load --partition "${NETWORK}" --key="address-${NETWORK}")
BYTECODE="../output/adder.wasm"
OUTFILE_DEPLOY="deploy-${NETWORK}.interaction.json"
OUTFILE_UPGRADE="upgrade-${NETWORK}.interaction.json"
OUTFILE_CALL="call-${NETWORK}.interaction.json"

export RUST_BACKTRACE=1

deploy() {
    "${TX_TOOL[@]}" deploy \
        --bytecode "${BYTECODE}" \
        --pem="${ALICE}" \
        --gas-limit=50000000 \
        --arguments 0 \
        --proxy="${PROXY}" \
        --chain="${CHAIN}" \
        --send \
        --outfile="${OUTFILE_DEPLOY}" \
        --wait-result \
        || return

    ADDRESS=$("${DATA_TOOL[@]}" parse --file="${OUTFILE_DEPLOY}" --expression="data['contractAddress']" 2>/dev/null)
    DEPLOY_TRANSACTION=$("${DATA_TOOL[@]}" parse --file="${OUTFILE_DEPLOY}" --expression="data['emittedTransactionHash']" 2>/dev/null)
    "${DATA_TOOL[@]}" store --partition "${NETWORK}" --key="address-${NETWORK}"           --value="${ADDRESS}"            2>/dev/null || true
    "${DATA_TOOL[@]}" store --partition "${NETWORK}" --key="deployTransaction-${NETWORK}" --value="${DEPLOY_TRANSACTION}" 2>/dev/null || true

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

upgrade() {
    "${TX_TOOL[@]}" upgrade "${ADDRESS}" \
        --bytecode "${BYTECODE}" \
        --pem="${ALICE}" \
        --gas-limit=50000000 \
        --proxy="${PROXY}" \
        --chain="${CHAIN}" \
        --send \
        --outfile="${OUTFILE_UPGRADE}" \
        --wait-result \
        || return
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
        --wait-result \
        || return
}

sum() {
    "${TX_TOOL[@]}" query "${ADDRESS}" \
        --function="getSum" \
        --proxy="${PROXY}" \
        || return
}

# Demonstrates the full sign + send pipeline:
# 1. Build the call transaction (nonce auto-fetched, signed) and save — no broadcast.
# 2. Re-sign the saved file with `tx sign`.
# 3. Broadcast with `tx send`.
add_v2() {
    NUMBER=5
    OUTFILE_CALL_PREPARED="call-prepared-${NETWORK}.interaction.json"
    OUTFILE_CALL_SIGNED="call-signed-${NETWORK}.interaction.json"

    "${TX_TOOL[@]}" call "${ADDRESS}" \
        --pem="${ALICE}" \
        --gas-limit=5000000 \
        --function="add" \
        --arguments "${NUMBER}" \
        --proxy="${PROXY}" \
        --chain="${CHAIN}" \
        --outfile="${OUTFILE_CALL_PREPARED}" \
        || return

    "${BASE[@]}" tx sign \
        --infile="${OUTFILE_CALL_PREPARED}" \
        --pem="${ALICE}" \
        --proxy="${PROXY}" \
        --outfile="${OUTFILE_CALL_SIGNED}" \
        || return

    "${BASE[@]}" tx send \
        --proxy="${PROXY}" \
        --infile="${OUTFILE_CALL_SIGNED}" \
        || return
}
