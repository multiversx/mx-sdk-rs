import { Address, BigUIntType, BigUIntValue, BytesValue, Code, GasLimit, Interaction, OptionalType, OptionalValue, OptionValue, Token, TokenIdentifierValue, U32Type, U32Value } from "@elrondnetwork/erdjs";
import { createSmartContract, DefaultInteractor, ITestSession, IUser } from "@elrondnetwork/erdjs-snippets";
import path from "path";

const PathToWasm = path.resolve(__dirname, "..", "output", "lottery-esdt.wasm");
const PathToAbi = path.resolve(__dirname, "..", "output", "lottery-esdt.abi.json");

export class LotteryInteractor extends DefaultInteractor {
    static async create(session: ITestSession, address?: Address) {
        let contract = await createSmartContract(PathToAbi, address);
        let interactor = new LotteryInteractor(session, contract);
        return interactor;
    }

    async deploy(deployer: IUser): Promise<Address> {
        return await this.doDeploy(deployer, PathToWasm, {
            gasLimit: new GasLimit(60000000),
            initArguments: []
        });
    }

    async start(owner: IUser, lotteryName: string, token: Token, price: number): Promise<void> {
        let interaction = <Interaction>this.contract.methods
            .start([
                BytesValue.fromUTF8(lotteryName),
                new TokenIdentifierValue(Buffer.from(token.identifier)),
                new BigUIntValue(price),
                OptionValue.newMissing(),
                OptionValue.newMissing(),
                new OptionValue(new U32Type(), new U32Value(1)),
                OptionValue.newMissing(),
                OptionValue.newMissing(),
                new OptionalValue(new OptionalType(new BigUIntType()))
            ])
            .withGasLimit(new GasLimit(10000000));

        await this.runInteraction(owner, interaction);
    }
}
