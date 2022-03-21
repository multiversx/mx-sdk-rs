import { Balance, ReturnCode, Token, TokenType } from "@elrondnetwork/erdjs";
import { AirdropService, createTokenAmount, ESDTInteractor, ITestSession, IUser, TestSession } from "@elrondnetwork/erdjs-snippets";
import { assert } from "chai";
import { createInteractor } from "./lotteryInteractor";

describe("lottery snippet", async function () {
    this.bail(true);

    const LotteryName = "fooLottery";

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
        let returnCode = await interactor.start(owner, LotteryName, lotteryToken, 1);
        assert.isTrue(returnCode.isSuccess());
    });

    it("get lottery info and status", async function () {
        let contractAddress = await session.loadAddress("contractAddress");
        let lotteryToken = await session.loadToken("lotteryToken");
        let interactor = await createInteractor(session, contractAddress);
        let lotteryInfo = await interactor.getLotteryInfo(LotteryName);
        let lotteryStatus = await interactor.getStatus(LotteryName);
        console.log("Info:", lotteryInfo);
        console.log("Prize pool:", lotteryInfo.prize_pool.toString());
        console.log("Status:", lotteryStatus);

        assert.equal(lotteryInfo.token_identifier.toString(), lotteryToken.identifier);
        assert.equal(lotteryStatus, "Running");
    });

    it("friends buy tickets", async function () {
        session.expectLongInteraction(this);

        await session.syncAllUsers();

        let contractAddress = await session.loadAddress("contractAddress");
        let lotteryToken = await session.loadToken("lotteryToken");
        let interactor = await createInteractor(session, contractAddress);

        let buyAmount = createTokenAmount(lotteryToken, "1");
        let buyPromises = session.users.getFriends().map(friend => interactor.buyTicket(friend, LotteryName, buyAmount));
        let returnCodes: ReturnCode[] = await Promise.all(buyPromises);
        
        for (const returnCode of returnCodes) {
            assert.isTrue(returnCode.isSuccess());
        }
    });
});
