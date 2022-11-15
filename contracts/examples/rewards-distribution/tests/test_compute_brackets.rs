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
                (0_010, 2_000),
                (0_090, 6_000),
                (0_400, 7_000),
                (2_500, 10_000),
                (25_000, 35_000),
                (72_000, 40_000),
            ]);

            let computed_brackets = sc.compute_brackets(brackets, 10_000, 2_070_00000u64.into());

            let expected_values = vec![
                (1, 41_40000),
                (10, 13_80000),
                (50, 3_62250),
                (300, 0_82800),
                (2800, 0_28980),
                (10000, 0_11500),
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
