#![allow(deprecated)] // TODO: migrate tests

use multiversx_sc::types::{EgldOrEsdtTokenIdentifier, ManagedVec};
use multiversx_sc_modules::staking::StakingModule;
use multiversx_sc_scenario::{
    managed_address, managed_biguint, managed_token_id, rust_biguint,
    testing_framework::BlockchainStateWrapper,
};

static STAKING_TOKEN_ID: &[u8] = b"STAKE-123456";
const INITIAL_BALANCE: u64 = 2_000_000;
const REQUIRED_STAKE_AMOUNT: u64 = 1_000_000;
const SLASH_AMOUNT: u64 = 600_000;
const QUORUM: usize = 2;

#[test]
fn staking_module_test() {
    // setup accounts
    let rust_zero = rust_biguint!(0);
    let mut b_mock = BlockchainStateWrapper::new();
    let owner = b_mock.create_user_account(&rust_zero);
    let alice = b_mock.create_user_account(&rust_zero);
    let bob = b_mock.create_user_account(&rust_zero);
    let carol = b_mock.create_user_account(&rust_zero);
    let eve = b_mock.create_user_account(&rust_zero);
    let staking_sc = b_mock.create_sc_account(
        &rust_zero,
        Some(&owner),
        use_module::contract_obj,
        "wasm path",
    );

    b_mock.set_esdt_balance(&alice, STAKING_TOKEN_ID, &rust_biguint!(INITIAL_BALANCE));
    b_mock.set_esdt_balance(&bob, STAKING_TOKEN_ID, &rust_biguint!(INITIAL_BALANCE));
    b_mock.set_esdt_balance(&carol, STAKING_TOKEN_ID, &rust_biguint!(INITIAL_BALANCE));
    b_mock.set_esdt_balance(&eve, STAKING_TOKEN_ID, &rust_biguint!(INITIAL_BALANCE));

    // init module
    b_mock
        .execute_tx(&owner, &staking_sc, &rust_zero, |sc| {
            let mut whitelist = ManagedVec::new();
            whitelist.push(managed_address!(&alice));
            whitelist.push(managed_address!(&bob));
            whitelist.push(managed_address!(&carol));

            sc.init_staking_module(
                &EgldOrEsdtTokenIdentifier::esdt(managed_token_id!(STAKING_TOKEN_ID)),
                &managed_biguint!(REQUIRED_STAKE_AMOUNT),
                &managed_biguint!(SLASH_AMOUNT),
                QUORUM,
                &whitelist,
            );
        })
        .assert_ok();

    // try stake - not a board member
    b_mock
        .execute_esdt_transfer(
            &eve,
            &staking_sc,
            STAKING_TOKEN_ID,
            0,
            &rust_biguint!(REQUIRED_STAKE_AMOUNT),
            |sc| {
                sc.stake();
            },
        )
        .assert_user_error("Only whitelisted members can stake");

    // stake half and try unstake
    b_mock
        .execute_esdt_transfer(
            &alice,
            &staking_sc,
            STAKING_TOKEN_ID,
            0,
            &rust_biguint!(REQUIRED_STAKE_AMOUNT / 2),
            |sc| {
                sc.stake();
            },
        )
        .assert_ok();
    b_mock
        .execute_tx(&alice, &staking_sc, &rust_zero, |sc| {
            sc.unstake(managed_biguint!(REQUIRED_STAKE_AMOUNT / 4));
        })
        .assert_user_error("Not enough stake");

    // bob and carol stake
    b_mock
        .execute_esdt_transfer(
            &bob,
            &staking_sc,
            STAKING_TOKEN_ID,
            0,
            &rust_biguint!(REQUIRED_STAKE_AMOUNT),
            |sc| {
                sc.stake();
            },
        )
        .assert_ok();
    b_mock
        .execute_esdt_transfer(
            &carol,
            &staking_sc,
            STAKING_TOKEN_ID,
            0,
            &rust_biguint!(REQUIRED_STAKE_AMOUNT),
            |sc| {
                sc.stake();
            },
        )
        .assert_ok();

    // try vote slash, not enough stake
    b_mock
        .execute_tx(&alice, &staking_sc, &rust_zero, |sc| {
            sc.vote_slash_member(managed_address!(&bob));
        })
        .assert_user_error("Not enough stake");

    // try vote slash, slashed address not a board member
    b_mock
        .execute_tx(&bob, &staking_sc, &rust_zero, |sc| {
            sc.vote_slash_member(managed_address!(&eve));
        })
        .assert_user_error("Voted user is not a staked board member");

    // alice stake over max amount and withdraw surplus
    b_mock
        .execute_esdt_transfer(
            &alice,
            &staking_sc,
            STAKING_TOKEN_ID,
            0,
            &rust_biguint!(REQUIRED_STAKE_AMOUNT),
            |sc| {
                sc.stake();

                let alice_staked_amount = sc.staked_amount(&managed_address!(&alice)).get();
                assert_eq!(alice_staked_amount, managed_biguint!(1_500_000));
            },
        )
        .assert_ok();
    b_mock
        .execute_tx(&alice, &staking_sc, &rust_zero, |sc| {
            sc.unstake(managed_biguint!(500_000));

            let alice_staked_amount = sc.staked_amount(&managed_address!(&alice)).get();
            assert_eq!(alice_staked_amount, managed_biguint!(1_000_000));
        })
        .assert_ok();
    b_mock.check_esdt_balance(&alice, STAKING_TOKEN_ID, &rust_biguint!(1_000_000));

    // alice vote to slash bob
    b_mock
        .execute_tx(&alice, &staking_sc, &rust_zero, |sc| {
            sc.vote_slash_member(managed_address!(&bob));

            assert_eq!(
                sc.slashing_proposal_voters(&managed_address!(&bob)).len(),
                1
            );
            assert!(sc
                .slashing_proposal_voters(&managed_address!(&bob))
                .contains(&managed_address!(&alice)));
        })
        .assert_ok();

    // bob vote to slash alice
    b_mock
        .execute_tx(&bob, &staking_sc, &rust_zero, |sc| {
            sc.vote_slash_member(managed_address!(&alice));
        })
        .assert_ok();

    // try slash before quorum reached
    b_mock
        .execute_tx(&bob, &staking_sc, &rust_zero, |sc| {
            sc.slash_member(managed_address!(&alice));
        })
        .assert_user_error("Quorum not reached");

    // carol vote
    b_mock
        .execute_tx(&carol, &staking_sc, &rust_zero, |sc| {
            sc.vote_slash_member(managed_address!(&alice));

            assert_eq!(
                sc.slashing_proposal_voters(&managed_address!(&alice)).len(),
                2
            );
            assert!(sc
                .slashing_proposal_voters(&managed_address!(&alice))
                .contains(&managed_address!(&bob)));
            assert!(sc
                .slashing_proposal_voters(&managed_address!(&alice))
                .contains(&managed_address!(&carol)));
        })
        .assert_ok();

    // slash alice
    b_mock
        .execute_tx(&bob, &staking_sc, &rust_zero, |sc| {
            sc.slash_member(managed_address!(&alice));

            assert_eq!(
                sc.staked_amount(&managed_address!(&alice)).get(),
                managed_biguint!(REQUIRED_STAKE_AMOUNT - SLASH_AMOUNT)
            );
            assert_eq!(
                sc.total_slashed_amount().get(),
                managed_biguint!(SLASH_AMOUNT)
            );
            assert!(sc
                .slashing_proposal_voters(&managed_address!(&alice))
                .is_empty());
        })
        .assert_ok();

    // alice try vote after slash
    b_mock
        .execute_tx(&alice, &staking_sc, &rust_zero, |sc| {
            sc.vote_slash_member(managed_address!(&bob));
        })
        .assert_user_error("Not enough stake");

    // alice try unstake the remaining tokens
    b_mock
        .execute_tx(&alice, &staking_sc, &rust_zero, |sc| {
            sc.unstake(managed_biguint!(400_000));
        })
        .assert_user_error("Not enough stake");

    // alice remove from board members
    b_mock
        .execute_tx(&owner, &staking_sc, &rust_zero, |sc| {
            // check alice's votes before slash
            assert!(sc
                .slashing_proposal_voters(&managed_address!(&bob))
                .contains(&managed_address!(&alice)));

            sc.remove_board_member(&managed_address!(&alice));

            assert_eq!(sc.user_whitelist().len(), 2);
            assert!(!sc.user_whitelist().contains(&managed_address!(&alice)));

            // alice's vote gets removed
            assert!(sc
                .slashing_proposal_voters(&managed_address!(&bob))
                .is_empty());
        })
        .assert_ok();

    // alice unstake ok
    b_mock
        .execute_tx(&alice, &staking_sc, &rust_zero, |sc| {
            sc.unstake(managed_biguint!(400_000));
        })
        .assert_ok();
    b_mock.check_esdt_balance(
        &alice,
        STAKING_TOKEN_ID,
        &rust_biguint!(INITIAL_BALANCE - SLASH_AMOUNT),
    );
}
