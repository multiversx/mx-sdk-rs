use std::borrow::Borrow;

use multisig::user_role::UserRole;
use multisig_rust_test_setup::{CallActionDataRaw, MultisigSetup};
use multiversx_sc::types::{BoxedBytes, CodeMetadata, ManagedAddress};
use multiversx_sc_scenario::{managed_address, managed_biguint, rust_biguint, DebugApi};

mod multisig_rust_test_setup;
use adder::Adder;
use factorial::Factorial;
use multisig::Multisig;

use crate::multisig_rust_test_setup::{ActionRaw, EGLD_TOKEN_ID};

#[test]
fn init_test() {
    let _ = MultisigSetup::new(multisig::contract_obj);
}

#[test]
fn add_board_member_test() {
    let rust_zero = rust_biguint!(0);
    let mut ms_setup = MultisigSetup::new(multisig::contract_obj);
    let new_board_member_addr = ms_setup.b_mock.create_user_account(&rust_zero);

    ms_setup
        .b_mock
        .execute_query(&ms_setup.ms_wrapper, |sc| {
            // check role before
            let user_role = sc.user_role(managed_address!(&new_board_member_addr));
            assert_eq!(user_role, UserRole::None);
        })
        .assert_ok();

    let (action_id, tx_result) =
        ms_setup.call_propose(ActionRaw::AddBoardMember(new_board_member_addr.clone()));
    tx_result.assert_ok();

    ms_setup.call_sign(action_id).assert_ok();
    ms_setup.call_perform_action(action_id).assert_ok();

    let prev_board_memeber_addr = ms_setup.board_member_address.clone();
    ms_setup
        .b_mock
        .execute_query(&ms_setup.ms_wrapper, |sc| {
            // check role after
            let user_role = sc.user_role(managed_address!(&new_board_member_addr));
            assert_eq!(user_role, UserRole::BoardMember);

            let board_members = sc.get_all_board_members().to_vec();
            assert_eq!(
                (board_members.get(0).borrow() as &ManagedAddress<DebugApi>).clone(),
                managed_address!(&prev_board_memeber_addr)
            );
            assert_eq!(
                (board_members.get(1).borrow() as &ManagedAddress<DebugApi>).clone(),
                managed_address!(&new_board_member_addr)
            );
        })
        .assert_ok();
}

#[test]
fn add_proposer_test() {
    let rust_zero = rust_biguint!(0);
    let mut ms_setup = MultisigSetup::new(multisig::contract_obj);
    let new_proposer_addr = ms_setup.b_mock.create_user_account(&rust_zero);

    ms_setup
        .b_mock
        .execute_query(&ms_setup.ms_wrapper, |sc| {
            // check role before
            let user_role = sc.user_role(managed_address!(&new_proposer_addr));
            assert_eq!(user_role, UserRole::None);
        })
        .assert_ok();

    let (action_id, tx_result) =
        ms_setup.call_propose(ActionRaw::AddProposer(new_proposer_addr.clone()));
    tx_result.assert_ok();

    ms_setup.call_sign(action_id).assert_ok();
    ms_setup.call_perform_action(action_id).assert_ok();

    let prev_proposer_addr = ms_setup.proposer_address.clone();
    ms_setup
        .b_mock
        .execute_query(&ms_setup.ms_wrapper, |sc| {
            // check role after
            let user_role = sc.user_role(managed_address!(&new_proposer_addr));
            assert_eq!(user_role, UserRole::Proposer);

            let proposers = sc.get_all_proposers().to_vec();
            assert_eq!(
                (proposers.get(0).borrow() as &ManagedAddress<DebugApi>).clone(),
                managed_address!(&prev_proposer_addr)
            );
            assert_eq!(
                (proposers.get(1).borrow() as &ManagedAddress<DebugApi>).clone(),
                managed_address!(&new_proposer_addr)
            );
        })
        .assert_ok();
}

#[test]
fn remove_proposer_test() {
    let mut ms_setup = MultisigSetup::new(multisig::contract_obj);

    let proposer_addr = ms_setup.proposer_address.clone();
    ms_setup
        .b_mock
        .execute_query(&ms_setup.ms_wrapper, |sc| {
            // check role before
            let user_role = sc.user_role(managed_address!(&proposer_addr));
            assert_eq!(user_role, UserRole::Proposer);
        })
        .assert_ok();

    let (action_id, tx_result) =
        ms_setup.call_propose(ActionRaw::RemoveUser(proposer_addr.clone()));
    tx_result.assert_ok();

    ms_setup.call_sign(action_id).assert_ok();
    ms_setup.call_perform_action(action_id).assert_ok();

    ms_setup
        .b_mock
        .execute_query(&ms_setup.ms_wrapper, |sc| {
            // check role after
            let user_role = sc.user_role(managed_address!(&proposer_addr));
            assert_eq!(user_role, UserRole::None);

            let proposers = sc.get_all_proposers().to_vec();
            assert!(proposers.is_empty());
        })
        .assert_ok();
}

#[test]
fn try_remove_all_board_members_test() {
    let mut ms_setup = MultisigSetup::new(multisig::contract_obj);

    let (action_id, tx_result) =
        ms_setup.call_propose(ActionRaw::RemoveUser(ms_setup.board_member_address.clone()));
    tx_result.assert_ok();

    ms_setup.call_sign(action_id).assert_ok();
    ms_setup
        .call_perform_action(action_id)
        .assert_user_error("quorum cannot exceed board size");
}

#[test]
fn change_quorum_test() {
    let rust_zero = rust_biguint!(0);
    let mut ms_setup = MultisigSetup::new(multisig::contract_obj);
    let new_quorum_size = 2;

    // try change quorum > board size
    let (first_action_id, tx_result) =
        ms_setup.call_propose(ActionRaw::ChangeQuorum(new_quorum_size));
    tx_result.assert_ok();

    ms_setup.call_sign(first_action_id).assert_ok();
    ms_setup
        .call_perform_action(first_action_id)
        .assert_user_error("quorum cannot exceed board size");

    // try discard before unsigning
    ms_setup
        .call_discard_action(first_action_id)
        .assert_user_error("cannot discard action with valid signatures");

    // unsign and discard action
    ms_setup.call_unsign(first_action_id).assert_ok();
    ms_setup.call_discard_action(first_action_id).assert_ok();

    // try sign discarded action
    ms_setup
        .call_sign(first_action_id)
        .assert_user_error("action does not exist");

    // add another board member
    let new_board_member_addr = ms_setup.b_mock.create_user_account(&rust_zero);
    let (second_action_id, tx_result) =
        ms_setup.call_propose(ActionRaw::AddBoardMember(new_board_member_addr));
    tx_result.assert_ok();

    ms_setup.call_sign(second_action_id).assert_ok();
    ms_setup.call_perform_action(second_action_id).assert_ok();

    // change quorum to 2
    let (third_action_id, tx_result) =
        ms_setup.call_propose(ActionRaw::ChangeQuorum(new_quorum_size));
    tx_result.assert_ok();

    ms_setup.call_sign(third_action_id).assert_ok();
    ms_setup.call_perform_action(third_action_id).assert_ok();
}

#[test]
fn transfer_execute_to_user_test() {
    let rust_zero = rust_biguint!(0);
    let mut ms_setup = MultisigSetup::new(multisig::contract_obj);
    let user_addr = ms_setup.b_mock.create_user_account(&rust_zero);
    let egld_amount = 100;

    ms_setup
        .call_deposit(EGLD_TOKEN_ID, egld_amount)
        .assert_ok();

    ms_setup.b_mock.check_egld_balance(
        ms_setup.ms_wrapper.address_ref(),
        &rust_biguint!(egld_amount),
    );

    // failed attempt
    let (_, tx_result) = ms_setup.call_propose(ActionRaw::SendTransferExecute(CallActionDataRaw {
        to: user_addr.clone(),
        egld_amount: rust_biguint!(0),
        endpoint_name: BoxedBytes::empty(),
        arguments: Vec::new(),
    }));
    tx_result.assert_user_error("proposed action has no effect");

    // propose
    let (action_id, tx_result) =
        ms_setup.call_propose(ActionRaw::SendTransferExecute(CallActionDataRaw {
            to: user_addr.clone(),
            egld_amount: rust_biguint!(egld_amount),
            endpoint_name: BoxedBytes::empty(),
            arguments: Vec::new(),
        }));
    tx_result.assert_ok();

    ms_setup.call_sign(action_id).assert_ok();
    ms_setup.call_perform_action(action_id).assert_ok();

    ms_setup
        .b_mock
        .check_egld_balance(&user_addr, &rust_biguint!(egld_amount));
}

#[test]
fn transfer_execute_sc_call_test() {
    let rust_zero = rust_biguint!(0);
    let mut ms_setup = MultisigSetup::new(multisig::contract_obj);
    let adder_owner = ms_setup.b_mock.create_user_account(&rust_zero);
    let adder_wrapper = ms_setup.b_mock.create_sc_account(
        &rust_zero,
        Some(&adder_owner),
        adder::contract_obj,
        "path",
    );

    ms_setup
        .b_mock
        .execute_tx(&adder_owner, &adder_wrapper, &rust_zero, |sc| {
            sc.init(managed_biguint!(5));
        })
        .assert_ok();

    let (action_id, tx_result) =
        ms_setup.call_propose(ActionRaw::SendTransferExecute(CallActionDataRaw {
            to: adder_wrapper.address_ref().clone(),
            egld_amount: rust_zero,
            endpoint_name: BoxedBytes::from(&b"add"[..]),
            arguments: vec![BoxedBytes::from(&[5u8][..])],
        }));
    tx_result.assert_ok();

    ms_setup.call_sign(action_id).assert_ok();
    ms_setup.call_perform_action(action_id).assert_ok();

    ms_setup
        .b_mock
        .execute_query(&adder_wrapper, |sc| {
            let actual_sum = sc.sum().get();
            let expected_sum = managed_biguint!(10);
            assert_eq!(actual_sum, expected_sum);
        })
        .assert_ok();
}

#[test]
fn async_call_to_sc_test() {
    let rust_zero = rust_biguint!(0);
    let mut ms_setup = MultisigSetup::new(multisig::contract_obj);
    let adder_owner = ms_setup.b_mock.create_user_account(&rust_zero);
    let adder_wrapper = ms_setup.b_mock.create_sc_account(
        &rust_zero,
        Some(&adder_owner),
        adder::contract_obj,
        "path",
    );

    ms_setup
        .b_mock
        .execute_tx(&adder_owner, &adder_wrapper, &rust_zero, |sc| {
            sc.init(managed_biguint!(5));
        })
        .assert_ok();

    let (action_id, tx_result) =
        ms_setup.call_propose(ActionRaw::SendAsyncCall(CallActionDataRaw {
            to: adder_wrapper.address_ref().clone(),
            egld_amount: rust_zero,
            endpoint_name: BoxedBytes::from(&b"add"[..]),
            arguments: vec![BoxedBytes::from(&[5u8][..])],
        }));
    tx_result.assert_ok();

    ms_setup.call_sign(action_id).assert_ok();

    let perform_action_result = ms_setup.call_perform_action(action_id);
    perform_action_result.assert_ok();

    ms_setup
        .b_mock
        .execute_query(&adder_wrapper, |sc| {
            let actual_sum = sc.sum().get();
            let expected_sum = managed_biguint!(10);
            assert_eq!(actual_sum, expected_sum);
        })
        .assert_ok();
}

#[test]
fn deploy_and_upgrade_from_source_test() {
    let rust_zero = rust_biguint!(0);
    let mut ms_setup = MultisigSetup::new(multisig::contract_obj);

    // init deploy source SC
    let adder_owner = ms_setup.b_mock.create_user_account(&rust_zero);
    let adder_wrapper = ms_setup.b_mock.create_sc_account(
        &rust_zero,
        Some(&adder_owner),
        adder::contract_obj,
        "path",
    );

    ms_setup
        .b_mock
        .execute_tx(&adder_owner, &adder_wrapper, &rust_zero, |sc| {
            sc.init(managed_biguint!(5));
        })
        .assert_ok();

    // deploy from source

    let ms_addr = ms_setup.ms_wrapper.address_ref().clone();
    let new_adder_wrapper = ms_setup
        .b_mock
        .prepare_deploy_from_sc(&ms_addr, adder::contract_obj);

    let (deploy_action_id, tx_result) = ms_setup.call_propose(ActionRaw::SCDeployFromSource {
        source: adder_wrapper.address_ref().clone(),
        amount: rust_zero.clone(),
        code_metadata: CodeMetadata::all(),
        arguments: vec![BoxedBytes::from(&[5u8][..])],
    });
    tx_result.assert_ok();

    ms_setup.call_sign(deploy_action_id).assert_ok();
    let (tx_result, new_adder_addr_from_result) =
        ms_setup.call_perform_action_with_result(deploy_action_id);
    tx_result.assert_ok();

    assert_eq!(new_adder_wrapper.address_ref(), &new_adder_addr_from_result);

    // call the new SC

    let (action_id, tx_result) =
        ms_setup.call_propose(ActionRaw::SendTransferExecute(CallActionDataRaw {
            to: new_adder_wrapper.address_ref().clone(),
            egld_amount: rust_zero.clone(),
            endpoint_name: BoxedBytes::from(&b"add"[..]),
            arguments: vec![BoxedBytes::from(&[5u8][..])],
        }));
    tx_result.assert_ok();

    ms_setup.call_sign(action_id).assert_ok();
    ms_setup.call_perform_action(action_id).assert_ok();

    ms_setup
        .b_mock
        .execute_query(&new_adder_wrapper, |sc| {
            let actual_sum = sc.sum().get();
            let expected_sum = managed_biguint!(10);
            assert_eq!(actual_sum, expected_sum);
        })
        .assert_ok();

    // init upgrade source SC
    let fact_owner = ms_setup.b_mock.create_user_account(&rust_zero);
    let fact_wrapper = ms_setup.b_mock.create_sc_account(
        &rust_zero,
        Some(&fact_owner),
        factorial::contract_obj,
        "path222",
    );

    // upgrade adder to factorial
    let (upgrade_action_id, tx_result) = ms_setup.call_propose(ActionRaw::SCUpgradeFromSource {
        source: fact_wrapper.address_ref().clone(),
        amount: rust_zero.clone(),
        code_metadata: CodeMetadata::all(),
        arguments: Vec::new(),
        sc_address: adder_wrapper.address_ref().clone(),
    });
    tx_result.assert_ok();

    ms_setup.call_sign(upgrade_action_id).assert_ok();
    ms_setup.call_perform_action(upgrade_action_id).assert_ok();

    tx_result.assert_ok();

    let after_upgrade_wrapper = ms_setup
        .b_mock
        .upgrade_wrapper(adder_wrapper, factorial::contract_obj);

    // call SC after upgrade

    ms_setup
        .b_mock
        .execute_query(&after_upgrade_wrapper, |sc| {
            let actual_fact = sc.factorial(managed_biguint!(5));
            let expected_fact = managed_biguint!(120);
            assert_eq!(actual_fact, expected_fact);
        })
        .assert_ok();
}
