import { createAirdropService, FiveMinutesInMilliseconds, INetworkProvider, ITestSession, ITestUser, TestSession } from "@elrondnetwork/erdjs-snippets";
import { TokenPayment } from "@elrondnetwork/erdjs";
import { assert } from "chai";
import { createInteractor } from "./adderInteractor";

describe("adder snippet", async function () {
    this.bail(true);

    let session: ITestSession;
    let provider: INetworkProvider;
    let whale: ITestUser;
    let owner: ITestUser;
    let friends: ITestUser[];

    this.beforeAll(async function () {
        session = await TestSession.load("devnet", __dirname);
        provider = session.networkProvider;
        whale = session.users.getUser("whale");
        owner = session.users.getUser("whale");
        friends = session.users.getGroup("friends");
        await session.syncNetworkConfig();
    });

    this.beforeEach(async function () {
        session.correlation.step = this.currentTest?.fullTitle() || "";
    });

    it("airdrop", async function () {
        this.timeout(FiveMinutesInMilliseconds);

        await session.syncUsers([whale]);
        let payment = TokenPayment.egldFromAmount(0.1);
        await createAirdropService(session).sendToEachUser(whale, friends, [payment]);
    });

    it("setup", async function () {
        this.timeout(FiveMinutesInMilliseconds);

        await session.syncUsers([owner]);

        let interactor = await createInteractor(session);
        let { address, returnCode } = await interactor.deploy(owner, 42);

        assert.isTrue(returnCode.isSuccess());

        await session.saveAddress({ name: "adder", address: address });
    });

    it("add", async function () {
        this.timeout(FiveMinutesInMilliseconds);
        // If the step fails, retry it (using a Mocha utility function).
        this.retries(5);

        await session.syncUsers([owner]);

        const contractAddress = await session.loadAddress("adder");
        const interactor = await createInteractor(session, contractAddress);

        const sumBefore = await interactor.getSum();
        const snapshotBefore = await session.audit.onSnapshot({ state: { sum: sumBefore } });

        const returnCode = await interactor.add(owner, 3);
        await session.audit.onContractOutcome({ returnCode });

        const sumAfter = await interactor.getSum();
        await session.audit.onSnapshot({ state: { sum: sumBefore }, comparableTo: snapshotBefore });

        assert.isTrue(returnCode.isSuccess());
        assert.equal(sumAfter, sumBefore + 3);
    });

    it("getSum", async function () {
        let contractAddress = await session.loadAddress("adder");
        let interactor = await createInteractor(session, contractAddress);
        let result = await interactor.getSum();
        assert.isTrue(result > 0);
    });

    it("generate report", async function () {
        await session.generateReport();
    });

    it("destroy session", async function () {
        await session.destroy();
    });
});
