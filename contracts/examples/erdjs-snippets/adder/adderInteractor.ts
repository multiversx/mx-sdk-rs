// adderInteractor.ts
/**
 * The code in this file is partially usable as production code, as well.
 * Note: in production code, make sure you do not depend on {@link ITestUser}.
 * Note: in production code, make sure you DO NOT reference the package "erdjs-snippets".
 * Note: in dApps, make sure you use a proper wallet provider to sign the transaction.
 * @module
 */
import path from "path";
import { AbiRegistry, Address, BigUIntValue, Code, CodeMetadata, DefaultSmartContractController, GasLimit, Interaction, IProvider, ISmartContractController, ReturnCode, SmartContract, SmartContractAbi } from "@elrondnetwork/erdjs";
import { ITestUser } from "@elrondnetwork/erdjs-snippets";

const PathToWasm = path.resolve(__dirname, "adder.wasm");
const PathToAbi = path.resolve(__dirname, "adder.abi.json");

export async function createInteractor(session: ITestSession, contractAddress?: IAddress): Promise<AdderInteractor> {
    let registry = await loadAbiRegistry(PathToAbi);
    let abi = new SmartContractAbi(registry, ["Adder"]);
    let contract = new SmartContract({ address: contractAddress, abi: abi });
    let networkProvider = session.networkProvider;
    let networkConfig = session.getNetworkConfig();
    let interactor = new AdderInteractor(contract, networkProvider, networkConfig);
    return interactor;
}

export class AdderInteractor {
    private readonly contract: SmartContract;
    private readonly networkProvider: INetworkProvider;
    private readonly networkConfig: NetworkConfig;
    private readonly transactionWatcher: TransactionWatcher;
    private readonly resultsParser: ResultsParser;

    constructor(contract: SmartContract, networkProvider: INetworkProvider, networkConfig: NetworkConfig) {
        this.contract = contract;
        this.networkProvider = networkProvider;
        this.networkConfig = networkConfig;
        this.transactionWatcher = new TransactionWatcher(networkProvider);
        this.resultsParser = new ResultsParser();
    }

    async deploy(deployer: ITestUser, initialValue: number): Promise<{ address: IAddress, returnCode: ReturnCode }> {
        // Load the bytecode.
        let code = await loadCode(PathToWasm);

        // Prepare the deploy transaction.
        let transaction = this.contract.deploy({
            code: code,
            codeMetadata: new CodeMetadata(),
            initArguments: [new BigUIntValue(initialValue)],
            gasLimit: new GasLimit(20000000),
            chainID: this.networkConfig.ChainID
        });

        // Set the transaction nonce. The account nonce must be synchronized beforehand.
        // Also, locally increment the nonce of the deployer (optional).
        transaction.setNonce(deployer.account.getNonceThenIncrement());

        // Let's sign the transaction. For dApps, use a wallet provider instead.
        await deployer.signer.sign(transaction);

        // The contract address is deterministically computable:
        let address = SmartContract.computeAddress(transaction.getSender(), transaction.getNonce());
        
        // Let's broadcast the transaction and await its completion:
        await this.networkProvider.sendTransaction(transaction);
        let transactionOnNetwork = await this.transactionWatcher.awaitCompleted(transaction);
        
        // In the end, parse the results:
        let { returnCode } = this.resultsParser.parseUntypedOutcome(transactionOnNetwork);

        console.log(`AdderInteractor.deploy(): contract = ${address}`);
        return { address, returnCode };
    }

    async add(caller: ITestUser, value: number): Promise<ReturnCode> {
        // Prepare the interaction
        let interaction = <Interaction>this.contract.methods
            .add([value])
            .withGasLimit(new GasLimit(10000000))
            .withNonce(caller.account.getNonceThenIncrement())
            .withChainID(this.networkConfig.ChainID);

        // Let's check the interaction, then build the transaction object.
        let transaction = interaction.check().buildTransaction();

        // Let's sign the transaction. For dApps, use a wallet provider instead.
        await caller.signer.sign(transaction);

        // Let's broadcast the transaction and await its completion:
        await this.networkProvider.sendTransaction(transaction);
        let transactionOnNetwork = await this.transactionWatcher.awaitCompleted(transaction);

        // In the end, parse the results:
        let { returnCode } = this.resultsParser.parseOutcome(transactionOnNetwork, interaction.getEndpoint());
        return returnCode;
    }

    async getSum(): Promise<number> {
        // Prepare the interaction, check it, then build the query:
        let interaction = <Interaction>this.contract.methods.getSum();
        let query = interaction.check().buildQuery();

        // Let's run the query and parse the results:
        let queryResponse = await this.networkProvider.queryContract(query);
        let { firstValue } = this.resultsParser.parseQueryResponse(queryResponse, interaction.getEndpoint());

        // Now let's interpret the results.
        let firstValueAsBigUInt = <BigUIntValue>firstValue;
        return firstValueAsBigUInt.valueOf().toNumber();
    }
}
