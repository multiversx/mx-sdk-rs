use elrond_wasm::{
    contract_base::ContractBase,
    types::{Address, BigUint, ManagedFrom, SCResult, H256},
};
use elrond_wasm_debug::{assert_sc_error, managed_biguint, rust_biguint, testing_framework::*};
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
fn test_sc_payment() {
    let mut wrapper = ContractObjWrapper::new(rust_testing_framework_tester::contract_obj);

    let caller_addr = wrapper.create_user_account(&rust_biguint!(1_000));
    let sc_addr = wrapper.create_sc_account(&rust_biguint!(2_000), Some(&caller_addr));

    wrapper = wrapper.execute_tx(&caller_addr, &sc_addr, &rust_biguint!(1_000), |sc| {
        let actual_payment = sc.receive_egld();
        let expected_payment = managed_biguint!(sc, 1_000);
        assert_eq!(actual_payment, expected_payment);
    });

    wrapper.check_egkd_balance(&caller_addr, &rust_biguint!(0));
    wrapper.check_egkd_balance(&sc_addr, &rust_biguint!(3_000));
}

#[test]
fn test_sc_half_payment() {
    let mut wrapper = ContractObjWrapper::new(rust_testing_framework_tester::contract_obj);

    let caller_addr = wrapper.create_user_account(&rust_biguint!(1_000));
    let sc_addr = wrapper.create_sc_account(&rust_biguint!(2_000), Some(&caller_addr));

    wrapper = wrapper.execute_tx(&caller_addr, &sc_addr, &rust_biguint!(1_000), |sc| {
        sc.recieve_egld_half();
    });

    wrapper.check_egkd_balance(&caller_addr, &rust_biguint!(500));
    wrapper.check_egkd_balance(&sc_addr, &rust_biguint!(2_500));
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

/*
Update cache before call?

tx_context.tx_cache.subtract_egld_balance(
        &tx_context.tx_input_box.from,
        &tx_context.tx_input_box.egld_value,
    );
    tx_context.tx_cache.increase_egld_balance(
        &tx_context.tx_input_box.to,
        &tx_context.tx_input_box.egld_value,
    );

    // TODO: temporary, will convert to explicit builtin function first
    for esdt_transfer in tx_context.tx_input_box.esdt_values.iter() {
        tx_context.tx_cache.transfer_esdt_balance(
            &tx_context.tx_input_box.from,
            &tx_context.tx_input_box.to,
            &esdt_transfer.token_identifier,
            esdt_transfer.nonce,
            &esdt_transfer.value,
        );
    }

    let tx_result = if !is_smart_contract_address(&tx_context.tx_input_box.to)
        || tx_context.tx_input_box.func_name.is_empty()
    {
        // direct EGLD transfer
        TxResult::empty()
    } else {
        execute_tx_context(tx_context.clone())
    };

    let blockchain_updates = tx_context.into_blockchain_updates();

    (tx_result, blockchain_updates)
*/

// fn type_test<A: VMApi>(_sc: ContractObj<A>) {}

/*
fn execute_test_tx<F: FnOnce(DebugApi)>(
    tx_input: TxInput,
    world: BlockchainMock,
    f: F,
) -> BlockchainMock {
    let rc_world = Rc::new(world);
    let api = DebugApi::new(tx_input, TxCache::new(rc_world));

    f(api);
    let bu = api.into_blockchain_updates();

    let mut world = Rc::try_unwrap(rc_world).unwrap();
    bu.apply(&mut world);
    world
}
*/
