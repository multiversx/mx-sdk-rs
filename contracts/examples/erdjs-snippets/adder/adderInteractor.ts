import path from "path";
import { AbiRegistry, Address, BigUIntValue, Code, CodeMetadata, DefaultSmartContractController, GasLimit, Interaction, ISmartContractController, ReturnCode, SmartContract, SmartContractAbi } from "@elrondnetwork/erdjs";
import { ITestSession, IUser } from "@elrondnetwork/erdjs-snippets";

const PathToWasm = path.resolve(__dirname, "..", "..", "adder", "output", "adder.wasm");
const PathToAbi = path.resolve(__dirname, "..", "..", "adder", "output", "adder.abi.json");

/**
 * Creates a contract interactor for a test session. The code within this function is usable in production, as well.
 * Make sure you do not depend on the test session, though.
 */
export async function createInteractor(session: ITestSession, address?: Address): Promise<AdderInteractor> {
    let registry = await AbiRegistry.load({ files: [PathToAbi] });
    let abi = new SmartContractAbi(registry, ["Adder"]);
    let contract = new SmartContract({ address: address, abi: abi });
    let controller = new DefaultSmartContractController(abi, session.proxy);
    let interactor = new AdderInteractor(contract, controller);
    return interactor;
}

export class AdderInteractor {
    private readonly contract: SmartContract;
    private readonly controller: ISmartContractController;

    constructor(contract: SmartContract, controller: ISmartContractController) {
        this.contract = contract;
        this.controller = controller;
    }

    async deploy(deployer: IUser, initialValue: number): Promise<{ address: Address, returnCode: ReturnCode }> {
        // Load the bytecode from a file.
        let code = await Code.fromFile(PathToWasm);

        // Prepare the deploy transaction.
        let transaction = this.contract.deploy({
            code: code,
            codeMetadata: new CodeMetadata(),
            initArguments: [new BigUIntValue(initialValue)],
            gasLimit: new GasLimit(20000000)
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

        console.log(`AdderInteractor.deploy(): contract = ${address}`);
        return { address, returnCode };
    }

    async add(caller: IUser, value: number): Promise<ReturnCode> {
        // Prepare the interaction
        let interaction = <Interaction>this.contract.methods
            .add([new BigUIntValue(value)])
            .withGasLimit(new GasLimit(10000000))
            .withNonce(caller.account.getNonceThenIncrement());

        // Let's build the transaction object.
        let transaction = interaction.buildTransaction();

        // Let's sign the transaction. For dApps, use a wallet provider instead.
        await caller.signer.sign(transaction);

        // Let's perform the interaction via the controller
        let { bundle: { returnCode } } = await this.controller.execute(interaction, transaction);
        return returnCode;
    }

    async getSum(): Promise<number> {
        // Prepare the interaction
        let interaction = <Interaction>this.contract.methods.getSum();

        // Let's perform the interaction via the controller.
        let { firstValue } = await this.controller.query(interaction);

        // Now let's interpret the results.
        let firstValueAsBigUInt = <BigUIntValue>firstValue;
        return firstValueAsBigUInt.valueOf().toNumber();
    }
}
