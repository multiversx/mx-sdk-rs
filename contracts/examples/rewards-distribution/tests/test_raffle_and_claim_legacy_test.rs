#![allow(deprecated)] // TODO: migrate tests

use multiversx_sc::{
    codec::multi_types::MultiValue2,
    types::{BigUint, EgldOrEsdtTokenIdentifier, MultiValueEncoded, OperationCompletionStatus},
};
use multiversx_sc_scenario::{
    managed_token_id, rust_biguint,
    testing_framework::{BlockchainStateWrapper, TxTokenTransfer},
    DebugApi,
};
use rewards_distribution::RewardsDistribution as _;
mod mock_seed_nft_minter;
use mock_seed_nft_minter::MockSeedNftMinter as _;

mod utils;

#[test]
fn test_raffle_and_claim() {
    DebugApi::dummy();

    let mut wrapper = BlockchainStateWrapper::new();

    let full_reward_amount = rust_biguint!(2_070_000_000u64);
    let nft_count = 10_000u64;

    let owner = wrapper.create_user_account(&rust_biguint!(0));
    let alice = wrapper.create_user_account(&full_reward_amount);

    let nft_token_id = b"NFT-123456";

    let nft_balance = rust_biguint!(1);
    let nft_attributes: [u8; 0] = [];

    let nft_nonces = [1, 2, 3, 4, 5, 6];
    for nft_nonce in nft_nonces {
        wrapper.set_nft_balance(
            &alice,
            b"NFT-123456",
            nft_nonce,
            &nft_balance,
            &nft_attributes,
        );
    }

    let seed_nft_minter_mock_sc = wrapper.create_sc_account(
        &rust_biguint!(0),
        Some(&owner),
        mock_seed_nft_minter::contract_obj,
        "seed nft minter mock",
    );

    // setup the mock contract
    wrapper
        .execute_tx(&alice, &seed_nft_minter_mock_sc, &rust_biguint!(0), |sc| {
            sc.init(managed_token_id!(nft_token_id));

            sc.set_nft_count(nft_count);
        })
        .assert_ok();

    let rewards_distribution_sc = wrapper.create_sc_account(
        &rust_biguint!(0),
        Some(&owner),
        rewards_distribution::contract_obj,
        "rewards distribution",
    );

    // setup the rewards distribution contract
    wrapper
        .execute_tx(&alice, &rewards_distribution_sc, &rust_biguint!(0), |sc| {
            let brackets = utils::to_brackets(&[
                (10, 2_000),
                (90, 6_000),
                (400, 7_000),
                (2_500, 10_000),
                (25_000, 35_000),
                (72_000, 40_000),
            ]);
            sc.init(seed_nft_minter_mock_sc.address_ref().into(), brackets);
        })
        .assert_ok();

    // deposit the royalties
    wrapper
        .execute_tx(
            &alice,
            &rewards_distribution_sc,
            &full_reward_amount,
            |sc| {
                sc.deposit_royalties();
            },
        )
        .assert_ok();

    // run the raffle
    wrapper
        .execute_tx(&alice, &rewards_distribution_sc, &rust_biguint!(0), |sc| {
            let completion_status = sc.raffle();

            assert_eq!(completion_status, OperationCompletionStatus::Completed);
        })
        .assert_ok();

    // post-raffle reward amount frequency checks

    wrapper
        .execute_tx(&alice, &rewards_distribution_sc, &rust_biguint!(0), |sc| {
            // collect the claimable amounts
            let raffle_id = 0;
            let mut rewards: Vec<BigUint<DebugApi>> = Vec::new();

            for nonce in 1u64..=nft_count {
                let amount = sc.compute_claimable_amount(
                    raffle_id,
                    &EgldOrEsdtTokenIdentifier::egld(),
                    0,
                    nonce,
                );
                rewards.push(amount);
            }

            assert_eq!(rewards.len() as u64, nft_count);

            // check that the reward amounts match in frequency
            let expected_reward_amounts = [
                (41_400_000, 1),
                (13_799_999, 9),
                (3_622_500, 40),
                (828_000, 250),
                (289_800, 2500),
                (114_999, 7200),
            ];

            let total_expected_count: u64 =
                expected_reward_amounts.iter().map(|(_, count)| count).sum();
            assert_eq!(total_expected_count, nft_count);

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
        })
        .assert_ok();

    // claim the rewards

    let nft_payments: Vec<TxTokenTransfer> = nft_nonces
        .iter()
        .map(|nonce| TxTokenTransfer {
            token_identifier: nft_token_id.to_vec(),
            nonce: *nonce,
            value: rust_biguint!(1),
        })
        .collect();

    let expected_rewards = [114_999, 114_999, 114_999, 828_000, 114_999, 114_999];
    wrapper
        .execute_esdt_multi_transfer(&alice, &rewards_distribution_sc, &nft_payments, |sc| {
            // get and check the claimable reward amounts for each NFT (sample the few first values)
            let raffle_id = 0;
            assert_eq!(nft_nonces.len(), expected_rewards.len());
            for (nonce, expected_reward) in std::iter::zip(nft_nonces, expected_rewards) {
                let rewards = sc.compute_claimable_amount(
                    raffle_id,
                    &EgldOrEsdtTokenIdentifier::egld(),
                    0,
                    nonce,
                );
                assert_eq!(rewards, expected_reward);
            }

            // claim the rewards
            let reward_id_range_start = 0;
            let reward_id_range_end = 0;
            let mut reward_tokens: MultiValueEncoded<
                DebugApi,
                MultiValue2<EgldOrEsdtTokenIdentifier<DebugApi>, u64>,
            > = MultiValueEncoded::new();
            reward_tokens.push((EgldOrEsdtTokenIdentifier::egld(), 0).into());
            sc.claim_rewards(reward_id_range_start, reward_id_range_end, reward_tokens);

            // check that the flags which mark claimed rewards were set
            for nonce in nft_nonces {
                let was_claimed = sc
                    .was_claimed(raffle_id, &EgldOrEsdtTokenIdentifier::egld(), 0, nonce)
                    .get();
                assert!(was_claimed);
            }
        })
        .assert_ok();

    // confirm the received amount matches the sum of the queried rewards
    let alice_balance_after_claim: u64 = expected_rewards.iter().sum();
    wrapper.check_egld_balance(&alice, &rust_biguint!(alice_balance_after_claim));

    // a second claim with the same nfts should succeed, but return no more rewards
    wrapper
        .execute_esdt_multi_transfer(&alice, &rewards_distribution_sc, &nft_payments, |sc| {
            let reward_id_range_start = 0;
            let reward_id_range_end = 0;
            let mut reward_tokens: MultiValueEncoded<
                DebugApi,
                MultiValue2<EgldOrEsdtTokenIdentifier<DebugApi>, u64>,
            > = MultiValueEncoded::new();
            reward_tokens.push((EgldOrEsdtTokenIdentifier::egld(), 0).into());
            sc.claim_rewards(reward_id_range_start, reward_id_range_end, reward_tokens);
        })
        .assert_ok();

    // check that a second claim does not modify the balance
    wrapper.check_egld_balance(&alice, &rust_biguint!(alice_balance_after_claim));
}
