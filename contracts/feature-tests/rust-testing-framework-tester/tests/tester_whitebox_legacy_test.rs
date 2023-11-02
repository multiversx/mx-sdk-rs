#![allow(deprecated)] // TODO: migrate tests

use adder::*;
use forwarder::call_sync::*;
use num_traits::ToPrimitive;

use basic_features::BasicFeatures;
use multiversx_sc::{
    codec::Empty,
    contract_base::ContractBase,
    err_msg,
    types::{Address, BigUint, EsdtLocalRole, EsdtTokenPayment, ManagedVec, TokenIdentifier},
};
use multiversx_sc_scenario::{
    api::DebugApi, assert_values_eq, managed_address, managed_biguint, managed_buffer,
    managed_token_id, rust_biguint, testing_framework::*,
};
use rust_testing_framework_tester::{dummy_module::DummyModule, *};

const TEST_OUTPUT_PATH: &str = "test.scen.json";
const TEST_MULTIPLE_SC_OUTPUT_PATH: &str = "test_multiple_sc.scen.json";
const TEST_ESDT_OUTPUT_PATH: &str = "test_esdt_generation.scen.json";

const SC_WASM_PATH: &str = "output/rust-testing-framework-tester.wasm";
const ADDER_WASM_PATH: &str = "../../examples/adder/output/adder.wasm";
const BASIC_FEATURES_WASM_PATH: &str =
    "../../feature-tests/basic-features/output/basic-features.wasm";

const NFT_TOKEN_ID: &[u8] = b"NFT-123456";
const NFT_AMOUNT: u64 = 1;
const FIRST_NFT_NONCE: u64 = 5;
const FIRST_ATTRIBUTES: &[u8] = b"FirstAttributes";
const FIRST_ROYALTIES: u64 = 1_000;
const SECOND_ROYALTIES: u64 = 5_000;
const FIRST_URIS: &[&[u8]] = &[b"FirstUri", b"SecondUri"];

#[test]
fn test_add() {
    let mut wrapper = BlockchainStateWrapper::new();
    let sc_wrapper = wrapper.create_sc_account(
        &rust_biguint!(0),
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );

    wrapper
        .execute_query(&sc_wrapper, |sc| {
            let first = managed_biguint!(1000);
            let second = managed_biguint!(2000);

            let expected_result = first.clone() + second.clone();
            let actual_result = sc.sum(first, second);
            assert_values_eq!(expected_result, actual_result);
        })
        .assert_ok();
}

#[test]
fn test_add_spawned_thread() {
    let handler = std::thread::spawn(|| {
        let mut wrapper = BlockchainStateWrapper::new();
        let sc_wrapper = wrapper.create_sc_account(
            &rust_biguint!(0),
            None,
            rust_testing_framework_tester::contract_obj,
            SC_WASM_PATH,
        );

        wrapper
            .execute_query(&sc_wrapper, |sc| {
                let first = managed_biguint!(1000);
                let second = managed_biguint!(2000);

                let expected_result = first.clone() + second.clone();
                let actual_result = sc.sum(first, second);
                assert_values_eq!(expected_result, actual_result);
            })
            .assert_ok();
    });

    handler.join().unwrap();
}

#[should_panic]
#[test]
fn test_add_wrong_expect() {
    let mut wrapper = BlockchainStateWrapper::new();
    let sc_wrapper = wrapper.create_sc_account(
        &rust_biguint!(0),
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );

    wrapper
        .execute_query(&sc_wrapper, |sc| {
            let first = managed_biguint!(1000);
            let second = managed_biguint!(2000);

            let expected_result = first.clone() + second.clone() + 1u32;
            let actual_result = sc.sum(first, second);
            assert_values_eq!(expected_result, actual_result);
        })
        .assert_ok();
}

#[test]
fn test_sc_result_ok() {
    let mut wrapper = BlockchainStateWrapper::new();
    let sc_wrapper = wrapper.create_sc_account(
        &rust_biguint!(0),
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );

    wrapper
        .execute_query(&sc_wrapper, |sc| {
            let first = managed_biguint!(1000);
            let second = managed_biguint!(2000);

            let expected_result = first.clone() + second.clone();
            let actual_result = sc.sum_sc_result(first, second);
            assert_eq!(expected_result, actual_result);
        })
        .assert_ok();
}

#[test]
fn test_sc_result_ok_unwrap() {
    let mut wrapper = BlockchainStateWrapper::new();
    let sc_wrapper = wrapper.create_sc_account(
        &rust_biguint!(0),
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );

    wrapper
        .execute_query(&sc_wrapper, |sc| {
            let first = managed_biguint!(1000);
            let second = managed_biguint!(2000);

            let expected_result = first.clone() + second.clone();
            let actual_result = sc.sum_sc_result(first, second);
            assert_eq!(expected_result, actual_result);
        })
        .assert_ok();
}

#[test]
fn test_sc_result_err() {
    let mut wrapper = BlockchainStateWrapper::new();
    let sc_wrapper = wrapper.create_sc_account(
        &rust_biguint!(0),
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );

    wrapper
        .execute_query(&sc_wrapper, |sc| {
            let first = managed_biguint!(0);
            let second = managed_biguint!(2000);

            let _ = sc.sum_sc_result(first, second);
        })
        .assert_user_error("Non-zero required");
}

#[should_panic(
    expected = "Tx success expected, but failed. Status: 4, message: \"Non-zero required\""
)]
#[test]
fn test_sc_result_err_unwrap() {
    let mut wrapper = BlockchainStateWrapper::new();
    let sc_wrapper = wrapper.create_sc_account(
        &rust_biguint!(0),
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );

    wrapper
        .execute_query(&sc_wrapper, |sc| {
            let first = managed_biguint!(0);
            let second = managed_biguint!(2000);

            let _ = sc.sum_sc_result(first, second);
        })
        .assert_ok();
}

#[should_panic(
    expected = "Tx error message mismatch. Want status 4, message \"Non-zero required\". Have status 0, message \"\""
)]
#[test]
fn test_assert_err_with_ok() {
    let mut wrapper = BlockchainStateWrapper::new();
    let sc_wrapper = wrapper.create_sc_account(
        &rust_biguint!(0),
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );

    wrapper
        .execute_query(&sc_wrapper, |sc| {
            let first = managed_biguint!(1000);
            let second = managed_biguint!(2000);

            let _ = sc.sum_sc_result(first, second);
            // assert_sc_panic!(actual_result, "Non-zero required");
        })
        .assert_user_error("Non-zero required");
}

#[test]
fn test_sc_payment_ok() {
    let mut wrapper = BlockchainStateWrapper::new();
    let caller_addr = wrapper.create_user_account(&rust_biguint!(1_000));
    let sc_wrapper = wrapper.create_sc_account(
        &rust_biguint!(2_000),
        Some(&caller_addr),
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );

    wrapper
        .execute_tx(&caller_addr, &sc_wrapper, &rust_biguint!(1_000), |sc| {
            let actual_payment = sc.receive_egld();
            let expected_payment = managed_biguint!(1_000);
            assert_eq!(actual_payment, expected_payment);
        })
        .assert_ok();

    wrapper.check_egld_balance(&caller_addr, &rust_biguint!(0));
    wrapper.check_egld_balance(sc_wrapper.address_ref(), &rust_biguint!(3_000));
}

#[test]
fn test_sc_payment_reverted() {
    let mut wrapper = BlockchainStateWrapper::new();
    let caller_addr = wrapper.create_user_account(&rust_biguint!(1_000));
    let sc_wrapper = wrapper.create_sc_account(
        &rust_biguint!(2_000),
        Some(&caller_addr),
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );

    wrapper
        .execute_tx(&caller_addr, &sc_wrapper, &rust_biguint!(1_000), |sc| {
            sc.reject_payment();
        })
        .assert_user_error("No payment allowed!");

    wrapper.check_egld_balance(&caller_addr, &rust_biguint!(1_000));
    wrapper.check_egld_balance(sc_wrapper.address_ref(), &rust_biguint!(2_000));
}

#[test]
fn test_sc_half_payment() {
    let mut wrapper = BlockchainStateWrapper::new();
    let caller_addr = wrapper.create_user_account(&rust_biguint!(1_000));
    let sc_wrapper = wrapper.create_sc_account(
        &rust_biguint!(2_000),
        Some(&caller_addr),
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );

    wrapper
        .execute_tx(&caller_addr, &sc_wrapper, &rust_biguint!(1_000), |sc| {
            sc.recieve_egld_half();
        })
        .assert_ok();

    wrapper.check_egld_balance(&caller_addr, &rust_biguint!(500));
    wrapper.check_egld_balance(sc_wrapper.address_ref(), &rust_biguint!(2_500));
}

#[test]
fn test_esdt_balance() {
    let mut wrapper = BlockchainStateWrapper::new();
    let sc_wrapper = wrapper.create_sc_account(
        &rust_biguint!(0),
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );
    let token_id = &b"COOL-123456"[..];

    wrapper.set_esdt_balance(sc_wrapper.address_ref(), token_id, &rust_biguint!(1_000));
    wrapper.check_esdt_balance(sc_wrapper.address_ref(), token_id, &rust_biguint!(1_000));

    wrapper
        .execute_query(&sc_wrapper, |sc| {
            let managed_id = managed_token_id!(token_id);

            let actual_balance = sc.get_esdt_balance(managed_id, 0);
            let expected_balance = managed_biguint!(1_000);
            assert_eq!(expected_balance, actual_balance);
        })
        .assert_ok();

    wrapper.add_mandos_check_account(sc_wrapper.address_ref());
    wrapper.write_mandos_output(TEST_ESDT_OUTPUT_PATH);
}

#[test]
fn test_esdt_payment_ok() {
    let mut wrapper = BlockchainStateWrapper::new();
    let rust_zero = rust_biguint!(0);

    let caller_addr = wrapper.create_user_account(&rust_zero);
    let sc_wrapper = wrapper.create_sc_account(
        &rust_zero,
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );
    let token_id = &b"COOL-123456"[..];

    wrapper.set_esdt_balance(&caller_addr, token_id, &rust_biguint!(1_000));
    wrapper.set_esdt_balance(sc_wrapper.address_ref(), token_id, &rust_biguint!(2_000));

    wrapper
        .execute_esdt_transfer(
            &caller_addr,
            &sc_wrapper,
            token_id,
            0,
            &rust_biguint!(1_000),
            |sc| {
                let (actual_token_id, actual_payment) = sc.receive_esdt();
                let expected_payment = managed_biguint!(1_000);

                assert_eq!(actual_token_id, managed_token_id!(token_id));
                assert_eq!(actual_payment, expected_payment);
            },
        )
        .assert_ok();

    wrapper.check_esdt_balance(&caller_addr, token_id, &rust_zero);
    wrapper.check_esdt_balance(sc_wrapper.address_ref(), token_id, &rust_biguint!(3_000));
}

#[test]
fn test_esdt_payment_reverted() {
    let mut wrapper = BlockchainStateWrapper::new();
    let rust_zero = rust_biguint!(0);

    let caller_addr = wrapper.create_user_account(&rust_zero);
    let sc_wrapper = wrapper.create_sc_account(
        &rust_zero,
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );
    let token_id = &b"COOL-123456"[..];

    wrapper.set_esdt_balance(&caller_addr, token_id, &rust_biguint!(1_000));
    wrapper.set_esdt_balance(sc_wrapper.address_ref(), token_id, &rust_biguint!(2_000));

    wrapper
        .execute_esdt_transfer(
            &caller_addr,
            &sc_wrapper,
            token_id,
            0,
            &rust_biguint!(1_000),
            |sc| {
                sc.reject_payment();
            },
        )
        .assert_user_error("No payment allowed!");

    wrapper.check_esdt_balance(&caller_addr, token_id, &rust_biguint!(1_000));
    wrapper.check_esdt_balance(sc_wrapper.address_ref(), token_id, &rust_biguint!(2_000));
}

#[test]
fn test_nft_balance() {
    let mut wrapper = BlockchainStateWrapper::new();
    let sc_wrapper = wrapper.create_sc_account(
        &rust_biguint!(0),
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );
    let token_id = &b"COOL-123456"[..];
    let nft_nonce = 2;
    let nft_balance = rust_biguint!(1_000);
    let nft_attributes = NftDummyAttributes {
        creation_epoch: 666,
        cool_factor: 101,
    };

    wrapper.set_nft_balance(
        sc_wrapper.address_ref(),
        token_id,
        nft_nonce,
        &nft_balance,
        &nft_attributes,
    );
    wrapper.check_nft_balance(
        sc_wrapper.address_ref(),
        token_id,
        nft_nonce,
        &nft_balance,
        Some(&nft_attributes),
    );

    wrapper
        .execute_query(&sc_wrapper, |sc| {
            let managed_id = managed_token_id!(token_id);

            let actual_balance = sc.get_esdt_balance(managed_id, nft_nonce);
            let expected_balance = managed_biguint!(1_000);
            assert_eq!(expected_balance, actual_balance);
        })
        .assert_ok();
}

#[should_panic]
#[test]
fn check_nft_zero_balance() {
    let mut wrapper = BlockchainStateWrapper::new();
    let sc_wrapper = wrapper.create_sc_account(
        &rust_biguint!(0),
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );
    let token_id = &b"COOL-123456"[..];
    let nft_nonce = 2;
    let nft_balance = rust_biguint!(1_000);
    let nft_attributes = NftDummyAttributes {
        creation_epoch: 666,
        cool_factor: 101,
    };

    wrapper.check_nft_balance(
        sc_wrapper.address_ref(),
        token_id,
        nft_nonce,
        &nft_balance,
        Some(&nft_attributes),
    );
}

#[test]
fn test_sc_send_nft_to_user() {
    let mut wrapper = BlockchainStateWrapper::new();
    let caller_addr = wrapper.create_user_account(&rust_biguint!(0));
    let sc_wrapper = wrapper.create_sc_account(
        &rust_biguint!(0),
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );
    let token_id = &b"COOL-123456"[..];
    let nft_nonce = 2;
    let nft_balance = rust_biguint!(1_000);
    let nft_attributes = NftDummyAttributes {
        creation_epoch: 666,
        cool_factor: 101,
    };

    wrapper.set_nft_balance(
        sc_wrapper.address_ref(),
        token_id,
        nft_nonce,
        &nft_balance,
        &nft_attributes,
    );
    wrapper.check_nft_balance(
        sc_wrapper.address_ref(),
        token_id,
        nft_nonce,
        &nft_balance,
        Some(&nft_attributes),
    );

    wrapper
        .execute_tx(&caller_addr, &sc_wrapper, &rust_biguint!(0), |sc| {
            let managed_addr = managed_address!(&caller_addr);
            let managed_id = managed_token_id!(token_id);
            let managed_amt = managed_biguint!(400);
            sc.send_nft(managed_addr, managed_id, nft_nonce, managed_amt);
        })
        .assert_ok();

    wrapper.check_nft_balance(
        &caller_addr,
        token_id,
        nft_nonce,
        &rust_biguint!(400),
        Some(&nft_attributes),
    );
    wrapper.check_nft_balance(
        sc_wrapper.address_ref(),
        token_id,
        nft_nonce,
        &rust_biguint!(600),
        Some(&nft_attributes),
    );
}

#[test]
fn test_sc_esdt_mint_burn() {
    let mut wrapper = BlockchainStateWrapper::new();
    let caller_addr = wrapper.create_user_account(&rust_biguint!(0));
    let sc_wrapper = wrapper.create_sc_account(
        &rust_biguint!(0),
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );
    let token_id = &b"COOL-123456"[..];

    wrapper.set_esdt_local_roles(
        sc_wrapper.address_ref(),
        token_id,
        &[EsdtLocalRole::Mint, EsdtLocalRole::Burn][..],
    );

    wrapper
        .execute_tx(&caller_addr, &sc_wrapper, &rust_biguint!(0), |sc| {
            let managed_id = managed_token_id!(token_id);
            let managed_amt = managed_biguint!(400);
            sc.mint_esdt(managed_id, 0, managed_amt);
        })
        .assert_ok();

    wrapper.check_esdt_balance(sc_wrapper.address_ref(), token_id, &rust_biguint!(400));

    wrapper
        .execute_tx(&caller_addr, &sc_wrapper, &rust_biguint!(0), |sc| {
            let managed_id = managed_token_id!(token_id);
            let managed_amt = managed_biguint!(100);
            sc.burn_esdt(managed_id, 0, managed_amt);
        })
        .assert_ok();

    wrapper.check_esdt_balance(sc_wrapper.address_ref(), token_id, &rust_biguint!(300));
}

#[test]
fn test_sc_nft() {
    let mut wrapper = BlockchainStateWrapper::new();
    let caller_addr = wrapper.create_user_account(&rust_biguint!(0));
    let sc_wrapper = wrapper.create_sc_account(
        &rust_biguint!(0),
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );
    let token_id = &b"COOL-123456"[..];
    let nft_attributes = NftDummyAttributes {
        creation_epoch: 666,
        cool_factor: 101,
    };

    wrapper.set_esdt_local_roles(
        sc_wrapper.address_ref(),
        token_id,
        &[
            EsdtLocalRole::NftCreate,
            EsdtLocalRole::NftAddQuantity,
            EsdtLocalRole::NftBurn,
        ][..],
    );

    wrapper
        .execute_tx(&caller_addr, &sc_wrapper, &rust_biguint!(0), |sc| {
            let managed_id = managed_token_id!(token_id);
            let managed_amt = managed_biguint!(100);

            let nft_nonce = sc.create_nft(
                managed_id.clone(),
                managed_amt.clone(),
                nft_attributes.clone(),
            );
            assert_eq!(nft_nonce, 1u64);

            let nft_nonce_second = sc.create_nft(managed_id, managed_amt, nft_attributes.clone());
            assert_eq!(nft_nonce_second, 2u64);
        })
        .assert_ok();

    wrapper.check_nft_balance(
        sc_wrapper.address_ref(),
        token_id,
        1,
        &rust_biguint!(100),
        Some(&nft_attributes),
    );
    wrapper.check_nft_balance(
        sc_wrapper.address_ref(),
        token_id,
        2,
        &rust_biguint!(100),
        Some(&nft_attributes),
    );

    wrapper
        .execute_tx(&caller_addr, &sc_wrapper, &rust_biguint!(0), |sc| {
            let managed_id = managed_token_id!(token_id);
            let managed_amt = managed_biguint!(100);
            sc.mint_esdt(managed_id, 1, managed_amt);
        })
        .assert_ok();

    wrapper.check_nft_balance(
        sc_wrapper.address_ref(),
        token_id,
        1,
        &rust_biguint!(200),
        Some(&nft_attributes),
    );
    wrapper.check_nft_balance(
        sc_wrapper.address_ref(),
        token_id,
        2,
        &rust_biguint!(100),
        Some(&nft_attributes),
    );

    wrapper
        .execute_tx(&caller_addr, &sc_wrapper, &rust_biguint!(0), |sc| {
            let managed_id = managed_token_id!(token_id);
            let managed_amt = managed_biguint!(50);
            sc.burn_esdt(managed_id, 2, managed_amt);
        })
        .assert_ok();

    wrapper.check_nft_balance(
        sc_wrapper.address_ref(),
        token_id,
        1,
        &rust_biguint!(200),
        Some(&nft_attributes),
    );
    wrapper.check_nft_balance(
        sc_wrapper.address_ref(),
        token_id,
        2,
        &rust_biguint!(50),
        Some(&nft_attributes),
    );
}

#[test]
fn test_over_set_nft() {
    let rust_zero = rust_biguint!(0);
    let mut b_mock = BlockchainStateWrapper::new();
    let user = b_mock.create_user_account(&rust_zero);
    let sc_wrapper = b_mock.create_sc_account(
        &rust_zero,
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );

    b_mock.set_nft_balance_all_properties(
        &user,
        NFT_TOKEN_ID,
        FIRST_NFT_NONCE,
        &rust_biguint!(NFT_AMOUNT),
        &FIRST_ATTRIBUTES.to_vec(),
        FIRST_ROYALTIES,
        None,
        None,
        None,
        &uris_to_vec(FIRST_URIS),
    );
    b_mock.set_nft_balance_all_properties(
        &user,
        NFT_TOKEN_ID,
        FIRST_NFT_NONCE,
        &rust_biguint!(NFT_AMOUNT),
        &FIRST_ATTRIBUTES.to_vec(),
        SECOND_ROYALTIES,
        None,
        None,
        None,
        &uris_to_vec(FIRST_URIS),
    );

    b_mock
        .execute_tx(&user, &sc_wrapper, &rust_zero, |sc| {
            let merged_token_data = sc.blockchain().get_esdt_token_data(
                &managed_address!(&user),
                &managed_token_id!(NFT_TOKEN_ID),
                FIRST_NFT_NONCE,
            );
            assert_eq!(
                merged_token_data.royalties,
                managed_biguint!(SECOND_ROYALTIES)
            );
        })
        .assert_ok();
}

#[test]
fn test_esdt_multi_transfer() {
    let mut wrapper = BlockchainStateWrapper::new();
    let caller_addr = wrapper.create_user_account(&rust_biguint!(0));
    let sc_wrapper = wrapper.create_sc_account(
        &rust_biguint!(0),
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );
    let token_id_1 = &b"COOL-123456"[..];
    let token_id_2 = &b"VERYCOOL-123456"[..];
    let nft_nonce = 5;

    wrapper.set_esdt_balance(&caller_addr, token_id_1, &rust_biguint!(100));
    wrapper.set_nft_balance(
        &caller_addr,
        token_id_2,
        nft_nonce,
        &rust_biguint!(1),
        &Empty,
    );

    let transfers = vec![
        TxTokenTransfer {
            token_identifier: token_id_1.to_vec(),
            nonce: 0,
            value: rust_biguint!(100),
        },
        TxTokenTransfer {
            token_identifier: token_id_2.to_vec(),
            nonce: nft_nonce,
            value: rust_biguint!(1),
        },
    ];

    wrapper
        .execute_esdt_multi_transfer(&caller_addr, &sc_wrapper, &transfers, |sc| {
            let mut expected_transfers = Vec::new();
            expected_transfers.push(EsdtTokenPayment::new(
                managed_token_id!(token_id_1),
                0,
                managed_biguint!(100),
            ));
            expected_transfers.push(EsdtTokenPayment::new(
                managed_token_id!(token_id_2),
                nft_nonce,
                managed_biguint!(1),
            ));

            let actual_transfers = sc.receive_multi_esdt().into_vec();
            assert_eq!(
                expected_transfers[0].token_identifier,
                actual_transfers[0].token_identifier
            );
            assert_eq!(
                expected_transfers[0].token_nonce,
                actual_transfers[0].token_nonce
            );
            assert_eq!(expected_transfers[0].amount, actual_transfers[0].amount);

            assert_eq!(
                expected_transfers[1].token_identifier,
                actual_transfers[1].token_identifier
            );
            assert_eq!(
                expected_transfers[1].token_nonce,
                actual_transfers[1].token_nonce
            );
            assert_eq!(expected_transfers[1].amount, actual_transfers[1].amount);
        })
        .assert_ok();

    wrapper.check_esdt_balance(sc_wrapper.address_ref(), token_id_1, &rust_biguint!(100));
    wrapper.check_nft_balance::<Empty>(
        sc_wrapper.address_ref(),
        token_id_2,
        nft_nonce,
        &rust_biguint!(1),
        None,
    );
}

#[test]
fn test_query() {
    let mut wrapper = BlockchainStateWrapper::new();
    let sc_wrapper = wrapper.create_sc_account(
        &rust_biguint!(2_000),
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );

    let _ = wrapper.execute_query(&sc_wrapper, |sc| {
        let actual_balance = sc.get_egld_balance();
        let expected_balance = managed_biguint!(2_000);
        assert_eq!(actual_balance, expected_balance);
    });
}

#[test]
fn storage_check_test() {
    let rust_zero = rust_biguint!(0);
    let mut wrapper = BlockchainStateWrapper::new();
    let user_addr = wrapper.create_user_account(&rust_zero);
    let sc_wrapper = wrapper.create_sc_account(
        &rust_zero,
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );

    // simulate deploy
    wrapper
        .execute_tx(&user_addr, &sc_wrapper, &rust_zero, |sc| {
            sc.init();
        })
        .assert_ok();

    wrapper
        .execute_tx(&user_addr, &sc_wrapper, &rust_zero, |sc| {
            let total_before = sc.total_value().get();
            let per_caller_before = sc.value_per_caller(&managed_address!(&user_addr)).get();

            assert_eq!(total_before, managed_biguint!(1));
            assert_eq!(per_caller_before, managed_biguint!(0));

            let added_value = managed_biguint!(50);
            sc.add(added_value.clone());

            let expected_total_after = total_before + added_value.clone();
            let expected_per_caller_after = per_caller_before + added_value;

            let actual_total_after = sc.total_value().get();
            let actual_per_caller_after = sc.value_per_caller(&managed_address!(&user_addr)).get();

            assert_eq!(expected_total_after, actual_total_after);
            assert_eq!(expected_per_caller_after, actual_per_caller_after);
        })
        .assert_ok();

    wrapper
        .execute_query(&sc_wrapper, |sc| {
            let expected_total = managed_biguint!(51);
            let expected_per_caller = managed_biguint!(50);

            let actual_total = sc.total_value().get();
            let actual_per_caller = sc.value_per_caller(&managed_address!(&user_addr)).get();

            assert_eq!(expected_total, actual_total);
            assert_eq!(expected_per_caller, actual_per_caller);
        })
        .assert_ok();
}

#[test]
fn storage_revert_test() {
    let rust_zero = rust_biguint!(0);
    let mut wrapper = BlockchainStateWrapper::new();
    let user_addr = wrapper.create_user_account(&rust_zero);
    let sc_wrapper = wrapper.create_sc_account(
        &rust_zero,
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );

    // simulate deploy
    wrapper
        .execute_tx(&user_addr, &sc_wrapper, &rust_zero, |sc| {
            sc.init();
        })
        .assert_ok();

    wrapper
        .execute_tx(&user_addr, &sc_wrapper, &rust_zero, |sc| {
            let total_before = sc.total_value().get();
            let per_caller_before = sc.value_per_caller(&managed_address!(&user_addr)).get();

            assert_eq!(total_before, managed_biguint!(1));
            assert_eq!(per_caller_before, managed_biguint!(0));

            let added_value = managed_biguint!(50);
            sc.add(added_value.clone());

            let expected_total_after = total_before + added_value.clone();
            let expected_per_caller_after = per_caller_before + added_value;

            let actual_total_after = sc.total_value().get();
            let actual_per_caller_after = sc.value_per_caller(&managed_address!(&user_addr)).get();

            assert_eq!(expected_total_after, actual_total_after);
            assert_eq!(expected_per_caller_after, actual_per_caller_after);

            sc.panic();
        })
        .assert_user_error("Oh no!");

    wrapper
        .execute_query(&sc_wrapper, |sc| {
            let expected_total = managed_biguint!(1);
            let expected_per_caller = managed_biguint!(0);

            let actual_total = sc.total_value().get();
            let actual_per_caller = sc.value_per_caller(&managed_address!(&user_addr)).get();

            assert_eq!(expected_total, actual_total);
            assert_eq!(expected_per_caller, actual_per_caller);
        })
        .assert_ok();
}

#[test]
fn storage_set_test() {
    let rust_zero = rust_biguint!(0);
    let mut wrapper = BlockchainStateWrapper::new();
    let user_addr = wrapper.create_user_account(&rust_zero);
    let sc_wrapper = wrapper.create_sc_account(
        &rust_zero,
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );

    // simulate deploy
    wrapper
        .execute_tx(&user_addr, &sc_wrapper, &rust_zero, |sc| {
            sc.init();
        })
        .assert_ok();

    wrapper
        .execute_tx(&user_addr, &sc_wrapper, &rust_zero, |sc| {
            sc.total_value().set(&managed_biguint!(50));
            sc.value_per_caller(&managed_address!(&user_addr))
                .set(&managed_biguint!(50));
        })
        .assert_ok();

    wrapper
        .execute_query(&sc_wrapper, |sc| {
            let expected_value = managed_biguint!(50);

            let actual_total = sc.total_value().get();
            let actual_per_caller = sc.value_per_caller(&managed_address!(&user_addr)).get();

            assert_eq!(expected_value, actual_total);
            assert_eq!(expected_value, actual_per_caller);
        })
        .assert_ok();
}

#[test]
fn blockchain_state_test() {
    let rust_zero = rust_biguint!(0);
    let mut wrapper = BlockchainStateWrapper::new();
    let sc_wrapper = wrapper.create_sc_account(
        &rust_zero,
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );

    let expected_epoch = 10;
    let expected_nonce = 20;
    let expected_timestamp = 30;

    wrapper.set_block_epoch(expected_epoch);
    wrapper.set_block_nonce(expected_nonce);
    wrapper.set_block_timestamp(expected_timestamp);

    wrapper
        .execute_query(&sc_wrapper, |sc| {
            let actual_epoch = sc.get_block_epoch();
            let actual_nonce = sc.get_block_nonce();
            let actual_timestamp = sc.get_block_timestamp();

            assert_eq!(expected_epoch, actual_epoch);
            assert_eq!(expected_nonce, actual_nonce);
            assert_eq!(expected_timestamp, actual_timestamp);
        })
        .assert_ok();
}

#[test]
fn execute_on_dest_context_query_test() {
    let rust_zero = rust_biguint!(0);
    let mut wrapper = BlockchainStateWrapper::new();
    let user_addr = wrapper.create_user_account(&rust_zero);
    let sc_wrapper = wrapper.create_sc_account(
        &rust_zero,
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );
    let other_sc_wrapper = wrapper.create_sc_account(
        &rust_zero,
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );

    wrapper
        .execute_tx(&user_addr, &other_sc_wrapper, &rust_zero, |sc| {
            sc.total_value().set(&managed_biguint!(5));
        })
        .assert_ok();

    wrapper
        .execute_query(&sc_wrapper, |sc| {
            let expected_result = managed_biguint!(5);
            let actual_result =
                sc.call_other_contract_execute_on_dest(managed_address!(&other_sc_wrapper
                    .address_ref()
                    .clone()));

            assert_eq!(expected_result, actual_result);
        })
        .assert_ok();
}

#[test]
fn execute_on_dest_context_change_state_test() {
    let rust_zero = rust_biguint!(0);
    let mut wrapper = BlockchainStateWrapper::new();
    let user_addr = wrapper.create_user_account(&rust_zero);
    let sc_wrapper = wrapper.create_sc_account(
        &rust_zero,
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );
    let other_sc_wrapper = wrapper.create_sc_account(
        &rust_zero,
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );

    wrapper
        .execute_tx(&user_addr, &other_sc_wrapper, &rust_zero, |sc| {
            sc.total_value().set(&managed_biguint!(5));
        })
        .assert_ok();

    wrapper
        .execute_tx(&user_addr, &sc_wrapper, &rust_zero, |sc| {
            sc.execute_on_dest_add_value(
                managed_address!(&other_sc_wrapper.address_ref().clone()),
                managed_biguint!(5),
            );
        })
        .assert_ok();

    wrapper
        .execute_query(&other_sc_wrapper, |sc| {
            let expected_result = managed_biguint!(10);
            let actual_result = sc.get_val();

            assert_eq!(expected_result, actual_result);
        })
        .assert_ok();
}

#[test]
fn test_mandos_generation() {
    let rust_zero = rust_biguint!(0);
    let mut wrapper = BlockchainStateWrapper::new();
    let user_addr = wrapper.create_user_account(&rust_zero);
    let sc_wrapper = wrapper.create_sc_account(
        &rust_zero,
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );

    // simulate deploy
    wrapper
        .execute_tx(&user_addr, &sc_wrapper, &rust_zero, |sc| {
            sc.init();
        })
        .assert_ok();
    wrapper.add_mandos_set_account(sc_wrapper.address_ref());
    wrapper.add_mandos_check_account(sc_wrapper.address_ref());

    let add_value = rust_biguint!(50);
    let mut sc_call_mandos = ScCallMandos::new(&user_addr, sc_wrapper.address_ref(), "addValue");
    sc_call_mandos.add_argument(&add_value.to_bytes_be());
    sc_call_mandos.set_gas_limit(100_000_000);

    let tx_expect = TxExpectMandos::new(0);
    wrapper.add_mandos_sc_call(sc_call_mandos, Some(tx_expect));

    wrapper
        .execute_tx(&user_addr, &sc_wrapper, &rust_zero, |sc| {
            let total_before = sc.total_value().get();
            let per_caller_before = sc.value_per_caller(&managed_address!(&user_addr)).get();

            assert_eq!(total_before, managed_biguint!(1));
            assert_eq!(per_caller_before, managed_biguint!(0));

            let added_value = managed_biguint!(50);
            sc.add(added_value.clone());

            let expected_total_after = total_before + added_value.clone();
            let expected_per_caller_after = per_caller_before + added_value;

            let actual_total_after = sc.total_value().get();
            let actual_per_caller_after = sc.value_per_caller(&managed_address!(&user_addr)).get();

            assert_eq!(expected_total_after, actual_total_after);
            assert_eq!(expected_per_caller_after, actual_per_caller_after);
        })
        .assert_ok();
    wrapper.add_mandos_check_account(sc_wrapper.address_ref());

    let expected_value = rust_biguint!(51);
    let sc_query_mandos = ScQueryMandos::new(sc_wrapper.address_ref(), "getTotalValue");

    let mut query_expect = TxExpectMandos::new(0);
    query_expect.add_out_value(&expected_value.to_bytes_be());

    wrapper.add_mandos_sc_query(sc_query_mandos, Some(query_expect));

    wrapper
        .execute_query(&sc_wrapper, |sc| {
            let expected_total = managed_biguint!(51);
            let expected_per_caller = managed_biguint!(50);

            let actual_total = sc.total_value().get();
            let actual_per_caller = sc.value_per_caller(&managed_address!(&user_addr)).get();

            assert_eq!(expected_total, actual_total);
            assert_eq!(expected_per_caller, actual_per_caller);
        })
        .assert_ok();

    wrapper.write_mandos_output(TEST_OUTPUT_PATH);
}

#[test]
fn test_multiple_contracts() {
    let mut wrapper = BlockchainStateWrapper::new();
    let _sc_wrapper = wrapper.create_sc_account(
        &rust_biguint!(0u64),
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );

    let _sc_wrapper_other = wrapper.create_sc_account(
        &rust_biguint!(0u64),
        None,
        adder::contract_obj,
        ADDER_WASM_PATH,
    );

    wrapper.write_mandos_output(TEST_MULTIPLE_SC_OUTPUT_PATH);
}

#[test]
fn test_async_call() {
    let rust_zero = rust_biguint!(0);
    let mut wrapper = BlockchainStateWrapper::new();
    let user_addr = wrapper.create_user_account(&rust_zero);
    let sc_wrapper = wrapper.create_sc_account(
        &rust_zero,
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );
    let adder_wrapper =
        wrapper.create_sc_account(&rust_zero, None, adder::contract_obj, ADDER_WASM_PATH);

    let tx_result = wrapper.execute_tx(&user_addr, &sc_wrapper, &rust_zero, |sc| {
        let adder_address = managed_address!(adder_wrapper.address_ref());
        let value_to_add = managed_biguint!(10);
        sc.call_other_contract_add_async_call(adder_address, value_to_add);
    });
    tx_result.assert_ok();

    wrapper
        .execute_query(&sc_wrapper, |sc| {
            let callback_executed = sc.callback_executed().get();
            assert!(callback_executed);
        })
        .assert_ok();

    wrapper
        .execute_query(&adder_wrapper, |sc| {
            let current_sum = sc.sum().get();
            let expected_sum = BigUint::from(10u32);
            assert_eq!(current_sum, expected_sum);
        })
        .assert_ok();
}

#[test]
fn test_wrapper_getters() {
    let mut wrapper = BlockchainStateWrapper::new();
    let egld_balance = rust_biguint!(1_000);

    let esdt_token_id = b"ESDT-123456";
    let esdt_balance = rust_biguint!(100);

    let nft_token_id = b"NFT-123456";
    let nft_nonce = 5;
    let nft_balance = rust_biguint!(10);
    let nft_attributes = NftDummyAttributes {
        creation_epoch: 2,
        cool_factor: 100,
    };

    let user_addr = wrapper.create_user_account(&egld_balance);
    wrapper.set_esdt_balance(&user_addr, esdt_token_id, &esdt_balance);
    wrapper.set_nft_balance(
        &user_addr,
        nft_token_id,
        nft_nonce,
        &nft_balance,
        &nft_attributes,
    );

    let actual_egld_balance = wrapper.get_egld_balance(&user_addr);
    let actual_esdt_balance = wrapper.get_esdt_balance(&user_addr, esdt_token_id, 0);
    let actual_nft_balance = wrapper.get_esdt_balance(&user_addr, nft_token_id, nft_nonce);
    let actual_attributes = wrapper
        .get_nft_attributes::<NftDummyAttributes>(&user_addr, nft_token_id, nft_nonce)
        .unwrap();

    assert_eq!(egld_balance, actual_egld_balance);
    assert_eq!(esdt_balance, actual_esdt_balance);
    assert_eq!(nft_balance, actual_nft_balance);
    assert_eq!(nft_attributes, actual_attributes);
}

#[test]
fn fixed_address_account_creation_test() {
    let mut wrapper = BlockchainStateWrapper::new();
    wrapper
        .create_user_account_fixed_address(&Address::from_slice(&[1u8; 32][..]), &rust_biguint!(0));
}

#[should_panic(
    expected = "Invalid SC Address: \"0202020202020202020202020202020202020202020202020202020202020202\""
)]
#[test]
fn fixed_address_invalid_sc_test() {
    let mut wrapper = BlockchainStateWrapper::new();
    let user_addr = Address::from_slice(&[1u8; 32][..]);

    wrapper.create_user_account_fixed_address(&user_addr, &rust_biguint!(0));
    wrapper.create_sc_account_fixed_address(
        &Address::from_slice(&[2u8; 32][..]),
        &rust_biguint!(0),
        Some(&user_addr),
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );
}

#[test]
fn managed_environment_test() {
    let wrapper = BlockchainStateWrapper::new();
    wrapper.execute_in_managed_environment(|| {
        let _my_struct = StructWithManagedTypes::<DebugApi> {
            big_uint: managed_biguint!(500),
            buffer: managed_buffer!(b"MyBuffer"),
        };
    });
}

#[test]
fn managed_environment_consistency_test() {
    let mut wrapper = BlockchainStateWrapper::new();
    let adder_wrapper = wrapper.create_sc_account(
        &rust_biguint!(0),
        None,
        adder::contract_obj,
        ADDER_WASM_PATH,
    );

    let first_var = wrapper.execute_in_managed_environment(|| BigUint::<DebugApi>::from(1u32));
    wrapper
        .execute_query(&adder_wrapper, |_sc| {
            let second_var = BigUint::from(2u32);
            let third_var = BigUint::from(3u32);
            let sum = first_var + second_var;
            assert_eq!(sum, third_var);
        })
        .assert_error(
            err_msg::DEBUG_API_ERR_STATUS,
            err_msg::DEBUG_API_ERR_HANDLE_CONTEXT_MISMATCH,
        );
}

#[test]
fn test_managed_values_standalone_consistency() {
    DebugApi::dummy();

    let mut blockchain_wrapper = BlockchainStateWrapper::new();
    let owner_address = blockchain_wrapper.create_user_account(&rust_biguint!(0u64));
    let basic_features_wrapper = blockchain_wrapper.create_sc_account(
        &rust_biguint!(0u64),
        Some(&owner_address),
        basic_features::contract_obj,
        BASIC_FEATURES_WASM_PATH,
    );

    let foo_token = TokenIdentifier::<DebugApi>::from_esdt_bytes(b"FOO-a1a1a1");
    blockchain_wrapper
        .execute_query(&basic_features_wrapper, |_sc| {
            let _bar = TokenIdentifier::<DebugApi>::from_esdt_bytes(b"BAR-a1a1a1");
            // 'foo' and '_bar' have the same numerical handle value
            // check that the value of 'foo' is taken from the correct context
            assert_eq!(
                foo_token,
                TokenIdentifier::<DebugApi>::from_esdt_bytes(b"FOO-a1a1a1")
            );
        })
        .assert_error(
            err_msg::DEBUG_API_ERR_STATUS,
            err_msg::DEBUG_API_ERR_HANDLE_CONTEXT_MISMATCH,
        );
}

#[test]
fn test_managed_values_argument_and_return_value_consistency() {
    DebugApi::dummy();

    let mut blockchain_wrapper = BlockchainStateWrapper::new();
    let owner_address = blockchain_wrapper.create_user_account(&rust_biguint!(0u64));
    let basic_features_wrapper = blockchain_wrapper.create_sc_account(
        &rust_biguint!(0u64),
        Some(&owner_address),
        basic_features::contract_obj,
        BASIC_FEATURES_WASM_PATH,
    );

    let argument = managed_biguint!(42u64);
    let mut result = managed_biguint!(0u64);

    blockchain_wrapper
        .execute_tx(
            &owner_address,
            &basic_features_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                let dummy: BigUint<DebugApi> = managed_biguint!(100u64);
                assert_eq!(dummy.to_u64().unwrap(), 100);

                // 'argument' was created in the top-level context
                assert_eq!(argument.to_u64().unwrap(), 42);
                result = sc.endpoint_with_mutable_arg(argument, 3, 4);
                assert_eq!(result.to_u64().unwrap(), 49);
            },
        )
        .assert_error(
            err_msg::DEBUG_API_ERR_STATUS,
            err_msg::DEBUG_API_ERR_HANDLE_CONTEXT_MISMATCH,
        );
}

#[test]
fn test_managed_values_insert_handle_panics() {
    DebugApi::dummy();

    let mut blockchain_wrapper = BlockchainStateWrapper::new();
    let owner_address = blockchain_wrapper.create_user_account(&rust_biguint!(0u64));
    let basic_features_wrapper = blockchain_wrapper.create_sc_account(
        &rust_biguint!(0u64),
        Some(&owner_address),
        basic_features::contract_obj,
        BASIC_FEATURES_WASM_PATH,
    );

    let item = managed_biguint!(42);

    blockchain_wrapper
        .execute_tx(
            &owner_address,
            &basic_features_wrapper,
            &rust_biguint!(0u64),
            |_sc| {
                let mut vec: ManagedVec<DebugApi, BigUint<DebugApi>> = ManagedVec::new();
                // this should panic because we're pushing the handle's value, which discards the context
                vec.push(item);
            },
        )
        .assert_user_error("panic occurred");
}

#[should_panic]
#[test]
fn test_managed_types_without_environment() {
    let _my_struct = StructWithManagedTypes::<DebugApi> {
        big_uint: managed_biguint!(500),
        buffer: managed_buffer!(b"MyBuffer"),
    };
}

#[test]
fn test_random_buffer() {
    let mut wrapper = BlockchainStateWrapper::new();
    let sc_wrapper = wrapper.create_sc_account(
        &rust_biguint!(0),
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );

    wrapper
        .execute_query(&sc_wrapper, |sc| {
            let rand_buffer = sc.get_random_buffer_once(2);
            let expected_buffer = managed_buffer!(&[0x8b, 0xdd]);
            assert_eq!(rand_buffer, expected_buffer);
        })
        .assert_ok();
}

#[test]
fn test_random_buffer_twice() {
    let mut wrapper = BlockchainStateWrapper::new();
    let sc_wrapper = wrapper.create_sc_account(
        &rust_biguint!(0),
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );

    wrapper
        .execute_query(&sc_wrapper, |sc| {
            let (rand_buffer_1, rand_buffer_2) = sc.get_random_buffer_twice(2, 2);

            let expected_buffer_1 = managed_buffer!(&[0x8b, 0xdd]);
            let expected_buffer_2 = managed_buffer!(&[0xbe, 0x24]);

            assert_eq!(rand_buffer_1, expected_buffer_1);
            assert_eq!(rand_buffer_2, expected_buffer_2);
        })
        .assert_ok();
}

#[test]
fn test_modules() {
    let mut wrapper = BlockchainStateWrapper::new();
    let sc_wrapper = wrapper.create_sc_account(
        &rust_biguint!(0),
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );

    wrapper
        .execute_query(&sc_wrapper, |sc| {
            let _ = sc.some_function();
        })
        .assert_ok();
}

#[test]
fn test_back_and_forth_transfers() {
    let mut wrapper = BlockchainStateWrapper::new();
    let user = wrapper.create_user_account(&rust_biguint!(0));
    let first_token_id = b"FIRSTTOKEN-abcdef";
    let second_token_id = b"SECTOKEN-abcdef";
    let third_token_id = b"THIRDTOKEN-abcdef";

    let first_token_amount = rust_biguint!(1_000_000);
    let second_token_amount = rust_biguint!(2_000_000);
    let third_token_amount = rust_biguint!(5_000_000);

    wrapper.set_esdt_balance(&user, &first_token_id[..], &first_token_amount);
    wrapper.set_esdt_balance(&user, &second_token_id[..], &second_token_amount);

    let forwarder_wrapper = wrapper.create_sc_account(
        &rust_biguint!(0),
        None,
        forwarder::contract_obj,
        "../forwarder/output/forwarder.wasm",
    );

    let vault_wrapper = wrapper.create_sc_account(
        &rust_biguint!(0),
        None,
        vault::contract_obj,
        "../vault/output/vault.wasm",
    );
    wrapper.set_esdt_balance(
        vault_wrapper.address_ref(),
        &third_token_id[..],
        &third_token_amount,
    );

    let transfers = vec![
        TxTokenTransfer {
            token_identifier: first_token_id.to_vec(),
            nonce: 0,
            value: first_token_amount.clone(),
        },
        TxTokenTransfer {
            token_identifier: second_token_id.to_vec(),
            nonce: 0,
            value: second_token_amount.clone(),
        },
    ];

    wrapper
        .execute_esdt_multi_transfer(&user, &forwarder_wrapper, &transfers, |sc| {
            sc.forward_sync_retrieve_funds_with_accept_func(
                managed_address!(vault_wrapper.address_ref()),
                managed_token_id!(&third_token_id[..]),
                managed_biguint!(third_token_amount.to_u64().unwrap()),
            );
        })
        .assert_ok();

    wrapper.check_esdt_balance(
        forwarder_wrapper.address_ref(),
        &third_token_id[..],
        &third_token_amount,
    );
}

#[test]
fn dump_state_single_test() {
    let mut wrapper = BlockchainStateWrapper::new();
    let sc_wrapper = wrapper.create_sc_account(
        &rust_biguint!(0),
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );
    let fungible_token_id = &b"COOL-123456"[..];
    let semi_fungible_token_id = &b"NOTCOOL-123456"[..];
    let sft_attributes_first = NftDummyAttributes {
        creation_epoch: 2,
        cool_factor: 100,
    };
    let sft_attributes_second = NftDummyAttributes {
        creation_epoch: 5,
        cool_factor: 255,
    };

    wrapper.set_egld_balance(sc_wrapper.address_ref(), &rust_biguint!(444));
    wrapper.set_esdt_balance(
        sc_wrapper.address_ref(),
        fungible_token_id,
        &rust_biguint!(1_000),
    );
    wrapper.set_nft_balance(
        sc_wrapper.address_ref(),
        semi_fungible_token_id,
        5,
        &rust_biguint!(200),
        &sft_attributes_first,
    );
    wrapper.set_nft_balance(
        sc_wrapper.address_ref(),
        semi_fungible_token_id,
        10,
        &rust_biguint!(300),
        &sft_attributes_second,
    );

    wrapper.dump_state_for_account::<NftDummyAttributes>(sc_wrapper.address_ref());
}

#[test]
fn dump_state_raw_attributes_test() {
    let mut wrapper = BlockchainStateWrapper::new();
    let sc_wrapper = wrapper.create_sc_account(
        &rust_biguint!(0),
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );
    let fungible_token_id = &b"COOL-123456"[..];
    let semi_fungible_token_id = &b"NOTCOOL-123456"[..];
    let sft_attributes_first = NftDummyAttributes {
        creation_epoch: 2,
        cool_factor: 100,
    };
    let sft_attributes_second = NftDummyAttributes {
        creation_epoch: 5,
        cool_factor: 255,
    };

    wrapper.set_egld_balance(sc_wrapper.address_ref(), &rust_biguint!(444));
    wrapper.set_esdt_balance(
        sc_wrapper.address_ref(),
        fungible_token_id,
        &rust_biguint!(1_000),
    );
    wrapper.set_nft_balance(
        sc_wrapper.address_ref(),
        semi_fungible_token_id,
        5,
        &rust_biguint!(200),
        &sft_attributes_first,
    );
    wrapper.set_nft_balance(
        sc_wrapper.address_ref(),
        semi_fungible_token_id,
        10,
        &rust_biguint!(300),
        &sft_attributes_second,
    );

    wrapper.dump_state_for_account_hex_attributes(sc_wrapper.address_ref());
}

#[test]
fn dump_state_all_test() {
    let mut wrapper = BlockchainStateWrapper::new();
    let user_addr = wrapper.create_user_account(&rust_biguint!(333));
    let sc_wrapper = wrapper.create_sc_account(
        &rust_biguint!(0),
        None,
        rust_testing_framework_tester::contract_obj,
        SC_WASM_PATH,
    );
    let fungible_token_id = &b"COOL-123456"[..];
    let semi_fungible_token_id = &b"NOTCOOL-123456"[..];
    let sft_attributes_first = NftDummyAttributes {
        creation_epoch: 2,
        cool_factor: 100,
    };
    let sft_attributes_second = NftDummyAttributes {
        creation_epoch: 5,
        cool_factor: 255,
    };

    wrapper.set_egld_balance(sc_wrapper.address_ref(), &rust_biguint!(444));
    wrapper.set_esdt_balance(
        sc_wrapper.address_ref(),
        fungible_token_id,
        &rust_biguint!(1_000),
    );
    wrapper.set_nft_balance(
        sc_wrapper.address_ref(),
        semi_fungible_token_id,
        5,
        &rust_biguint!(200),
        &sft_attributes_first,
    );
    wrapper.set_nft_balance(
        sc_wrapper.address_ref(),
        semi_fungible_token_id,
        10,
        &rust_biguint!(300),
        &sft_attributes_second,
    );

    wrapper
        .execute_tx(&user_addr, &sc_wrapper, &rust_biguint!(0), |sc| {
            sc.total_value().set(&managed_biguint!(1_000_000));
            sc.value_per_caller(&managed_address!(&user_addr))
                .set(&managed_biguint!(2_000_000));
            sc.callback_executed().set(true);
        })
        .assert_ok();

    wrapper.dump_state();
}

fn uris_to_vec(uris: &[&[u8]]) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    for uri in uris {
        out.push((*uri).to_vec());
    }

    out
}
