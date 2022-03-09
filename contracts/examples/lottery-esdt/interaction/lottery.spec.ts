import { Address, Balance, GasLimit, Nonce, NumericalType, NumericalValue, Token, TokenType, Transaction, U8Type } from "@elrondnetwork/erdjs";
import { AirdropService, BunchOfUsers, createTokenAmount, ESDTInteractor, ITestSession, IUser, TestSession } from "@elrondnetwork/erdjs-snippets";
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

    it("airdrop EGLD", async function () {
        session.expectLongInteraction(this);

        let amount = Balance.egld(1);
        await session.syncUsers([whale]);
        await AirdropService.createOnSession(session).sendToEachUser(whale, amount);
    });

    it("issue lottery token", async function () {
        session.expectLongInteraction(this);

        let interactor = await ESDTInteractor.create(session);
        let token = new Token({ name: "FOO", ticker: "FOO", decimals: 0, supply: "100000000", type: TokenType.Fungible })
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

        let interactor = await LotteryInteractor.create(session);
        let contractAddress = await interactor.deploy(owner);
        await session.saveAddress("contractAddress", contractAddress);
    });

    it("start lottery", async function () {
        session.expectLongInteraction(this);

        await session.syncUsers([owner]);

        let contractAddress = await session.loadAddress("contractAddress");
        let lotteryToken = await session.loadToken("lotteryToken");
        let interactor = await LotteryInteractor.create(session, contractAddress);
        await interactor.start(owner, "fooLottery", lotteryToken, 1);
    });
});

