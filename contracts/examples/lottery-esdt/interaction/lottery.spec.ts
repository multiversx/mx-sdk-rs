import { Address, Balance, GasLimit, Nonce, NumericalType, NumericalValue, Transaction, U8Type } from "@elrondnetwork/erdjs";
import { AirdropService, ITestSession, IUser, TestSession } from "@elrondnetwork/erdjs-snippets";
import { assert } from "chai";
import { LotteryInteractor } from "./lotteryInteractor";

describe("lottery snippet", async function () {
    this.bail(true);

    let suite = this;
    let session: ITestSession;
    let whale: IUser;
    let owner: IUser;

    this.beforeAll(async function () {
        session = await TestSession.loadOnSuite("default", suite);
        whale = session.users.whale;
        owner = session.users.alice;
        await session.syncNetworkConfig();
    });

    it("airdrop", async function () {
        session.expectLongInteraction(this);

        await session.syncUsers([whale]);
        await AirdropService.createOnSession(session).sendToEachUser(Balance.egld(1));
    });

    it("setup", async function () {
        session.expectLongInteraction(this);

        await session.syncUsers([owner]);

        let interactor = await LotteryInteractor.create(session);
        let contractAddress = await interactor.deploy(owner);
        await session.saveAddress("contractAddress", contractAddress);
    });
});

