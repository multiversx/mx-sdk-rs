#!/bin/bash

NETWORK=devnet      # choose: devnet, testnet, mainnet, chain-simulator
TOOL_VARIANT=source # choose: sc-meta, source, mxpy

# ── Tool selection ─────────────────────────────────────────────────────────────
case "${TOOL_VARIANT}" in
    sc-meta) BASE=(sc-meta)                                    ; TX_CMD=tx       ;;
    source)  BASE=(cargo run --manifest-path ../../../../framework/meta/Cargo.toml --) ; TX_CMD=tx       ;;
    mxpy)    BASE=(mxpy --verbose)                             ; TX_CMD=contract ;;
esac

TX_TOOL=("${BASE[@]}" "${TX_CMD}")
DATA_TOOL=("${BASE[@]}" data)
# ──────────────────────────────────────────────────────────────────────────────

ALICE="../../../../sdk/core/src/test_wallets/alice.pem"
BYTECODE="../output/ping-pong-egld.wasm"

case "${NETWORK}" in
    devnet)           PROXY=https://devnet-gateway.multiversx.com;  CHAIN=D      ;;
    testnet)          PROXY=https://testnet-gateway.multiversx.com; CHAIN=T      ;;
    mainnet)          PROXY=https://gateway.multiversx.com;         CHAIN=1      ;;
    chain-simulator)  PROXY=http://localhost:8085;                  CHAIN=chain  ;;
esac

ADDRESS=$("${DATA_TOOL[@]}" load --partition "${NETWORK}" --key="ping-pong-address")
OUTFILE_DEPLOY="deploy-${NETWORK}.interaction.json"
OUTFILE_UPGRADE="upgrade-${NETWORK}.interaction.json"
OUTFILE_CALL="call-${NETWORK}.interaction.json"

export RUST_BACKTRACE=1

# Encodes a u64 as a 16-char hex string (for Option<u64> ABI encoding).
number_to_u64() {
    printf "%016x" "$1"
}

# Encodes Some(n) as 0x01<hex-u64> for Option<TimestampMillis> arguments.
option_u64_arg() {
    echo "0x01$(number_to_u64 "$1")"
}

deploy() {
    PING_AMOUNT=1500000000000000000 # 1.5 EGLD
    DURATION_MS=180000              # 3 minutes in milliseconds
    # Pass "0" for opt_activation_timestamp to use None (activate immediately).
    # To activate later: option_u64_arg <unix_timestamp_ms>
    OPT_ACTIVATION=0

    "${TX_TOOL[@]}" deploy \
        --bytecode="${BYTECODE}" \
        --pem="${ALICE}" \
        --gas-limit=60000000 \
        --arguments "${PING_AMOUNT}" "${DURATION_MS}" "${OPT_ACTIVATION}" \
        --proxy="${PROXY}" \
        --chain="${CHAIN}" \
        --send \
        --outfile="${OUTFILE_DEPLOY}" \
        || return

    ADDRESS=$("${DATA_TOOL[@]}" parse --file="${OUTFILE_DEPLOY}" --expression="data['contractAddress']" 2>/dev/null)
    "${DATA_TOOL[@]}" store --partition "${NETWORK}" --key="ping-pong-address" --value="${ADDRESS}" 2>/dev/null || true

    echo ""
    echo "Ping-pong address: ${ADDRESS}"
}

upgrade() {
    PING_AMOUNT=1500000000000000000 # 1.5 EGLD
    DURATION_MS=180000              # 3 minutes in milliseconds
    OPT_ACTIVATION=0

    "${TX_TOOL[@]}" upgrade "${ADDRESS}" \
        --bytecode="${BYTECODE}" \
        --pem="${ALICE}" \
        --gas-limit=60000000 \
        --arguments "${PING_AMOUNT}" "${DURATION_MS}" "${OPT_ACTIVATION}" \
        --proxy="${PROXY}" \
        --chain="${CHAIN}" \
        --send \
        --outfile="${OUTFILE_UPGRADE}" \
        || return
}

ping() {
    PING_AMOUNT=1500000000000000000 # must match the deployed ping_amount

    "${TX_TOOL[@]}" call "${ADDRESS}" \
        --pem="${ALICE}" \
        --gas-limit=5000000 \
        --function="ping" \
        --value="${PING_AMOUNT}" \
        --proxy="${PROXY}" \
        --chain="${CHAIN}" \
        --send \
        --outfile="${OUTFILE_CALL}" \
        --wait-result \
        || return
}

pong() {
    "${TX_TOOL[@]}" call "${ADDRESS}" \
        --pem="${ALICE}" \
        --gas-limit=5000000 \
        --function="pong" \
        --proxy="${PROXY}" \
        --chain="${CHAIN}" \
        --send \
        --outfile="${OUTFILE_CALL}" \
        --wait-result \
        || return
}

pongAll() {
    "${TX_TOOL[@]}" call "${ADDRESS}" \
        --pem="${ALICE}" \
        --gas-limit=50000000 \
        --function="pongAll" \
        --proxy="${PROXY}" \
        --chain="${CHAIN}" \
        --send \
        --outfile="${OUTFILE_CALL}" \
        --wait-result \
        || return
}

getContractState() {
    "${TX_TOOL[@]}" query "${ADDRESS}" \
        --function="getContractState" \
        --proxy="${PROXY}" \
        || return
}

getUserAddresses() {
    "${TX_TOOL[@]}" query "${ADDRESS}" \
        --function="getUserAddresses" \
        --proxy="${PROXY}" \
        || return
}
