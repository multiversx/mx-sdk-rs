import { Balance, IProvider, ReturnCode, Token, TokenType } from "@elrondnetwork/erdjs";
import { AirdropService, createTokenAmount, ESDTInteractor, ITestSession, ITestUser, TestSession } from "@elrondnetwork/erdjs-snippets";
import { assert } from "chai";
import { createInteractor } from "./lotteryInteractor";

describe("lottery snippet", async function () {
    this.bail(true);

    const LotteryName = "fooLottery";

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

    it("airdrop EGLD", async function () {
        session.expectLongInteraction(this);

        let amount = Balance.egld(1);
        await session.syncUsers([whale]);
        await AirdropService.createOnSession(session).sendToEachUser(whale, amount);
    });

    it("issue lottery token", async function () {
        session.expectLongInteraction(this);

        let interactor = await ESDTInteractor.create(session);
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
        await AirdropService.createOnSession(session).sendToEachUser(owner, amount);
    });

    it("setup", async function () {
        session.expectLongInteraction(this);

        await session.syncUsers([owner]);

        let interactor = await createInteractor(provider);
        let { address, returnCode } = await interactor.deploy(owner);

        assert.isTrue(returnCode.isSuccess());

        await session.saveAddress("contractAddress", address);
    });

    it("start lottery", async function () {
        session.expectLongInteraction(this);

        await session.syncUsers([owner]);

        let contractAddress = await session.loadAddress("contractAddress");
        let lotteryToken = await session.loadToken("lotteryToken");
        let interactor = await createInteractor(provider, contractAddress);
        let whitelist = session.users.getAddressesOfFriends();
        let returnCode = await interactor.start(owner, LotteryName, lotteryToken, 1, whitelist);
        assert.isTrue(returnCode.isSuccess());
    });

    it("get lottery info and status", async function () {
        let contractAddress = await session.loadAddress("contractAddress");
        let lotteryToken = await session.loadToken("lotteryToken");
        let interactor = await createInteractor(provider, contractAddress);
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
        let interactor = await createInteractor(provider, contractAddress);
        let whitelist = await interactor.getWhitelist(LotteryName);
        console.log("Whitelist:", whitelist);
        
        assert.deepEqual(whitelist, session.users.getAddressesOfFriends());
    });

    it("friends buy tickets", async function () {
        session.expectLongInteraction(this);

        await session.syncAllUsers();

        let contractAddress = await session.loadAddress("contractAddress");
        let lotteryToken = await session.loadToken("lotteryToken");
        let interactor = await createInteractor(provider, contractAddress);

        let buyAmount = createTokenAmount(lotteryToken, "1");
        let buyPromises = session.users.getFriends().map(friend => interactor.buyTicket(friend, LotteryName, buyAmount));
        let returnCodes: ReturnCode[] = await Promise.all(buyPromises);
        
        for (const returnCode of returnCodes) {
            assert.isTrue(returnCode.isSuccess());
        }
    });
});
