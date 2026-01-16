ALICE="${USERS}/alice.pem"
BOB="${USERS}/bob.pem"

ADDRESS=$(mxpy data load --key=address-testnet)
DEPLOY_TRANSACTION=$(mxpy data load --key=deployTransaction-testnet)
PROXY=https://testnet-api.multiversx.com

DEPLOY_GAS="80000000"
TARGET=10
DEADLINE_UNIX_TIMESTAMP=1609452000 # Fri Jan 01 2021 00:00:00 GMT+0200 (Eastern European Standard Time)
EGLD_TOKEN_ID=0x45474c44 # "EGLD"

deploy() {
    mxpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} \
          --gas-limit=${DEPLOY_GAS} \
          --arguments ${TARGET} ${DEADLINE_UNIX_TIMESTAMP} ${EGLD_TOKEN_ID} \
          --outfile="deploy-testnet.interaction.json" --send --proxy=${PROXY} --chain=T || return

    TRANSACTION=$(mxpy data parse --file="deploy-testnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(mxpy data parse --file="deploy-testnet.interaction.json" --expression="data['contractAddress']")

    mxpy data store --key=address-testnet --value=${ADDRESS}
    mxpy data store --key=deployTransaction-testnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

checkDeployment() {
    mxpy tx get --hash=$DEPLOY_TRANSACTION --omit-fields="['data', 'signature']" --proxy=${PROXY}
    mxpy account get --address=$ADDRESS --omit-fields="['code']" --proxy=${PROXY}
}

# BOB sends funds
sendFunds() {
    mxpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${BOB} --gas-limit=10000000 \
        --function="fund" --value=5 \
        --proxy=${PROXY} --chain=T \
        --send
}

# ALICE claims
claimFunds() {
    mxpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=10000000 \
        --function="claim" \
        --proxy=${PROXY} --chain=T \
        --send
}

# 0 - Funding Period
# 1 - Successful
# 2 - Failed
status() {
    mxpy --verbose contract query ${ADDRESS} --function="status" --proxy=${PROXY} --chain=T
}

getCurrentFunds() {
    mxpy --verbose contract query ${ADDRESS} --function="getCurrentFunds" --proxy=${PROXY} --chain=T
}

getTarget() {
    mxpy --verbose contract query ${ADDRESS} --function="getTarget" --proxy=${PROXY} --chain=T
}

getDeadline() {
    mxpy --verbose contract query ${ADDRESS} --function="getDeadline" --proxy=${PROXY} --chain=T
}

# BOB's deposit
getDeposit() {
    local BOB_ADDRESS_BECH32=erd1spyavw0956vq68xj8y4tenjpq2wd5a9p2c6j8gsz7ztyrnpxrruqzu66jx
    local BOB_ADDRESS_HEX=0x$(mxpy wallet bech32 --decode ${BOB_ADDRESS_BECH32})

    mxpy --verbose contract query ${ADDRESS} --function="getDeposit" --arguments ${BOB_ADDRESS_HEX} --proxy=${PROXY} --chain=T
}

getCrowdfundingTokenName() {
    mxpy --verbose contract query ${ADDRESS} --function="getCrowdfundingTokenName" --proxy=${PROXY} --chain=T
}
