#![allow(deprecated)] // TODO: migrate tests

use core::iter::zip;

use multiversx_sc_scenario::{rust_biguint, testing_framework::BlockchainStateWrapper, DebugApi};
use rewards_distribution::{RewardsDistribution, DIVISION_SAFETY_CONSTANT};

mod utils;

multiversx_sc::imports!();

#[test]
fn test_compute_brackets() {
    DebugApi::dummy();

    let mut wrapper = BlockchainStateWrapper::new();

    let owner = wrapper.create_user_account(&rust_biguint!(0u64));

    let rewards_distribution_sc = wrapper.create_sc_account(
        &rust_biguint!(0u64),
        Some(&owner),
        rewards_distribution::contract_obj,
        "rewards-distribution.wasm",
    );

    wrapper
        .execute_tx(&owner, &rewards_distribution_sc, &rust_biguint!(0), |sc| {
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
        })
        .assert_ok();
}
