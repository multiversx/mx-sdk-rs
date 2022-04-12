// lotteryInteractor.ts
/**
 * The code in this file is partially usable as production code, as well.
 * Note: in production code, make sure you do not depend on {@link ITestUser}.
 * Note: in production code, make sure you DO NOT reference the package "erdjs-snippets".
 * Note: in dApps, make sure you use a proper wallet provider to sign the transaction.
 * @module
 */
import { AbiRegistry, Address, Balance, BigUIntValue, BytesValue, Code, CodeMetadata, createListOfAddresses, DefaultSmartContractController, EnumValue, GasLimit, Interaction, IProvider, ISmartContractController, OptionalValue, OptionValue, ReturnCode, SmartContract, SmartContractAbi, Struct, Token, TokenIdentifierValue, U32Value, VariadicValue } from "@elrondnetwork/erdjs";
import path from "path";
import { ITestUser } from "@elrondnetwork/erdjs-snippets";

const PathToWasm = path.resolve(__dirname, "lottery-esdt.wasm");
const PathToAbi = path.resolve(__dirname, "lottery-esdt.abi.json");

export async function createInteractor(session: ITestSession, contractAddress?: IAddress): Promise<LotteryInteractor> {
    let registry = await loadAbiRegistry(PathToAbi);
    let abi = new SmartContractAbi(registry, ["Lottery"]);
    let contract = new SmartContract({ address: contractAddress, abi: abi });
    let networkProvider = session.networkProvider;
    let networkConfig = session.getNetworkConfig();
    let interactor = new LotteryInteractor(contract, networkProvider, networkConfig);
    return interactor;
}

export class LotteryInteractor {
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

    async deploy(deployer: ITestUser): Promise<{ address: IAddress, returnCode: ReturnCode }> {
        // Load the bytecode from a file.
        let code = await loadCode(PathToWasm);

        // Prepare the deploy transaction.
        let transaction = this.contract.deploy({
            code: code,
            codeMetadata: new CodeMetadata(),
            initArguments: [],
            gasLimit: new GasLimit(60000000),
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

        console.log(`LotteryInteractor.deploy(): contract = ${address}`);
        return { address, returnCode };
    }

    async start(owner: ITestUser, lotteryName: string, token: Token, price: number, whitelist: IAddress[]): Promise<ReturnCode> {
        console.log(`LotteryInteractor.start(): lotteryName = ${lotteryName}`);

        // Prepare the interaction
        let interaction = <Interaction>this.contract.methods
            .start([
                lotteryName,
                token.identifier,
                price,
                null,
                null,
                1,
                null,
                whitelist
            ])
            .withGasLimit(new GasLimit(20000000))
            .withNonce(owner.account.getNonceThenIncrement())
            .withChainID(this.networkConfig.ChainID);

        // Let's check the interaction, then build the transaction object.
        let transaction = interaction.check().buildTransaction();

        // Let's sign the transaction. For dApps, use a wallet provider instead.
        await owner.signer.sign(transaction);

        // Let's broadcast the transaction and await its completion:
        await this.networkProvider.sendTransaction(transaction);
        let transactionOnNetwork = await this.transactionWatcher.awaitCompleted(transaction);

        // In the end, parse the results:
        let { returnCode, returnMessage } = this.resultsParser.parseOutcome(transactionOnNetwork, interaction.getEndpoint());
        console.log(`LotteryInteractor.start(): lotteryName = ${lotteryName}, returnCode = ${returnCode}, returnMessage = ${returnMessage}`);
        return returnCode;
    }

    async buyTicket(user: ITestUser, lotteryName: string, amount: Balance): Promise<ReturnCode> {
        console.log(`LotteryInteractor.buyTicket(): address = ${user.address}, amount = ${amount.toCurrencyString()}`);

        // Prepare the interaction
        let interaction = <Interaction>this.contract.methods
            .buy_ticket([
                lotteryName
            ])
            .withGasLimit(new GasLimit(50000000))
            .withSingleESDTTransfer(amount)
            .withNonce(user.account.getNonceThenIncrement())
            .withChainID(this.networkConfig.ChainID);

        // Let's check the interaction, then build the transaction object.
        let transaction = interaction.check().buildTransaction();

        // Let's sign the transaction. For dApps, use a wallet provider instead.
        await user.signer.sign(transaction);

        // Let's broadcast the transaction and await its completion:
        await this.networkProvider.sendTransaction(transaction);
        let transactionOnNetwork = await this.transactionWatcher.awaitCompleted(transaction);

        // In the end, parse the results:
        let { returnCode } = this.resultsParser.parseOutcome(transactionOnNetwork, interaction.getEndpoint());
        return returnCode;
    }

    async getLotteryInfo(lotteryName: string): Promise<Struct> {
        // Prepare the interaction
        let interaction = <Interaction>this.contract.methods.getLotteryInfo([lotteryName]);
        let query = interaction.check().buildQuery();

        // Let's run the query and parse the results:
        let queryResponse = await this.networkProvider.queryContract(query);
        let { firstValue } = this.resultsParser.parseQueryResponse(queryResponse, interaction.getEndpoint());

        // Now let's interpret the results.
        let firstValueAsStruct = <Struct>firstValue;
        return firstValueAsStruct;
    }

    async getWhitelist(lotteryName: string): Promise<string[]> {
        // Prepare the interaction
        let interaction = <Interaction>this.contract.methods.getLotteryWhitelist([lotteryName]);
        let query = interaction.check().buildQuery();

        // Let's run the query and parse the results:
        let queryResponse = await this.networkProvider.queryContract(query);
        let { firstValue } = this.resultsParser.parseQueryResponse(queryResponse, interaction.getEndpoint());

        // Now let's interpret the results.
        let firstValueAsVariadic = <VariadicValue>firstValue;
        return firstValueAsVariadic.valueOf().map(item => item.toString());
    }

    async getStatus(lotteryName: string): Promise<string> {
        // Prepare the interaction
        let interaction = <Interaction>this.contract.methods.status([lotteryName]);
        let query = interaction.check().buildQuery();

        // Let's run the query and parse the results:
        let queryResponse = await this.networkProvider.queryContract(query);
        let { firstValue } = this.resultsParser.parseQueryResponse(queryResponse, interaction.getEndpoint());

        // Now let's interpret the results.
        let firstValueAsEnum = <EnumValue>firstValue;
        return firstValueAsEnum.name;
    }
}
