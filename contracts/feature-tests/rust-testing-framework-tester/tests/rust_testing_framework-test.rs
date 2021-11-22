use elrond_wasm::{
    contract_base::ContractBase,
    types::{BigUint, ManagedAddress, ManagedFrom, SCResult, TokenIdentifier},
};
use elrond_wasm_debug::{
    assert_sc_error, managed_address, managed_biguint, managed_token_id, rust_biguint,
    testing_framework::*,
};
use rust_testing_framework_tester::*;

#[test]
fn test_add() {
    let mut wrapper = ContractObjWrapper::new(rust_testing_framework_tester::contract_obj);
    let sc_addr = wrapper.create_sc_account(&rust_biguint!(0), None);

    wrapper.execute_query(&sc_addr, |sc| {
        let first = managed_biguint!(sc, 1000);
        let second = managed_biguint!(sc, 2000);

        let expected_result = first.clone() + second.clone();
        let actual_result = sc.sum(first, second);
        assert_eq!(expected_result, actual_result);
    });
}

#[test]
fn test_sc_result_ok() {
    let mut wrapper = ContractObjWrapper::new(rust_testing_framework_tester::contract_obj);
    let sc_addr = wrapper.create_sc_account(&rust_biguint!(0), None);

    wrapper.execute_query(&sc_addr, |sc| {
        let first = managed_biguint!(sc, 1000);
        let second = managed_biguint!(sc, 2000);

        let expected_result = SCResult::Ok(first.clone() + second.clone());
        let actual_result = sc.sum_sc_result(first, second);
        assert_eq!(expected_result, actual_result);
    });
}

#[test]
fn test_sc_result_err() {
    let mut wrapper = ContractObjWrapper::new(rust_testing_framework_tester::contract_obj);
    let sc_addr = wrapper.create_sc_account(&rust_biguint!(0), None);

    wrapper.execute_query(&sc_addr, |sc| {
        let first = managed_biguint!(sc, 0);
        let second = managed_biguint!(sc, 2000);

        let actual_result = sc.sum_sc_result(first, second);
        assert_sc_error!(actual_result, b"Non-zero required");
    });
}

#[test]
fn test_sc_payment_ok() {
    let mut wrapper = ContractObjWrapper::new(rust_testing_framework_tester::contract_obj);

    let caller_addr = wrapper.create_user_account(&rust_biguint!(1_000));
    let sc_addr = wrapper.create_sc_account(&rust_biguint!(2_000), Some(&caller_addr));

    wrapper = wrapper.execute_tx(&caller_addr, &sc_addr, &rust_biguint!(1_000), |sc| {
        let actual_payment = sc.receive_egld();
        let expected_payment = managed_biguint!(sc, 1_000);
        assert_eq!(actual_payment, expected_payment);

        StateChange::Commit
    });

    wrapper.check_egld_balance(&caller_addr, &rust_biguint!(0));
    wrapper.check_egld_balance(&sc_addr, &rust_biguint!(3_000));
}

#[test]
fn test_sc_payment_reverted() {
    let mut wrapper = ContractObjWrapper::new(rust_testing_framework_tester::contract_obj);

    let caller_addr = wrapper.create_user_account(&rust_biguint!(1_000));
    let sc_addr = wrapper.create_sc_account(&rust_biguint!(2_000), Some(&caller_addr));

    wrapper = wrapper.execute_tx(&caller_addr, &sc_addr, &rust_biguint!(1_000), |sc| {
        let actual_payment = sc.receive_egld();
        let expected_payment = managed_biguint!(sc, 1_000);
        assert_eq!(actual_payment, expected_payment);

        StateChange::Revert
    });

    wrapper.check_egld_balance(&caller_addr, &rust_biguint!(1_000));
    wrapper.check_egld_balance(&sc_addr, &rust_biguint!(2_000));
}

#[test]
fn test_sc_half_payment() {
    let mut wrapper = ContractObjWrapper::new(rust_testing_framework_tester::contract_obj);

    let caller_addr = wrapper.create_user_account(&rust_biguint!(1_000));
    let sc_addr = wrapper.create_sc_account(&rust_biguint!(2_000), Some(&caller_addr));

    wrapper = wrapper.execute_tx(&caller_addr, &sc_addr, &rust_biguint!(1_000), |sc| {
        sc.recieve_egld_half();

        StateChange::Commit
    });

    wrapper.check_egld_balance(&caller_addr, &rust_biguint!(500));
    wrapper.check_egld_balance(&sc_addr, &rust_biguint!(2_500));
}

#[test]
fn test_esdt_balance() {
    let mut wrapper = ContractObjWrapper::new(rust_testing_framework_tester::contract_obj);
    let sc_addr = wrapper.create_sc_account(&rust_biguint!(0), None);
    let token_id = &b"COOL-123456"[..];

    wrapper.set_esdt_balance(&sc_addr, token_id, &rust_biguint!(1_000));
    wrapper.check_esdt_balance(&sc_addr, token_id, &rust_biguint!(1_000));

    wrapper.execute_query(&sc_addr, |sc| {
        let managed_id = managed_token_id!(sc, token_id);

        let actual_balance = sc.get_esdt_balance(managed_id, 0);
        let expected_balance = managed_biguint!(sc, 1_000);
        assert_eq!(expected_balance, actual_balance);
    });
}

#[test]
fn test_esdt_payment_ok() {
    let mut wrapper = ContractObjWrapper::new(rust_testing_framework_tester::contract_obj);
    let rust_zero = rust_biguint!(0);

    let caller_addr = wrapper.create_user_account(&rust_zero);
    let sc_addr = wrapper.create_sc_account(&rust_zero, None);
    let token_id = &b"COOL-123456"[..];

    wrapper.set_esdt_balance(&caller_addr, token_id, &rust_biguint!(1_000));
    wrapper.set_esdt_balance(&sc_addr, token_id, &rust_biguint!(2_000));

    wrapper = wrapper.execute_esdt_transfer(
        &caller_addr,
        &sc_addr,
        token_id,
        0,
        &rust_biguint!(1_000),
        |sc| {
            let (actual_token_id, actual_payment) = sc.receive_esdt();
            let expected_payment = managed_biguint!(sc, 1_000);

            assert_eq!(actual_token_id, managed_token_id!(sc, token_id));
            assert_eq!(actual_payment, expected_payment);

            StateChange::Commit
        },
    );

    wrapper.check_esdt_balance(&caller_addr, token_id, &rust_zero);
    wrapper.check_esdt_balance(&sc_addr, token_id, &rust_biguint!(3_000));
}

#[test]
fn test_esdt_payment_reverted() {
    let mut wrapper = ContractObjWrapper::new(rust_testing_framework_tester::contract_obj);
    let rust_zero = rust_biguint!(0);

    let caller_addr = wrapper.create_user_account(&rust_zero);
    let sc_addr = wrapper.create_sc_account(&rust_zero, None);
    let token_id = &b"COOL-123456"[..];

    wrapper.set_esdt_balance(&caller_addr, token_id, &rust_biguint!(1_000));
    wrapper.set_esdt_balance(&sc_addr, token_id, &rust_biguint!(2_000));

    wrapper = wrapper.execute_esdt_transfer(
        &caller_addr,
        &sc_addr,
        token_id,
        0,
        &rust_biguint!(1_000),
        |sc| {
            let (actual_token_id, actual_payment) = sc.receive_esdt();
            let expected_payment = managed_biguint!(sc, 1_000);

            assert_eq!(actual_token_id, managed_token_id!(sc, token_id));
            assert_eq!(actual_payment, expected_payment);

            StateChange::Revert
        },
    );

    wrapper.check_esdt_balance(&caller_addr, token_id, &rust_biguint!(1_000));
    wrapper.check_esdt_balance(&sc_addr, token_id, &rust_biguint!(2_000));
}

#[test]
fn test_nft_balance() {
    let mut wrapper = ContractObjWrapper::new(rust_testing_framework_tester::contract_obj);
    let sc_addr = wrapper.create_sc_account(&rust_biguint!(0), None);
    let token_id = &b"COOL-123456"[..];
    let nft_nonce = 2;
    let nft_balance = rust_biguint!(1_000);
    let nft_attributes = NftDummyAttributes {
        creation_epoch: 666,
        cool_factor: 101,
    };

    wrapper.set_nft_balance(&sc_addr, token_id, nft_nonce, &nft_balance, &nft_attributes);
    wrapper.check_nft_balance(&sc_addr, token_id, nft_nonce, &nft_balance, &nft_attributes);

    wrapper.execute_query(&sc_addr, |sc| {
        let managed_id = managed_token_id!(sc, token_id);

        let actual_balance = sc.get_esdt_balance(managed_id, nft_nonce);
        let expected_balance = managed_biguint!(sc, 1_000);
        assert_eq!(expected_balance, actual_balance);
    });
}

#[test]
fn test_sc_send_nft_to_user() {
    let mut wrapper = ContractObjWrapper::new(rust_testing_framework_tester::contract_obj);
    let caller_addr = wrapper.create_user_account(&rust_biguint!(0));
    let sc_addr = wrapper.create_sc_account(&rust_biguint!(0), None);
    let token_id = &b"COOL-123456"[..];
    let nft_nonce = 2;
    let nft_balance = rust_biguint!(1_000);
    let nft_attributes = NftDummyAttributes {
        creation_epoch: 666,
        cool_factor: 101,
    };

    wrapper.set_nft_balance(&sc_addr, token_id, nft_nonce, &nft_balance, &nft_attributes);
    wrapper.check_nft_balance(&sc_addr, token_id, nft_nonce, &nft_balance, &nft_attributes);

    wrapper = wrapper.execute_tx(&caller_addr, &sc_addr, &rust_biguint!(0), |sc| {
        let managed_addr = managed_address!(sc, &caller_addr);
        let managed_id = managed_token_id!(sc, token_id);
        let managed_amt = managed_biguint!(sc, 400);
        sc.send_nft(managed_addr, managed_id, nft_nonce, managed_amt);

        StateChange::Commit
    });

    wrapper.check_nft_balance(
        &caller_addr,
        token_id,
        nft_nonce,
        &rust_biguint!(400),
        &nft_attributes,
    );
    wrapper.check_nft_balance(
        &sc_addr,
        token_id,
        nft_nonce,
        &rust_biguint!(600),
        &nft_attributes,
    );
}

#[test]
fn test_sc_esdt_mint() {
    let mut wrapper = ContractObjWrapper::new(rust_testing_framework_tester::contract_obj);
    let caller_addr = wrapper.create_user_account(&rust_biguint!(0));
    let sc_addr = wrapper.create_sc_account(&rust_biguint!(0), None);
    let token_id = &b"COOL-123456"[..];

    // TODO
}

#[test]
fn test_query() {
    let mut wrapper = ContractObjWrapper::new(rust_testing_framework_tester::contract_obj);
    let sc_addr = wrapper.create_sc_account(&rust_biguint!(2_000), None);

    let _ = wrapper.execute_query(&sc_addr, |sc| {
        let actual_balance = sc.get_egld_balance();
        let expected_balance = managed_biguint!(sc, 2_000);
        assert_eq!(actual_balance, expected_balance);
    });
}

#[test]
fn storage_check_test() {
    let rust_zero = rust_biguint!(0);
    let mut wrapper = ContractObjWrapper::new(rust_testing_framework_tester::contract_obj);
    let user_addr = wrapper.create_user_account(&rust_zero);
    let sc_addr = wrapper.create_sc_account(&rust_zero, None);

    // simulate deploy
    wrapper = wrapper.execute_tx(&user_addr, &sc_addr, &rust_zero, |sc| {
        sc.init();

        StateChange::Commit
    });

    wrapper = wrapper.execute_tx(&user_addr, &sc_addr, &rust_zero, |sc| {
        let total_before = sc.total_value().get();
        let per_caller_before = sc.value_per_caller(&managed_address!(sc, &user_addr)).get();

        assert_eq!(total_before, managed_biguint!(sc, 1));
        assert_eq!(per_caller_before, managed_biguint!(sc, 0));

        let added_value = managed_biguint!(sc, 50);
        sc.add(added_value.clone());

        let expected_total_after = total_before + added_value.clone();
        let expected_per_caller_after = per_caller_before + added_value;

        let actual_total_after = sc.total_value().get();
        let actual_per_caller_after = sc.value_per_caller(&managed_address!(sc, &user_addr)).get();

        assert_eq!(expected_total_after, actual_total_after);
        assert_eq!(expected_per_caller_after, actual_per_caller_after);

        StateChange::Commit
    });

    wrapper.execute_query(&sc_addr, |sc| {
        let expected_total = managed_biguint!(sc, 51);
        let expected_per_caller = managed_biguint!(sc, 50);

        let actual_total = sc.total_value().get();
        let actual_per_caller = sc.value_per_caller(&managed_address!(sc, &user_addr)).get();

        assert_eq!(expected_total, actual_total);
        assert_eq!(expected_per_caller, actual_per_caller);
    });
}

#[test]
fn storage_revert_test() {
    let rust_zero = rust_biguint!(0);
    let mut wrapper = ContractObjWrapper::new(rust_testing_framework_tester::contract_obj);
    let user_addr = wrapper.create_user_account(&rust_zero);
    let sc_addr = wrapper.create_sc_account(&rust_zero, None);

    // simulate deploy
    wrapper = wrapper.execute_tx(&user_addr, &sc_addr, &rust_zero, |sc| {
        sc.init();

        StateChange::Commit
    });

    wrapper = wrapper.execute_tx(&user_addr, &sc_addr, &rust_zero, |sc| {
        let total_before = sc.total_value().get();
        let per_caller_before = sc.value_per_caller(&managed_address!(sc, &user_addr)).get();

        assert_eq!(total_before, managed_biguint!(sc, 1));
        assert_eq!(per_caller_before, managed_biguint!(sc, 0));

        let added_value = managed_biguint!(sc, 50);
        sc.add(added_value.clone());

        let expected_total_after = total_before + added_value.clone();
        let expected_per_caller_after = per_caller_before + added_value;

        let actual_total_after = sc.total_value().get();
        let actual_per_caller_after = sc.value_per_caller(&managed_address!(sc, &user_addr)).get();

        assert_eq!(expected_total_after, actual_total_after);
        assert_eq!(expected_per_caller_after, actual_per_caller_after);

        StateChange::Revert
    });

    wrapper.execute_query(&sc_addr, |sc| {
        let expected_total = managed_biguint!(sc, 1);
        let expected_per_caller = managed_biguint!(sc, 0);

        let actual_total = sc.total_value().get();
        let actual_per_caller = sc.value_per_caller(&managed_address!(sc, &user_addr)).get();

        assert_eq!(expected_total, actual_total);
        assert_eq!(expected_per_caller, actual_per_caller);
    });
}

#[test]
fn storage_set_test() {
    let rust_zero = rust_biguint!(0);
    let mut wrapper = ContractObjWrapper::new(rust_testing_framework_tester::contract_obj);
    let user_addr = wrapper.create_user_account(&rust_zero);
    let sc_addr = wrapper.create_sc_account(&rust_zero, None);

    // simulate deploy
    wrapper = wrapper.execute_tx(&user_addr, &sc_addr, &rust_zero, |sc| {
        sc.init();

        StateChange::Commit
    });

    wrapper = wrapper.execute_tx(&user_addr, &sc_addr, &rust_zero, |sc| {
        sc.total_value().set(&managed_biguint!(sc, 50));
        sc.value_per_caller(&managed_address!(sc, &user_addr))
            .set(&managed_biguint!(sc, 50));

        StateChange::Commit
    });

    wrapper.execute_query(&sc_addr, |sc| {
        let expected_value = managed_biguint!(sc, 50);

        let actual_total = sc.total_value().get();
        let actual_per_caller = sc.value_per_caller(&managed_address!(sc, &user_addr)).get();

        assert_eq!(expected_value, actual_total);
        assert_eq!(expected_value, actual_per_caller);
    });
}

#[test]
fn blockchain_state_test() {
    let rust_zero = rust_biguint!(0);
    let mut wrapper = ContractObjWrapper::new(rust_testing_framework_tester::contract_obj);
    let sc_addr = wrapper.create_sc_account(&rust_zero, None);

    let expected_epoch = 10;
    let expected_nonce = 20;
    let expected_timestamp = 30;

    wrapper.set_block_epoch(expected_epoch);
    wrapper.set_block_nonce(expected_nonce);
    wrapper.set_block_timestamp(expected_timestamp);

    wrapper.execute_query(&sc_addr, |sc| {
        let actual_epoch = sc.get_block_epoch();
        let actual_nonce = sc.get_block_nonce();
        let actual_timestamp = sc.get_block_timestamp();

        assert_eq!(expected_epoch, actual_epoch);
        assert_eq!(expected_nonce, actual_nonce);
        assert_eq!(expected_timestamp, actual_timestamp);
    });
}
