import { Balance } from "@elrondnetwork/erdjs";
import { AirdropService, ITestSession, IUser, TestSession } from "@elrondnetwork/erdjs-snippets";
import { assert } from "chai";
import { AdderInteractor } from "./adderInteractor";

describe("adder snippet", async function () {
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
        await AirdropService.createOnSession(session).sendToEachUser(whale, Balance.egld(1));
    });

    it("setup", async function () {
        session.expectLongInteraction(this);

        await session.syncUsers([owner]);

        let interactor = await AdderInteractor.create(session);
        let contractAddress = await interactor.deploy(owner, 42);
        await session.saveAddress("contractAddress", contractAddress);
    });

    it("add", async function () {
        session.expectLongInteraction(this);

        await session.syncUsers([owner]);

        let contractAddress = await session.loadAddress("contractAddress");
        let interactor = await AdderInteractor.create(session, contractAddress);

        await interactor.add(owner, 3);
    });

    it("getSum", async function () {
        let contractAddress = await session.loadAddress("contractAddress");
        let interactor = await AdderInteractor.create(session, contractAddress);
        let result = await interactor.getSum(owner);
        assert.isTrue(result > 0);
    });
});
