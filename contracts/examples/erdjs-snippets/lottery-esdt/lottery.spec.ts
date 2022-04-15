import { ReturnCode, TokenPayment } from "@elrondnetwork/erdjs";
import { createAirdropService, createESDTInteractor, INetworkProvider, ITestSession, ITestUser, TestSession } from "@elrondnetwork/erdjs-snippets";
import { assert } from "chai";
import { createInteractor } from "./lotteryInteractor";

describe("lottery snippet", async function () {
    this.bail(true);

    const LotteryName = "fooLottery";

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

    it("airdrop EGLD", async function () {
        session.expectLongInteraction(this);

        let payment = TokenPayment.egldFromAmount(0.1);
        await session.syncUsers([whale]);
        await createAirdropService(session).sendToEachUser(whale, friends, [payment]);
    });

    it("issue lottery token", async function () {
        session.expectLongInteraction(this);

        let interactor = await createESDTInteractor(session);
        await session.syncUsers([owner]);
        let token = await interactor.issueFungibleToken(owner, { name: "FOO", ticker: "FOO", decimals: 0, supply: "100000000" });
        await session.saveToken("lotteryToken", token);
    });

    it("airdrop lottery token", async function () {
        session.expectLongInteraction(this);

        let lotteryToken = await session.loadToken("lotteryToken");
        let payment = TokenPayment.fungibleFromAmount(lotteryToken.identifier, "10", lotteryToken.decimals);
        await session.syncUsers([owner]);
        await createAirdropService(session).sendToEachUser(owner, friends, [payment]);
    });

    it("setup", async function () {
        session.expectLongInteraction(this);

        await session.syncUsers([owner]);

        let interactor = await createInteractor(session);
        let { address, returnCode } = await interactor.deploy(owner);

        assert.isTrue(returnCode.isSuccess());

        await session.saveAddress("contractAddress", address);
    });

    it("start lottery", async function () {
        session.expectLongInteraction(this);

        await session.syncUsers([owner]);

        let contractAddress = await session.loadAddress("contractAddress");
        let lotteryToken = await session.loadToken("lotteryToken");
        let interactor = await createInteractor(session, contractAddress);
        let whitelist = friends.map(user => user.address);
        let returnCode = await interactor.start(owner, LotteryName, lotteryToken.identifier, 1, whitelist);
        assert.isTrue(returnCode.isSuccess());
    });

    it("get lottery info and status", async function () {
        let contractAddress = await session.loadAddress("contractAddress");
        let lotteryToken = await session.loadToken("lotteryToken");
        let interactor = await createInteractor(session, contractAddress);
        let lotteryInfo = await interactor.getLotteryInfo(LotteryName);
        let lotteryStatus = await interactor.getStatus(LotteryName);
        console.log("Info:", lotteryInfo.valueOf());
        console.log("Prize pool:", lotteryInfo.getFieldValue("prize_pool").toString());
        console.log("Status:", lotteryStatus);

        assert.equal(lotteryInfo.getFieldValue("token_identifier"), lotteryToken.identifier);
        assert.equal(lotteryStatus, "Running");
    });
    
    it("get whitelist", async function () {
        let contractAddress = await session.loadAddress("contractAddress");
        let interactor = await createInteractor(session, contractAddress);
        let whitelist = await interactor.getWhitelist(LotteryName);
        let expectedWhitelist = friends.map(user => user.address).map(address => address.bech32());
        
        console.log("Whitelist:", whitelist);
        assert.deepEqual(whitelist, expectedWhitelist);
    });

    it("friends buy tickets", async function () {
        session.expectLongInteraction(this);

        await session.syncUsers([owner, ...friends]);

        let contractAddress = await session.loadAddress("contractAddress");
        let lotteryToken = await session.loadToken("lotteryToken");
        let interactor = await createInteractor(session, contractAddress);

        let payment = TokenPayment.fungibleFromAmount(lotteryToken.identifier, "1", lotteryToken.decimals);
        let buyPromises = friends.map(friend => interactor.buyTicket(friend, LotteryName, payment));
        let returnCodes: ReturnCode[] = await Promise.all(buyPromises);
        
        for (const returnCode of returnCodes) {
            assert.isTrue(returnCode.isSuccess());
        }
    });
});
