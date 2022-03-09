import { Address, BigUIntValue, Code, GasLimit, Interaction } from "@elrondnetwork/erdjs";
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
}
