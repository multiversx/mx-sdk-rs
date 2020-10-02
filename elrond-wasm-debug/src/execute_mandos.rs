#![allow(unused_variables)] // for now

use super::*;
use num_bigint::BigUint;
use elrond_wasm::*;
use mandos_rs::*;
use std::path::Path;

pub fn parse_execute_mandos<P: AsRef<Path>>(path: P, contract_map: &ContractMap<TxContext>) {
    let scenario = mandos_rs::parse_scenario(path);
    execute_mandos_scenario(scenario, contract_map);
}

pub fn execute_mandos_scenario(scenario: Scenario, contract_map: &ContractMap<TxContext>) {
    let mut state = BlockchainMock::new();

    for step in scenario.steps.iter() {
        match step {
            Step::ExternalSteps {
                path,
            } => {},
            Step::SetState {
                comment,
                accounts,
                new_addresses,
                block_hashes,
                previous_block_info,
                current_block_info,
            } => {
                for (address, account) in accounts.iter() {
                    state.add_account(AccountData{
                        address: address.value.into(),
                        nonce: account.nonce.value,
                        balance: account.balance.value.clone(),
                        storage: account.storage.iter().map(|(k, v)| (k.value.clone(), v.value.clone())).collect(),
                        contract_path: account.code.as_ref().map(|bytes_value| bytes_value.value.clone()),
                        contract_owner: None, // TODO: add contract owner in mandos
                    });
                }
                for new_address in new_addresses.iter() {
                    state.put_new_address(
                        new_address.creator_address.value.into(),
                        new_address.creator_nonce.value,
                        new_address.new_address.value.into())
                }
            },
            Step::ScCall {
                tx_id,
                comment,
                tx,
                expect,
            } => {
                let tx_input = TxInput{
                    from: tx.from.value.into(),
                    to: tx.to.value.into(),
                    call_value: tx.call_value.value.clone(),
                    func_name: tx.function.as_bytes().to_vec(),
                    args: tx.arguments.iter().map(|scen_arg| scen_arg.value.clone()).collect(),
                };

                state.increase_nonce(&tx.from.value.into());
                state.subtract_tx_payment(&tx.from.value.into(),
                    &tx.call_value.value,
                    tx.gas_limit.value, tx.gas_price.value);
                
                let contract_account = state.accounts
                    .get_mut(&tx.to.value.into())
                    .unwrap_or_else(|| panic!("Recipient account not found"));

                let contract_path = &contract_account.contract_path.clone()
                    .unwrap_or_else(|| panic!("Recipient account is not a smart contract"));
                    
                let tx_context = TxContext::new(
                    tx_input,
                    TxOutput{
                        contract_storage: contract_account.storage.clone(),
                        result: TxResult::empty(),
                    });
                let tx_output = execute_tx(tx_context, contract_path, contract_map);
                let tx_result = tx_output.result;

                if tx_result.result_status == 0 {
                    // replace storage with new one
                    let _ = std::mem::replace(&mut contract_account.storage, tx_output.contract_storage);

                    state.increase_balance(&tx.to.value.into(), &tx.call_value.value);
                } else {
                    state.increase_balance(&tx.from.value.into(), &tx.call_value.value);
                }

                if let Some(tx_expect) = expect {
                    check_tx_output(tx_id.as_str(), &tx_expect, &tx_result);
                }
            },
            Step::ScDeploy {
                tx_id,
                comment,
                tx,
                expect,
            } => {
                let tx_input = TxInput{
                    from: tx.from.value.into(),
                    to: H256::zero(),
                    call_value: tx.call_value.value.clone(),
                    func_name: b"init".to_vec(),
                    args: tx.arguments.iter().map(|scen_arg| scen_arg.value.clone()).collect(),
                };

                state.increase_nonce(&tx.from.value.into());
                state.subtract_tx_payment(&tx.from.value.into(),
                    &tx.call_value.value,
                    tx.gas_limit.value, tx.gas_price.value);

                let tx_context = TxContext::new(
                    tx_input.clone(),
                    TxOutput{
                        contract_storage: HashMap::new(),
                        result: TxResult::empty(),
                    });
                let contract_path = &tx.contract_code.value;
                let tx_output = execute_tx(tx_context, contract_path, contract_map);

                if let Some(tx_expect) = expect {
                    check_tx_output(tx_id.as_str(), &tx_expect, &tx_output.result);
                }

                if tx_output.result.result_status == 0 {
                    state.create_account_after_deploy(
                        &tx_input,
                        tx_output,
                        tx.call_value.value.clone(),
                        contract_path.clone());
                } else {
                    state.increase_balance(&tx.from.value.into(), &tx.call_value.value);
                }
            },
            Step::Transfer {
                tx_id,
                comment,
                tx,
            } => {
                panic!("transfer step not yet supported");
            },
            Step::ValidatorReward {
                tx_id,
                comment,
                tx,
            } => {
                panic!("ValidatorReward step not yet supported");
            },
            Step::CheckState {
                comment,
                accounts,
            } => {
                for (expected_address, expected_account) in accounts.accounts.iter() {
                    let account = state.accounts.get(&expected_address.value.into())
                        .unwrap_or_else(|| panic!("Expected account not found"));

                    assert!(
                        expected_account.nonce.check(account.nonce),
                        "bad account nonce. Address: {}. Want: {}. Have: {}",
                        expected_address,
                        expected_account.nonce,
                        account.nonce);

                    assert!(
                        expected_account.balance.check(&account.balance),
                        "bad account balance. Address: {}. Want: {}. Have: {}",
                        expected_address,
                        expected_account.balance,
                        account.balance);
                    
                }
            },
            Step::DumpState {..} => {
                state.print_accounts();
            },
        }
    }
}

fn check_tx_output(tx_id: &str, tx_expect: &TxExpect, tx_result: &TxResult) {
    assert_eq!(tx_expect.out.len(), tx_result.result_values.len());
    for (i, expected_out) in tx_expect.out.iter().enumerate() {
        let actual_value = &tx_result.result_values[i];
        assert!(
            expected_out.check(actual_value.as_slice()),
            "bad out value. Tx id: {}. Want: {}. Have: {}",
            tx_id,
            expected_out,
            verbose_hex(actual_value.as_slice()));
    }

    if let Some(expected_message) = &tx_expect.message {
        let want_str = std::str::from_utf8(expected_message.value.as_slice()).unwrap();
        let have_str = std::str::from_utf8(tx_result.result_message.as_slice()).unwrap();
        assert_eq!(want_str, have_str,
            "bad error message. Tx id: {}. Want: \"{}\". Have: \"{}\"",
            tx_id, want_str, have_str);
    }
    
    assert_eq!(tx_expect.status.value, tx_result.result_status);
}
