DENOMINATION="000000000000000000"
PROXY="https://api.elrond.com"
CHAIN="BoN"
ALICE_PEM="~/wallet/alice.pem"
CONTRACT_ADDRESS="erd1YourContractHere"

deploy() {
    erdpy contract deploy ${CONTRACT_FOLDER} --recall-nonce --pem=${ALICE_PEM} --gas-limit=50000000 --proxy=${PROXY} --chain=${CHAIN}
}

add() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${ALICE_PEM} --gas-limit=50000000 --function="add" --proxy=${PROXY} --arguments 0x0064
}

getSum() {
    erdpy --verbose contract query ${CONTRACT_ADDRESS} --function="getSum" --proxy=${PROXY}
}
