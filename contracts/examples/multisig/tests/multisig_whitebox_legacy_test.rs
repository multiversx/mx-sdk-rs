#![allow(deprecated)] // TODO: migrate tests

use multisig_blackbox_setup::{CallActionDataRaw, MultisigSetup};
use multiversx_sc::types::{BoxedBytes, CodeMetadata};
use multiversx_sc_scenario::{managed_biguint, rust_biguint};

mod multisig_blackbox_setup;
use adder::Adder;
use factorial::Factorial;

use crate::multisig_blackbox_setup::ActionRaw;

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
