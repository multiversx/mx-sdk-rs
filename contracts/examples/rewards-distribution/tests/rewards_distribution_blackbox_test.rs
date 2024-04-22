mod mock_seed_nft_minter;
mod mock_seed_nft_minter_proxy;
mod utils;

use multiversx_sc_scenario::imports::*;
use std::iter::zip;

use rewards_distribution::{
    rewards_distribution_proxy, ContractObj, ProxyTrait as _, RewardsDistribution,
    DIVISION_SAFETY_CONSTANT,
};

const NFT_TOKEN_ID: &[u8] = b"NFT-123456";
const NFT_TOKEN_ID_EXPR: &str = "str:NFT-123456";

const ALICE_ADDRESS_EXPR: AddressExpr = AddressExpr("alice");
const OWNER_ADDRESS_EXPR: AddressExpr = AddressExpr("owner");
const REWARDS_DISTRIBUTION_ADDRESS_EXPR: ScExpr = ScExpr("rewards-distribution");
const REWARDS_DISTRIBUTION_PATH_EXPR: MxscExpr = MxscExpr("output/rewards-distribution.mxsc.json");
const SEED_NFT_MINTER_ADDRESS_EXPR: ScExpr = ScExpr("seed-nft-minter");
const SEED_NFT_MINTER_PATH_EXPR: MxscExpr =
    MxscExpr("../seed-nft-minter/output/seed-nft-minter.mxsc.json");

type RewardsDistributionContract = ContractInfo<rewards_distribution::Proxy<StaticApi>>;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        REWARDS_DISTRIBUTION_PATH_EXPR.eval_to_expr().as_str(),
        rewards_distribution::ContractBuilder,
    );
    blockchain.register_contract(
        SEED_NFT_MINTER_PATH_EXPR.eval_to_expr().as_str(),
        mock_seed_nft_minter::ContractBuilder,
    );
    blockchain
}

struct RewardsDistributionTestState {
    world: ScenarioWorld,
    rewards_distribution_contract: RewardsDistributionContract,
    rewards_distribution_whitebox: WhiteboxContract<ContractObj<DebugApi>>,
}

impl RewardsDistributionTestState {
    fn new() -> Self {
        let mut world = world();

        world.account(OWNER_ADDRESS_EXPR).nonce(1);

        let rewards_distribution_contract =
            RewardsDistributionContract::new(REWARDS_DISTRIBUTION_ADDRESS_EXPR);
        let rewards_distribution_whitebox = WhiteboxContract::new(
            REWARDS_DISTRIBUTION_ADDRESS_EXPR,
            rewards_distribution::contract_obj,
        );

        Self {
            world,
            rewards_distribution_contract,
            rewards_distribution_whitebox,
        }
    }

    fn deploy_seed_nft_minter_contract(&mut self) -> &mut Self {
        self.world
            .tx()
            .from(OWNER_ADDRESS_EXPR)
            .typed(mock_seed_nft_minter_proxy::MockSeedNftMinterProxy)
            .init(TokenIdentifier::from_esdt_bytes(NFT_TOKEN_ID))
            .code(SEED_NFT_MINTER_PATH_EXPR)
            .run();

        self.world
            .tx()
            .from(OWNER_ADDRESS_EXPR)
            .to(SEED_NFT_MINTER_ADDRESS_EXPR)
            .typed(mock_seed_nft_minter_proxy::MockSeedNftMinterProxy)
            .set_nft_count(10_000u64)
            .run();

        self
    }

    fn deploy_rewards_distribution_contract(&mut self) -> &mut Self {
        let brackets_vec = &[
            (10, 2_000),
            (90, 6_000),
            (400, 7_000),
            (2_500, 10_000),
            (25_000, 35_000),
            (72_000, 40_000),
        ];
        let mut brackets = ManagedVec::new();
        for (index_percent, bracket_reward_percent) in brackets_vec.iter().cloned() {
            brackets.push(rewards_distribution_proxy::Bracket {
                index_percent,
                bracket_reward_percent,
            });
        }
        self.world
            .tx()
            .from(OWNER_ADDRESS_EXPR)
            .typed(rewards_distribution_proxy::RewardsDistributionProxy)
            .init(SEED_NFT_MINTER_ADDRESS_EXPR.to_address(), brackets)
            .code(REWARDS_DISTRIBUTION_PATH_EXPR)
            .run();

        self
    }
}

#[test]
fn test_compute_brackets() {
    let mut state = RewardsDistributionTestState::new();

    let rewards_distribution_code = state
        .world
        .code_expression(REWARDS_DISTRIBUTION_PATH_EXPR.eval_to_expr().as_str());

    state
        .world
        .account(REWARDS_DISTRIBUTION_ADDRESS_EXPR)
        .nonce(1)
        .owner(OWNER_ADDRESS_EXPR)
        .code(rewards_distribution_code)
        .balance("0");

    state.world.whitebox_call(
        &state.rewards_distribution_whitebox,
        ScCallStep::new().from(OWNER_ADDRESS_EXPR),
        |sc| {
            let brackets = utils::to_brackets(&[
                (10, 2_000),
                (90, 6_000),
                (400, 7_000),
                (2_500, 10_000),
                (25_000, 35_000),
                (72_000, 40_000),
            ]);

            let computed_brackets = sc.compute_brackets(brackets, 10_000);

            let expected_values = vec![
                (1, 2_000 * DIVISION_SAFETY_CONSTANT),
                (10, 6_000 * DIVISION_SAFETY_CONSTANT / (10 - 1)),
                (50, 7_000 * DIVISION_SAFETY_CONSTANT / (50 - 10)),
                (300, 10_000 * DIVISION_SAFETY_CONSTANT / (300 - 50)),
                (2_800, 35_000 * DIVISION_SAFETY_CONSTANT / (2_800 - 300)),
                (10_000, 40_000 * DIVISION_SAFETY_CONSTANT / (10_000 - 2_800)),
            ];

            assert_eq!(computed_brackets.len(), expected_values.len());
            for (computed, expected) in zip(computed_brackets.iter(), expected_values) {
                let (expected_end_index, expected_reward_percent) = expected;
                assert_eq!(computed.end_index, expected_end_index);
                assert_eq!(computed.nft_reward_percent, expected_reward_percent);
            }
        },
    );
}

#[test]
fn test_raffle_and_claim() {
    let mut state = RewardsDistributionTestState::new();

    let nft_nonces: [u64; 6] = [1, 2, 3, 4, 5, 6];
    let mut nft_payments = ManagedVec::new();
    for nonce in nft_nonces.into_iter() {
        let payment = EsdtTokenPayment::new(NFT_TOKEN_ID.into(), nonce, 1u64.into());
        nft_payments.push(payment);
    }

    {
        let mut account_setter = state
            .world
            .account(ALICE_ADDRESS_EXPR)
            .nonce(1)
            .balance("2_070_000_000");
        for nft_nonce in nft_nonces {
            account_setter = account_setter.esdt_nft_balance(
                NFT_TOKEN_ID_EXPR,
                nft_nonce,
                "1",
                Option::<&[u8]>::None,
            );
        }
    }

    state.world.set_state_step(
        SetStateStep::new()
            .new_address(OWNER_ADDRESS_EXPR, 1, SEED_NFT_MINTER_ADDRESS_EXPR)
            .new_address(OWNER_ADDRESS_EXPR, 3, REWARDS_DISTRIBUTION_ADDRESS_EXPR),
    );

    state
        .deploy_seed_nft_minter_contract()
        .deploy_rewards_distribution_contract();

    // deposit royalties
    state
        .world
        .tx()
        .from(ALICE_ADDRESS_EXPR)
        .to(REWARDS_DISTRIBUTION_ADDRESS_EXPR)
        .typed(rewards_distribution_proxy::RewardsDistributionProxy)
        .deposit_royalties()
        .egld(2_070_000_000)
        .run();

    // run the raffle
    state.world.sc_call(
        ScCallStep::new()
            .from(ALICE_ADDRESS_EXPR)
            .tx_hash(&[0u8; 32]) // blockchain rng is deterministic, so we can use a fixed hash
            .call(state.rewards_distribution_contract.raffle())
            .expect_value(OperationCompletionStatus::Completed),
    );

    let mut rewards: Vec<BigUint<StaticApi>> = Vec::new();
    // post-raffle reward amount frequency checksstate
    for nonce in 1u64..=10_000u64 {
        let reward = state
            .world
            .tx()
            .from(ALICE_ADDRESS_EXPR)
            .to(REWARDS_DISTRIBUTION_ADDRESS_EXPR)
            .typed(rewards_distribution_proxy::RewardsDistributionProxy)
            .compute_claimable_amount(0u64, &EgldOrEsdtTokenIdentifier::egld(), 0u64, nonce)
            .returns(ReturnsResult)
            .run();
        rewards.push(reward);
    }

    assert_eq!(rewards.len() as u64, 10_000u64);

    // check that the reward amounts match in frequency
    let expected_reward_amounts = [
        (41_400_000, 1),
        (13_799_999, 9),
        (3_622_500, 40),
        (828_000, 250),
        (289_800, 2500),
        (114_999, 7200),
    ];

    let total_expected_count: u64 = expected_reward_amounts.iter().map(|(_, count)| count).sum();
    assert_eq!(total_expected_count, 10_000u64);

    for (amount, expected_count) in expected_reward_amounts {
        let expected_amount = amount as u64;
        assert_eq!(
            rewards
                .iter()
                .filter(|value| *value == &expected_amount)
                .count(),
            expected_count as usize
        );
    }

    let expected_rewards = [114_999, 114_999, 114_999, 828_000, 114_999, 114_999];

    for (nonce, expected_reward) in std::iter::zip(nft_nonces, expected_rewards) {
        state
            .world
            .tx()
            .from(ALICE_ADDRESS_EXPR)
            .to(REWARDS_DISTRIBUTION_ADDRESS_EXPR)
            .typed(rewards_distribution_proxy::RewardsDistributionProxy)
            .compute_claimable_amount(0u64, &EgldOrEsdtTokenIdentifier::egld(), 0u64, nonce)
            .returns(ExpectValue(expected_reward))
            .run();
    }

    // claim rewards
    let mut reward_tokens: MultiValueEncoded<
        StaticApi,
        MultiValue2<EgldOrEsdtTokenIdentifier<StaticApi>, u64>,
    > = MultiValueEncoded::new();
    reward_tokens.push((EgldOrEsdtTokenIdentifier::egld(), 0).into());
    state
        .world
        .tx()
        .from(ALICE_ADDRESS_EXPR)
        .to(REWARDS_DISTRIBUTION_ADDRESS_EXPR)
        .typed(rewards_distribution_proxy::RewardsDistributionProxy)
        .claim_rewards(0u64, 0u64, reward_tokens)
        .with_multi_token_transfer(nft_payments.clone())
        .run();

    // check that the rewards were claimed
    for nonce in nft_nonces.iter() {
        state
            .world
            .query()
            .to(REWARDS_DISTRIBUTION_ADDRESS_EXPR)
            .typed(rewards_distribution_proxy::RewardsDistributionProxy)
            .was_claimed(0u64, &EgldOrEsdtTokenIdentifier::egld(), 0u64, nonce)
            .returns(ExpectValue(true))
            .run();
    }

    // confirm the received amount matches the sum of the queried rewards
    let alice_balance_after_claim: u64 = expected_rewards.iter().sum();
    let balance_expr: &str = &alice_balance_after_claim.to_string();

    state
        .world
        .check_account(ALICE_ADDRESS_EXPR)
        .balance(balance_expr);

    // a second claim with the same nfts should succeed, but return no more rewards
    let mut reward_tokens: MultiValueEncoded<
        StaticApi,
        MultiValue2<EgldOrEsdtTokenIdentifier<StaticApi>, u64>,
    > = MultiValueEncoded::new();
    reward_tokens.push((EgldOrEsdtTokenIdentifier::egld(), 0).into());
    state
        .world
        .tx()
        .from(ALICE_ADDRESS_EXPR)
        .to(REWARDS_DISTRIBUTION_ADDRESS_EXPR)
        .typed(rewards_distribution_proxy::RewardsDistributionProxy)
        .claim_rewards(0u64, 0u64, reward_tokens)
        .with_multi_token_transfer(nft_payments)
        .run();

    state
        .world
        .check_account(ALICE_ADDRESS_EXPR)
        .balance(balance_expr);
}
