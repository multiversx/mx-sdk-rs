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

export async function createInteractor(provider: IProvider, address?: Address): Promise<LotteryInteractor> {
    let registry = await AbiRegistry.load({ files: [PathToAbi] });
    let abi = new SmartContractAbi(registry, ["Lottery"]);
    let contract = new SmartContract({ address: address, abi: abi });
    let controller = new DefaultSmartContractController(abi, provider);
    let interactor = new LotteryInteractor(contract, controller);
    return interactor;
}

export class LotteryInteractor {
    private readonly contract: SmartContract;
    private readonly controller: ISmartContractController;

    constructor(contract: SmartContract, controller: ISmartContractController) {
        this.contract = contract;
        this.controller = controller;
    }
    
    async deploy(deployer: ITestUser): Promise<{ address: Address, returnCode: ReturnCode }> {
         // Load the bytecode from a file.
         let code = await Code.fromFile(PathToWasm);

         // Prepare the deploy transaction.
         let transaction = this.contract.deploy({
             code: code,
             codeMetadata: new CodeMetadata(),
             initArguments: [],
             gasLimit: new GasLimit(60000000)
         });

        // Set the transaction nonce. The account nonce must be synchronized beforehand.
        // Also, locally increment the nonce of the deployer (optional).
        transaction.setNonce(deployer.account.getNonceThenIncrement());

        // Let's sign the transaction. For dApps, use a wallet provider instead.
        await deployer.signer.sign(transaction);

        // After signing the deployment transaction, the contract address (deterministically computable) is available:
        let address = this.contract.getAddress();

        // Let's broadcast the transaction (and await for its execution), via the controller.
        let { bundle: { returnCode } } = await this.controller.deploy(transaction);

        console.log(`LotteryInteractor.deploy(): contract = ${address}`);
        return { address, returnCode };
    }

    async start(owner: ITestUser, lotteryName: string, token: Token, price: number, whitelist: Address[]): Promise<ReturnCode> {
        // Prepare the interaction
        let interaction = <Interaction>this.contract.methods
            .start([
                BytesValue.fromUTF8(lotteryName),
                new TokenIdentifierValue(token.identifier),
                new BigUIntValue(price),
                OptionValue.newMissing(),
                OptionValue.newMissing(),
                OptionValue.newProvided(new U32Value(1)),
                OptionValue.newMissing(),
                OptionValue.newProvided(createListOfAddresses(whitelist)),
                OptionalValue.newMissing()
            ])
            .withGasLimit(new GasLimit(20000000))
            .withNonce(owner.account.getNonceThenIncrement());

        // Let's build the transaction object.
        let transaction = interaction.buildTransaction();

        // Let's sign the transaction. For dApps, use a wallet provider instead.
        await owner.signer.sign(transaction);

        // Let's perform the interaction via the controller
        let { bundle: { returnCode } } = await this.controller.execute(interaction, transaction);
        return returnCode;
    }

    async buyTicket(user: ITestUser, lotteryName: string, amount: Balance): Promise<ReturnCode> {
        console.log(`buyTicket: address = ${user.address}, amount = ${amount.toCurrencyString()}`);

        // Prepare the interaction
        let interaction = <Interaction>this.contract.methods
            .buy_ticket([
                BytesValue.fromUTF8(lotteryName)
            ])
            .withGasLimit(new GasLimit(50000000))
            .withSingleESDTTransfer(amount)
            .withNonce(user.account.getNonceThenIncrement());

         // Let's build the transaction object.
         let transaction = interaction.buildTransaction();

         // Let's sign the transaction. For dApps, use a wallet provider instead.
         await user.signer.sign(transaction);
 
         // Let's perform the interaction via the controller
         let { bundle: { returnCode } } = await this.controller.execute(interaction, transaction);
         return returnCode;
    }

    async getLotteryInfo(lotteryName: string): Promise<Struct> {
        // Prepare the interaction
        let interaction = <Interaction>this.contract.methods.getLotteryInfo([
            BytesValue.fromUTF8(lotteryName)
        ]);

        // Let's perform the interaction via the controller.
        let { firstValue } = await this.controller.query(interaction);

        // Now let's interpret the results.
        let firstValueAsStruct = <Struct>firstValue;
        return firstValueAsStruct;
    }

    async getWhitelist(lotteryName: string): Promise<Address[]> {
        // Prepare the interaction
        let interaction = <Interaction>this.contract.methods.getLotteryWhitelist([
            BytesValue.fromUTF8(lotteryName)
        ]);

        // Let's perform the interaction via the controller.
        let { firstValue } = await this.controller.query(interaction);

        // Now let's interpret the results.
        let firstValueAsVariadic = <VariadicValue>firstValue;
        return firstValueAsVariadic.valueOf();
    }

    async getStatus(lotteryName: string): Promise<string> {
        // Prepare the interaction
        let interaction = <Interaction>this.contract.methods.status([
            BytesValue.fromUTF8(lotteryName)
        ]);
        
        // Let's perform the interaction via the controller.
        let { firstValue } = await this.controller.query(interaction);

        // Now let's interpret the results.
        let firstValueAsEnum = <EnumValue>firstValue;
        return firstValueAsEnum.name;
    }
}
