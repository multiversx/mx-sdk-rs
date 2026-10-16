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

PEM="alice.pem"
RELAYER_PEM="s1mon.pem"

# Creates a test wallet (alice.pem). Only works with sc-meta.
setup() {
    sc-meta wallet test-wallet --name alice
    sc-meta wallet test-wallet --name s1mon   # will act as the relayer
}

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
        --pem="${PEM}" \
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
        --pem="${PEM}" \
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
        --pem="${PEM}" \
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
        --pem="${PEM}" \
        --gas-limit=5000000 \
        --function="add" \
        --arguments "${NUMBER}" \
        --proxy="${PROXY}" \
        --chain="${CHAIN}" \
        --outfile="${OUTFILE_CALL_PREPARED}" \
        || return

    "${BASE[@]}" tx sign \
        --infile="${OUTFILE_CALL_PREPARED}" \
        --pem="${PEM}" \
        --proxy="${PROXY}" \
        --outfile="${OUTFILE_CALL_SIGNED}" \
        || return

    "${BASE[@]}" tx send \
        --proxy="${PROXY}" \
        --infile="${OUTFILE_CALL_SIGNED}" \
        || return
}

# Signs the add call with a Ledger hardware wallet instead of a PEM file.
# Uses --ledger (address index 0 by default); override with LEDGER_ADDRESS_INDEX.
add_ledger() {
    NUMBER=5
    LEDGER_ADDRESS_INDEX="${LEDGER_ADDRESS_INDEX:-0}"

    "${TX_TOOL[@]}" call "${ADDRESS}" \
        --ledger \
        --sender-wallet-index "${LEDGER_ADDRESS_INDEX}" \
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

# Demonstrates relayed v3 transactions (sc-meta only):
# Bob (relayer) pays the gas so Alice (sender) needs no EGLD for fees.
#
# Two-step flow:
#   1. Alice builds + signs the call with --relayer pointing to Bob's address.
#      The gas limit includes an extra base cost (50 000) for the relay operation.
#      The transaction is saved to a file — not broadcast yet.
#   2. Bob signs as relayer with `tx relay` and broadcasts.
add_relayed() {
    NUMBER=5
    RELAYER_ADDRESS="erd1fwdty5j20525zmah34xcsfyacvp890twuxu2ql2rgtpn5s9qqqqsajtqv5"
    OUTFILE_CALL_UNSIGNED="call-unsigned-${NETWORK}.interaction.json"
    OUTFILE_CALL_RELAYED="call-relayed-${NETWORK}.interaction.json"

    # Step 1: Alice signs (gas = 5 000 000 execution + 50 000 extra relay base cost).
    "${TX_TOOL[@]}" call "${ADDRESS}" \
        --pem="${PEM}" \
        --gas-limit=5050000 \
        --function="add" \
        --arguments "${NUMBER}" \
        --relayer="${RELAYER_ADDRESS}" \
        --proxy="${PROXY}" \
        --chain="${CHAIN}" \
        --outfile="${OUTFILE_CALL_UNSIGNED}" \
        || return

    # Step 2: Bob adds his relayer signature and broadcasts.
    "${BASE[@]}" tx relay \
        --infile="${OUTFILE_CALL_UNSIGNED}" \
        --relayer-pem="${RELAYER_PEM}" \
        --proxy="${PROXY}" \
        --send \
        --outfile="${OUTFILE_CALL_RELAYED}" \
        --wait-result \
        || return
}

# One-step variant: Alice signs and Bob co-signs in a single command.
# Requires both PEM files to be present on the same machine.
# The relayer address is derived automatically from --relayer-pem.
add_relayed_onestep() {
    NUMBER=5

    "${TX_TOOL[@]}" call "${ADDRESS}" \
        --pem="${PEM}" \
        --gas-limit=5050000 \
        --function="add" \
        --arguments "${NUMBER}" \
        --relayer-pem="${RELAYER_PEM}" \
        --proxy="${PROXY}" \
        --chain="${CHAIN}" \
        --send \
        --outfile="${OUTFILE_CALL}" \
        --wait-result \
        || return
}
