DENOMINATION="000000000000000000"
PROXY="http://localhost:7950"
CHAIN="local-testnet"
USERS="${CONTRACT_FOLDER}/testnet/wallets/users"
ALICE_PEM="$USERS/alice.pem"
CONTRACT_ADDRESS=$(cat address.txt)

deploySimulate() {
    erdpy contract deploy --project=${CONTRACT_FOLDER} --recall-nonce --pem=${ALICE_PEM} --gas-limit=900000000 --arguments=0x08,0xff --proxy=${PROXY} --chain=${CHAIN} --simulate > simulate.json
    jq ".result" simulate.json
    jq ".address" simulate.json
}

deploy() {
    erdpy contract deploy --project=${CONTRACT_FOLDER} --recall-nonce --pem=${ALICE_PEM} --gas-limit=900000000 --arguments=0x08,0xff --proxy=${PROXY} --chain=${CHAIN} --outfile=deploy.json --send
    grep "hash" deploy.json
    grep "address" deploy.json
    jq -r ".address" deploy.json > address.txt
}

add() {
    erdpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce --pem=${ALICE_PEM} --gas-limit=50000000 --function="add" --proxy=${PROXY} --arguments 0x0064 --chain=${CHAIN} --send
}

getSum() {
    erdpy --verbose contract query ${CONTRACT_ADDRESS} --function="getSum" --proxy=${PROXY}
}
