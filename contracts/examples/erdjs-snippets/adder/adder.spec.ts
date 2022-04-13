import { createAirdropService, INetworkProvider, ITestSession, ITestUser, TestSession } from "@elrondnetwork/erdjs-snippets";
import { TokenPayment } from "@elrondnetwork/erdjs";
import { assert } from "chai";
import { createInteractor } from "./adderInteractor";


describe("adder snippet", async function () {
    this.bail(true);

    let suite = this;
    let session: ITestSession;
    let provider: INetworkProvider;
    let whale: ITestUser;
    let owner: ITestUser;
    let friends: ITestUser[];

    this.beforeAll(async function () {
        session = await TestSession.loadOnSuite("devnet", suite);
        provider = session.networkProvider;
        whale = session.users.getUser("whale");
        owner = session.users.getUser("whale");
        friends = session.users.getGroup("friends");
        await session.syncNetworkConfig();
    });

    it("airdrop", async function () {
        session.expectLongInteraction(this);

        await session.syncUsers([whale]);
        await createAirdropService(session).sendToEachUser(whale, friends, TokenPayment.egldFromAmount(0.1));
    });

    it("setup", async function () {
        session.expectLongInteraction(this);

        await session.syncUsers([owner]);

        let interactor = await createInteractor(session);
        let { address, returnCode } = await interactor.deploy(owner, 42);

        assert.isTrue(returnCode.isSuccess());

        await session.saveAddress("contractAddress", address);
    });

    it("add", async function () {
        session.expectLongInteraction(this);

        await session.syncUsers([owner]);

        let contractAddress = await session.loadAddress("contractAddress");
        let interactor = await createInteractor(session, contractAddress);

        let sumBefore = await interactor.getSum();
        let returnCode = await interactor.add(owner, 3);
        let sumAfter = await interactor.getSum();
        assert.isTrue(returnCode.isSuccess());
        assert.equal(sumAfter, sumBefore + 3);
    });

    it("getSum", async function () {
        let contractAddress = await session.loadAddress("contractAddress");
        let interactor = await createInteractor(session, contractAddress);
        let result = await interactor.getSum();
        assert.isTrue(result > 0);
    });
});
