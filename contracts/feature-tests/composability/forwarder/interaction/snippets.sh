ALICE="/home/elrond/MySandbox/testnet/wallets/users/alice.pem"
ADDRESS=$(erdpy data load --key=address-testnet)
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction-testnet)
PROXY=https://testnet-api.elrond.com
CHAIN_ID=T

SC_PARENT_ADDRESS_BECH32=erd1qqqqqqqqqqqqqpgqfzydqmdw7m2vazsp6u5p95yxz76t2p9rd8ss0zp9ts

SC_CHILD_ADDRESS_HEX=0000000000000000050011d9d2104d1bb4703accbf6dd06b4ffa87a125bd69e1
SC_CHILD_ADDRESS_BECH32=$(erdpy wallet bech32 --encode ${SC_CHILD_ADDRESS_HEX})

deploy() {
    erdpy --verbose contract deploy \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --recall-nonce \
        --pem=${ALICE} \
        --bytecode="../output/forwarder.wasm" \
        --gas-limit=50000000 \
        --send \
        --outfile="deploy-testnet.interaction.json" \
         || return

    TRANSACTION=$(erdpy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['address']")

    erdpy data store --key=address-testnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-testnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

deployFactorialChild() {
    local FACTORIAL_CODE=0x"$(xxd -p ../../examples/factorial/output/factorial.wasm | tr -d '\n')"

    erdpy --verbose contract call ${SC_PARENT_ADDRESS_BECH32} --recall-nonce --pem=${ALICE} --gas-limit=10000000 --function="deployContract" --arguments ${FACTORIAL_CODE} --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

callChildFactorial() {
    erdpy --verbose contract call ${SC_CHILD_ADDRESS_BECH32} --recall-nonce --pem=${ALICE} --gas-limit=10000000 --function="factorial" --arguments 0x05 --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

upgradeChildToAdder() {
    local ADDER_CODE=0x"$(xxd -p ../../examples/adder/output/adder.wasm | tr -d '\n')"
    local INIT_VALUE=42

    erdpy --verbose contract call ${SC_PARENT_ADDRESS_BECH32} --recall-nonce --pem=${ALICE} --gas-limit=500000000 --function="upgradeChildContract" --arguments 0x${SC_CHILD_ADDRESS_HEX} ${ADDER_CODE} ${INIT_VALUE} --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

deployTwoContracts() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=5000000 --function="deployTwoContracts" --send --proxy=${PROXY} --chain=${CHAIN_ID}
}
