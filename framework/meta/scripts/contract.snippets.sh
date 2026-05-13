#!/bin/bash

NETWORK=devnet      # choose: devnet, testnet, mainnet, chain-simulator
TOOL_VARIANT=source # choose: sc-meta, source, mxpy

# ── Tool selection ─────────────────────────────────────────────────────────────
case "${TOOL_VARIANT}" in
    sc-meta) BASE=(sc-meta)                                      ; TX_CMD=tx       ;;
    source)  BASE=(cargo run --manifest-path ../Cargo.toml --)   ; TX_CMD=tx       ;;
    mxpy)    BASE=(mxpy --verbose)                               ; TX_CMD=contract ;;
esac

TX_TOOL=("${BASE[@]}" "${TX_CMD}")
DATA_TOOL=("${BASE[@]}" data)
# ──────────────────────────────────────────────────────────────────────────────

ALICE="../../../sdk/core/src/test_wallets/alice.pem"
BYTECODE_ADDER="../../../contracts/examples/adder/output/adder.wasm"
BYTECODE_PAYABLE_FEATURES="../../../contracts/feature-tests/payable-features/output/payable-features.wasm"

case "${NETWORK}" in
    devnet)           PROXY=https://devnet-gateway.multiversx.com;  CHAIN=D ;;
    testnet)          PROXY=https://testnet-gateway.multiversx.com; CHAIN=T ;;
    mainnet)          PROXY=https://gateway.multiversx.com;         CHAIN=1 ;;
    chain-simulator)  PROXY=http://localhost:8085;                  CHAIN=chain  ;;
esac

ADDRESS_ADDER=$("${DATA_TOOL[@]}" load --partition "${NETWORK}" --key="adder-address")
ADDRESS_PAYABLE_FEATURES=$("${DATA_TOOL[@]}" load --partition "${NETWORK}" --key="payable-features-address")
OUTFILE_DEPLOY="deploy-${NETWORK}.interaction.json"
OUTFILE_UPGRADE="upgrade-${NETWORK}.interaction.json"
OUTFILE_CALL="call-${NETWORK}.interaction.json"

export RUST_BACKTRACE=1

deployAdder() {
    "${TX_TOOL[@]}" deploy \
        --bytecode "${BYTECODE_ADDER}" \
        --pem="${ALICE}" \
        --gas-limit=50000000 \
        --arguments 0 \
        --proxy="${PROXY}" \
        --chain="${CHAIN}" \
        --send \
        --outfile="${OUTFILE_DEPLOY}" \
        || return

    ADDRESS_ADDER=$("${DATA_TOOL[@]}" parse --file="${OUTFILE_DEPLOY}" --expression="data['contractAddress']" 2>/dev/null)
    "${DATA_TOOL[@]}" store --partition "${NETWORK}" --key="adder-address" --value="${ADDRESS_ADDER}" 2>/dev/null || true

    echo ""
    echo "Adder address: ${ADDRESS_ADDER}"
}

deployPayableFeatures() {
    "${TX_TOOL[@]}" deploy \
        --bytecode "${BYTECODE_PAYABLE_FEATURES}" \
        --pem="${ALICE}" \
        --gas-limit=50000000 \
        --proxy="${PROXY}" \
        --chain="${CHAIN}" \
        --send \
        --outfile="${OUTFILE_DEPLOY}" \
        || return

    ADDRESS_PAYABLE_FEATURES=$("${DATA_TOOL[@]}" parse --file="${OUTFILE_DEPLOY}" --expression="data['contractAddress']" 2>/dev/null)
    "${DATA_TOOL[@]}" store --partition "${NETWORK}" --key="payable-features-address" --value="${ADDRESS_PAYABLE_FEATURES}" 2>/dev/null || true

    echo ""
    echo "Payable-features address: ${ADDRESS_PAYABLE_FEATURES}"
}

upgrade() {
    "${TX_TOOL[@]}" upgrade "${ADDRESS_ADDER}" \
        --bytecode "${BYTECODE_ADDER}" \
        --pem="${ALICE}" \
        --gas-limit=50000000 \
        --proxy="${PROXY}" \
        --chain="${CHAIN}" \
        --send \
        --outfile="${OUTFILE_UPGRADE}" \
        || return
}

add() {
    NUMBER=5
    "${TX_TOOL[@]}" call "${ADDRESS_ADDER}" \
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
    "${TX_TOOL[@]}" query "${ADDRESS_ADDER}" \
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

    "${TX_TOOL[@]}" call "${ADDRESS_ADDER}" \
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

pay1() {
    EGLD_VALUE=10000000000000000 # 0.01 EGLD
    "${TX_TOOL[@]}" call "${ADDRESS_PAYABLE_FEATURES}" \
        --pem="${ALICE}" \
        --gas-limit=5000000 \
        --function="payable_all" \
        --value="${EGLD_VALUE}" \
        --proxy="${PROXY}" \
        --chain="${CHAIN}" \
        --outfile="${OUTFILE_CALL}" \
        --send \
        --wait-result \
        || return
}

pay2() {
    USDC_AMOUNT=10000 # 0.01 USDC (6 decimals)
    EGLD_VALUE=10000000000000000 # 0.01 EGLD
    "${TX_TOOL[@]}" call "${ADDRESS_PAYABLE_FEATURES}" \
        --pem="${ALICE}" \
        --gas-limit=5000000 \
        --function="payable_all" \
        --token-transfers \
            "USDC-350c4e" "${USDC_AMOUNT}" \
            "EGLD-000000" "${EGLD_VALUE}" \
        --proxy="${PROXY}" \
        --chain="${CHAIN}" \
        --outfile="${OUTFILE_CALL}" \
        --send \
        --wait-result \
        || return
}

# pay3 uses --payments (TOKEN-ID NONCE AMOUNT triples), which is sc-meta only and NOT supported by mxpy.
pay3() {
    USDC_AMOUNT=10000 # 0.01 USDC (6 decimals), nonce 0 (fungible)
    EGLD_VALUE=10000000000000000 # 0.01 EGLD
    "${TX_TOOL[@]}" call "${ADDRESS_PAYABLE_FEATURES}" \
        --pem="${ALICE}" \
        --gas-limit=5000000 \
        --function="payable_all" \
        --payments "USDC-350c4e" 0 "${USDC_AMOUNT}" \
                   "EGLD-000000" 0 "${EGLD_VALUE}" \
        --proxy="${PROXY}" \
        --chain="${CHAIN}" \
        --outfile="${OUTFILE_CALL}" \
        --send \
        --wait-result \
        || return
}
