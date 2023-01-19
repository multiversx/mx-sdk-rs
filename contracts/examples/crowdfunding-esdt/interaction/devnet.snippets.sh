ALICE="${USERS}/alice.pem"
BOB="${USERS}/bob.pem"

ADDRESS=$(mxpy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(mxpy data load --key=deployTransaction-devnet)

DEPLOY_GAS="80000000"
TARGET=10
DEADLINE_UNIX_TIMESTAMP=1609452000 # Fri Jan 01 2021 00:00:00 GMT+0200 (Eastern European Standard Time)
EGLD_TOKEN_ID=0x45474c44 # "EGLD"

deploy() {
    mxpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} \
          --gas-limit=${DEPLOY_GAS} \
          --arguments ${TARGET} ${DEADLINE_UNIX_TIMESTAMP} ${EGLD_TOKEN_ID} \
          --outfile="deploy-devnet.interaction.json" --send || return

    TRANSACTION=$(mxpy data parse --file="deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(mxpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    mxpy data store --key=address-devnet --value=${ADDRESS}
    mxpy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

deploySimulate() {
    mxpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} \
          --gas-limit=${DEPLOY_GAS} \
          --arguments ${TARGET} ${DEADLINE_UNIX_TIMESTAMP} ${EGLD_TOKEN_ID} \
          --outfile="simulate-devnet.interaction.json" --simulate || return

    TRANSACTION=$(mxpy data parse --file="simulate-devnet.interaction.json" --expression="data['result']['hash']")
    ADDRESS=$(mxpy data parse --file="simulate-devnet.interaction.json" --expression="data['contractAddress']")
    RETCODE=$(mxpy data parse --file="simulate-devnet.interaction.json" --expression="data['result']['returnCode']")
    RETMSG=$(mxpy data parse --file="simulate-devnet.interaction.json" --expression="data['result']['returnMessage']")

    echo ""
    echo "Simulated transaction: ${TRANSACTION}"
    echo "Smart contract address: ${ADDRESS}"
    echo "Deployment return code: ${RETCODE}"
    echo "Deployment return message: ${RETMSG}"
}

checkDeployment() {
    mxpy tx get --hash=$DEPLOY_TRANSACTION --omit-fields="['data', 'signature']"
    mxpy account get --address=$ADDRESS --omit-fields="['code']"
}

# BOB sends funds
sendFunds() {
    mxpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${BOB} --gas-limit=10000000 \
        --function="fund" --value=5 \
        --send
}

# ALICE claims
claimFunds() {
    mxpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=10000000 \
        --function="claim" \
        --send
}

# 0 - Funding Period
# 1 - Successful
# 2 - Failed
status() {
    mxpy --verbose contract query ${ADDRESS} --function="status"
}

getCurrentFunds() {
    mxpy --verbose contract query ${ADDRESS} --function="getCurrentFunds"
}

getTarget() {
    mxpy --verbose contract query ${ADDRESS} --function="getTarget"
}

getDeadline() {
    mxpy --verbose contract query ${ADDRESS} --function="getDeadline"
}

# BOB's deposit
getDeposit() {
    local BOB_ADDRESS_BECH32=erd1spyavw0956vq68xj8y4tenjpq2wd5a9p2c6j8gsz7ztyrnpxrruqzu66jx
    local BOB_ADDRESS_HEX=0x$(mxpy wallet bech32 --decode ${BOB_ADDRESS_BECH32})

    mxpy --verbose contract query ${ADDRESS} --function="getDeposit" --arguments ${BOB_ADDRESS_HEX}
}

getCrowdfundingTokenName() {
    mxpy --verbose contract query ${ADDRESS} --function="getCrowdfundingTokenName"
}
