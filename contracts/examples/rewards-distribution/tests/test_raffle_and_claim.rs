use elrond_wasm::types::{BigUint, OperationCompletionStatus};
use elrond_wasm_debug::{
    managed_biguint, managed_token_id, rust_biguint, testing_framework::BlockchainStateWrapper,
    tx_mock::TxInputESDT, DebugApi,
};
use rewards_distribution::RewardsDistribution as _;
mod mock_seed_nft_minter;
use mock_seed_nft_minter::MockSeedNftMinter as _;

mod utils;

#[test]
fn test_raffle_and_claim() {
    let _ = DebugApi::dummy();

    let mut wrapper = BlockchainStateWrapper::new();

    let full_reward_amount = rust_biguint!(2_070_00000u64);
    let nft_count = 10_000u64;

    let owner = wrapper.create_user_account(&rust_biguint!(0));
    let alice = wrapper.create_user_account(&full_reward_amount);

    let nft_token_id = managed_token_id!(b"NFT-123456");

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
            sc.init((&nft_token_id).clone());

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
                (0_010, 2_000),
                (0_090, 6_000),
                (0_400, 7_000),
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
            // check the royalties left after the raffle
            let royalties_remaining = sc.royalties().get();
            assert_eq!(royalties_remaining, managed_biguint!(0));

            let mut rewards: Vec<BigUint<DebugApi>> = Vec::new();

            for nonce in 1u64..=nft_count {
                let reward = sc.rewards(nonce).get();
                rewards.push(reward);
            }

            assert_eq!(rewards.len() as u64, nft_count);
            // check that the reward amounts match in frequency
            let expected_reward_amounts = [
                (41_40000, 1),
                (13_80000, 9),
                (3_62250, 40),
                (0_82800, 250),
                (0_28980, 2500),
                (0_11500, 7200),
            ];
            for (amount, expected_count) in expected_reward_amounts {
                let expected_amount_biguint = managed_biguint!(amount as u64);
                assert_eq!(
                    rewards
                        .iter()
                        .filter(|value| *value == &expected_amount_biguint)
                        .count(),
                    expected_count as usize
                );
            }
        })
        .assert_ok();

    // claim the rewards

    let token_identifier = nft_token_id.clone().to_boxed_bytes().into_vec();
    let nft_payments: Vec<TxInputESDT> = nft_nonces
        .iter()
        .map(|nonce| TxInputESDT {
            token_identifier: (&token_identifier).clone(),
            nonce: *nonce,
            value: rust_biguint!(1),
        })
        .collect();

    let expected_rewards = [0_11500, 0_11500, 0_11500, 8_2800, 0_11500, 0_11500];
    wrapper
        .execute_esdt_multi_transfer(&alice, &rewards_distribution_sc, &nft_payments, |sc| {
            // get and check the claimable reward amounts for each NFT (sample the few first values)
            assert_eq!(nft_nonces.len(), expected_rewards.len());
            for (nonce, expected_rewards) in std::iter::zip(nft_nonces, expected_rewards) {
                let rewards = sc.rewards(nonce).get();
                assert_eq!(rewards, managed_biguint!(expected_rewards));
            }

            let mut claimable_amounts: Vec<BigUint<DebugApi>> = Vec::new();

            for nonce in 1u64..=nft_count {
                let claimable = sc.rewards(nonce).get();
                claimable_amounts.push(claimable);
            }

            // claim the rewards
            sc.claim_rewards();

            // check that the claimable rewards are now 0
            for nonce in nft_nonces {
                let rewards = sc.rewards(nonce).get();
                assert_eq!(rewards, managed_biguint!(0));
            }
        })
        .assert_ok();

    // confirm the received amount matches the sum of the queried rewards
    let alice_balance_after_claim: i32 = expected_rewards.iter().sum();
    wrapper.check_egld_balance(&alice, &rust_biguint!(alice_balance_after_claim));

    // a second claim with the same nfts should succeed, but return no more rewards
    wrapper
        .execute_esdt_multi_transfer(&alice, &rewards_distribution_sc, &nft_payments, |sc| {
            sc.claim_rewards();
        })
        .assert_ok();

    // check that a second claim does not modify the balance
    wrapper.check_egld_balance(&alice, &rust_biguint!(alice_balance_after_claim));
}
