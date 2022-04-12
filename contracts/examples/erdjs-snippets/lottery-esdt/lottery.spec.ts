import { Balance, IProvider, ReturnCode, Token, TokenType } from "@elrondnetwork/erdjs";
import { AirdropService, createTokenAmount, ESDTInteractor, ITestSession, ITestUser, TestSession } from "@elrondnetwork/erdjs-snippets";
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

        let amount = Balance.egld(0.1);
        await session.syncUsers([whale]);
        await createAirdropService(session).sendToEachUser(whale, friends, amount);
    });

    it("issue lottery token", async function () {
        session.expectLongInteraction(this);

        let interactor = await createESDTInteractor(session);
        let token = new Token({ name: "FOO", ticker: "FOO", decimals: 0, supply: "100000000", type: TokenType.Fungible });
        await session.syncUsers([owner]);
        await interactor.issueToken(owner, token);
        await session.saveToken("lotteryToken", token);
    });

    it("airdrop lottery token", async function () {
        session.expectLongInteraction(this);

        let lotteryToken = await session.loadToken("lotteryToken");
        let amount = createTokenAmount(lotteryToken, "10");
        await session.syncUsers([owner]);
        await createAirdropService(session).sendToEachUser(owner, friends, amount);
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
        let returnCode = await interactor.start(owner, LotteryName, lotteryToken, 1, whitelist);
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

        let buyAmount = createTokenAmount(lotteryToken, "1");
        let buyPromises = friends.map(friend => interactor.buyTicket(friend, LotteryName, buyAmount));
        let returnCodes: ReturnCode[] = await Promise.all(buyPromises);
        
        for (const returnCode of returnCodes) {
            assert.isTrue(returnCode.isSuccess());
        }
    });
});
