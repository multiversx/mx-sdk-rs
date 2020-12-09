import path = require("path");
import fs = require("fs");
import { Account, Argument, BackendSigner, Balance, Code, ContractFunction, GasLimit, NetworkConfig, ProxyProvider, SmartContract } from "@elrondnetwork/erdjs";
import { AbiRegistry, U32Value } from "@elrondnetwork/erdjs/out/smartcontracts/typesystem";
import { BinaryCodec } from "@elrondnetwork/erdjs/out/smartcontracts/codec"

async function main() {
    let codec = new BinaryCodec();
    let provider = new ProxyProvider("http://localhost:7950");
    let keyFilePath = path.resolve(__dirname, "../testnet/wallets/users/alice.json");
    let keyFileJson = fs.readFileSync(keyFilePath, { encoding: "utf8" });
    let keyFileObject = JSON.parse(keyFileJson);
    let password = "password";
    let signer = BackendSigner.fromWalletKey(keyFileObject, password);
    let user = new Account(signer.getAddress());

    let abiPath = path.resolve(__dirname, "abi.json");
    let abiJson = fs.readFileSync(abiPath, { encoding: "utf8" });
    let abiObject = JSON.parse(abiJson);
    let abi = new AbiRegistry();
    abi.extend(abiObject);
    let namespace = abi.findNamespace("lottery-egld");

    await NetworkConfig.getDefault().sync(provider);
    await user.sync(provider);

    // Deploy TRANSACTION
    let contractFile = path.resolve(__dirname, "../output/lottery-egld.wasm");
    let contract = new SmartContract({});
    let transactionDeploy = contract.deploy({
        code: Code.fromFile(contractFile),
        gasLimit: new GasLimit(100000000),
        initArguments: []
    });

    // The deploy transaction should be signed, so that the address of the contract
    // (required for the subsequent transactions) is computed.
    transactionDeploy.setNonce(user.nonce);
    await signer.sign(transactionDeploy);
    user.incrementNonce();

    // Start TRANSACTION
    let transactionStart = contract.call({
        func: new ContractFunction("start"),
        gasLimit: new GasLimit(50000000),
        args: [
            Argument.fromUTF8("foobar"),
            Argument.fromBigInt(Balance.eGLD(1).valueOf()),
            Argument.fromMissingOptional(),
            Argument.fromMissingOptional(),
            Argument.fromProvidedOptional(new U32Value(1)),
            Argument.fromMissingOptional(),
            Argument.fromMissingOptional()
        ]
    });

    // Apply nonces and sign the remaining transactions
    transactionStart.setNonce(user.nonce);
    await signer.sign(transactionStart);

    // Broadcast & execute
    await transactionDeploy.send(provider);
    await transactionStart.send(provider);

    await transactionDeploy.awaitExecuted(provider);
    await transactionStart.awaitExecuted(provider);

    // Query state
    let queryResponse = await contract.runQuery(provider, {
        func: new ContractFunction("lotteryExists"),
        args: [
            Argument.fromUTF8("foobar")
        ]
    });

    let values = codec.decodeFunctionOutput(queryResponse.buffers(), namespace.findFunction("lotteryExists"));
    console.log("lotteryExists", values);

    // Query state
    queryResponse = await contract.runQuery(provider, {
        func: new ContractFunction("lotteryInfo"),
        args: [
            Argument.fromUTF8("foobar")
        ]
    });

    values = codec.decodeFunctionOutput(queryResponse.buffers(), namespace.findFunction("lotteryInfo"));
    console.log("lotteryInfo", values[0].valueOf());
}

(async () => {
    await main();
})();
