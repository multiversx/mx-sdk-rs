// lotteryInteractor.ts
/**
 * The code in this file is partially usable as production code, as well.
 * Note: in production code, make sure you do not depend on {@link ITestUser}.
 * Note: in production code, make sure you DO NOT reference the package "erdjs-snippets".
 * Note: in dApps, make sure you use a proper wallet provider to sign the transaction.
 * @module
 */
import path from "path";
import { CodeMetadata, EnumValue, IAddress, Interaction, ResultsParser, ReturnCode, SmartContract, SmartContractAbi, Struct, TokenPayment, TransactionWatcher, VariadicValue } from "@elrondnetwork/erdjs";
import { IAudit, INetworkConfig, INetworkProvider, ITestSession, ITestUser, loadAbiRegistry, loadCode } from "@elrondnetwork/erdjs-snippets";

const PathToWasm = path.resolve(__dirname, "..", "..", "lottery-esdt", "output", "lottery-esdt.wasm");
const PathToAbi = path.resolve(__dirname, "..", "..", "lottery-esdt", "output", "lottery-esdt.abi.json");

export async function createInteractor(session: ITestSession, contractAddress?: IAddress): Promise<LotteryInteractor> {
    const registry = await loadAbiRegistry(PathToAbi);
    const abi = new SmartContractAbi(registry);
    const contract = new SmartContract({ address: contractAddress, abi: abi });
    const networkProvider = session.networkProvider;
    const networkConfig = session.getNetworkConfig();
    const audit = session.audit;
    const interactor = new LotteryInteractor(contract, networkProvider, networkConfig, audit);
    return interactor;
}

export class LotteryInteractor {
    private readonly contract: SmartContract;
    private readonly networkProvider: INetworkProvider;
    private readonly networkConfig: INetworkConfig;
    private readonly transactionWatcher: TransactionWatcher;
    private readonly resultsParser: ResultsParser;
    private readonly audit: IAudit;

    constructor(contract: SmartContract, networkProvider: INetworkProvider, networkConfig: INetworkConfig, audit: IAudit) {
        this.contract = contract;
        this.networkProvider = networkProvider;
        this.networkConfig = networkConfig;
        this.transactionWatcher = new TransactionWatcher(networkProvider);
        this.resultsParser = new ResultsParser();
        this.audit = audit;
    }

    async deploy(deployer: ITestUser): Promise<{ address: IAddress, returnCode: ReturnCode }> {
        // Load the bytecode from a file.
        let code = await loadCode(PathToWasm);

        // Prepare the deploy transaction.
        let transaction = this.contract.deploy({
            code: code,
            codeMetadata: new CodeMetadata(),
            initArguments: [],
            gasLimit: 60000000,
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
        const transactionHash = await this.networkProvider.sendTransaction(transaction);
        await this.audit.onContractDeploymentSent({ transactionHash: transactionHash, contractAddress: address });

        let transactionOnNetwork = await this.transactionWatcher.awaitCompleted(transaction);
        await this.audit.onTransactionCompleted({ transactionHash: transactionHash, transaction: transactionOnNetwork });

        // In the end, parse the results:
        let { returnCode } = this.resultsParser.parseUntypedOutcome(transactionOnNetwork);

        console.log(`LotteryInteractor.deploy(): contract = ${address}`);
        return { address, returnCode };
    }

    async start(owner: ITestUser, lotteryName: string, tokenIdentifier: string, price: number, whitelist: IAddress[]): Promise<ReturnCode> {
        console.log(`LotteryInteractor.start(): lotteryName = ${lotteryName}`);

        // Prepare the interaction
        let interaction = <Interaction>this.contract.methods
            .start([
                lotteryName,
                tokenIdentifier,
                price,
                null,
                null,
                1,
                null,
                whitelist
            ])
            .withGasLimit(20000000)
            .withNonce(owner.account.getNonceThenIncrement())
            .withChainID(this.networkConfig.ChainID);

        // Let's check the interaction, then build the transaction object.
        let transaction = interaction.check().buildTransaction();

        // Let's sign the transaction. For dApps, use a wallet provider instead.
        await owner.signer.sign(transaction);

        // Let's broadcast the transaction and await its completion:
        const transactionHash = await this.networkProvider.sendTransaction(transaction);
        await this.audit.onTransactionSent({ action: "start", args: [lotteryName, tokenIdentifier], transactionHash: transactionHash });

        let transactionOnNetwork = await this.transactionWatcher.awaitCompleted(transaction);
        await this.audit.onTransactionCompleted({ transactionHash: transactionHash, transaction: transactionOnNetwork });

        // In the end, parse the results:
        let { returnCode, returnMessage } = this.resultsParser.parseOutcome(transactionOnNetwork, interaction.getEndpoint());
        console.log(`LotteryInteractor.start(): lotteryName = ${lotteryName}, returnCode = ${returnCode}, returnMessage = ${returnMessage}`);
        return returnCode;
    }

    async buyTicket(user: ITestUser, lotteryName: string, amount: TokenPayment): Promise<ReturnCode> {
        console.log(`LotteryInteractor.buyTicket(): address = ${user.address}, amount = ${amount.toPrettyString()}`);

        // Prepare the interaction
        let interaction = <Interaction>this.contract.methods
            .buy_ticket([
                lotteryName
            ])
            .withGasLimit(50000000)
            .withSingleESDTTransfer(amount)
            .withNonce(user.account.getNonceThenIncrement())
            .withChainID(this.networkConfig.ChainID);

        // Let's check the interaction, then build the transaction object.
        let transaction = interaction.check().buildTransaction();

        // Let's sign the transaction. For dApps, use a wallet provider instead.
        await user.signer.sign(transaction);

        // Let's broadcast the transaction and await its completion:
        const transactionHash = await this.networkProvider.sendTransaction(transaction);
        await this.audit.onTransactionSent({ action: "buyTicket", args: [lotteryName, amount.toPrettyString()], transactionHash: transactionHash });

        const transactionOnNetwork = await this.transactionWatcher.awaitCompleted(transaction);
        await this.audit.onTransactionCompleted({ transactionHash: transactionHash, transaction: transactionOnNetwork });

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
