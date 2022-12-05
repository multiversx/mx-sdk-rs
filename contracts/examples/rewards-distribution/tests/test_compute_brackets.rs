use core::iter::zip;

use elrond_wasm_debug::{rust_biguint, testing_framework::BlockchainStateWrapper, DebugApi};
use rewards_distribution::RewardsDistribution;

mod utils;

elrond_wasm::imports!();

#[test]
fn test_compute_brackets() {
    let _ = DebugApi::dummy();

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

            let computed_brackets = sc.compute_brackets(brackets, 10_000, 2_070_000_000u64.into());

            let expected_values = vec![
                (1, 41_400_000),
                (10, 13_800_000),
                (50, 3_622_500),
                (300, 828_000),
                (2800, 289_800),
                (10000, 115_000),
            ];

            assert_eq!(computed_brackets.len(), expected_values.len());
            for (computed, expected) in zip(computed_brackets.iter(), expected_values) {
                let (expected_end_index, expected_reward) = expected;
                let reward = computed.reward.to_u64().unwrap();
                assert_eq!(computed.end_index, expected_end_index);
                assert_eq!(reward, expected_reward);
            }
        })
        .assert_ok();
}
