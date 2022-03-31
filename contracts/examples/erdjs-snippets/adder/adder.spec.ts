import { Balance, IProvider } from "@elrondnetwork/erdjs";
import { AirdropService, ITestSession, ITestUser, TestSession } from "@elrondnetwork/erdjs-snippets";
import { assert } from "chai";
import { createInteractor } from "./adderInteractor";

describe("adder snippet", async function () {
    this.bail(true);

    let suite = this;
    let session: ITestSession;
    let provider: IProvider;
    let whale: ITestUser;
    let owner: ITestUser;

    this.beforeAll(async function () {
        session = await TestSession.loadOnSuite("default", suite);
        provider = session.provider;
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

        let interactor = await createInteractor(provider);
        let { address, returnCode } = await interactor.deploy(owner, 42);
        
        assert.isTrue(returnCode.isSuccess());

        await session.saveAddress("contractAddress", address);
    });

    it("add", async function () {
        session.expectLongInteraction(this);

        await session.syncUsers([owner]);

        let contractAddress = await session.loadAddress("contractAddress");
        let interactor = await createInteractor(provider, contractAddress);

        let sumBefore = await interactor.getSum();
        let returnCode = await interactor.add(owner, 3);
        let sumAfter = await interactor.getSum();
        assert.isTrue(returnCode.isSuccess());
        assert.equal(sumAfter, sumBefore + 3);
    });

    it("getSum", async function () {
        let contractAddress = await session.loadAddress("contractAddress");
        let interactor = await createInteractor(provider, contractAddress);
        let result = await interactor.getSum();
        assert.isTrue(result > 0);
    });
});
