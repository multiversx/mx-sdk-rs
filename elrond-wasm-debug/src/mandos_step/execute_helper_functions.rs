use elrond_wasm::types::H256;
use mandos::model::{CheckLogs, Checkable, TxExpect};
use std::collections::HashMap;

pub const ESDT_TRANSFER_STRING: &[u8] = b"ESDTTransfer";
pub const ESDT_NFT_TRANSFER_STRING: &[u8] = b"ESDTNFTTransfer";

use crate::{
    address_hex, async_call_tx_input, async_callback_tx_input, bytes_to_string, execute_tx,
    merge_results, try_execute_builtin_function, verbose_hex, world_mock::AccountEsdt, AccountData,
    AsyncCallTxData, BlockchainMock, BlockchainMockError, ContractMap, TxContext, TxInput,
    TxManagedTypes, TxOutput, TxResult,
};

pub fn generate_tx_hash_dummy(tx_id: &str) -> H256 {
    let bytes = tx_id.as_bytes();
    let mut result = [b'.'; 32];
    if bytes.len() > 32 {
        result[..].copy_from_slice(&bytes[..32]);
    } else {
        result[..bytes.len()].copy_from_slice(bytes);
    }
    result.into()
}

fn try_prepare_builtin_function(tx_input: &TxInput) -> Option<TxInput> {
    if tx_already_builtin_func(tx_input) {
        return Some(tx_input.clone());
    }

    if tx_is_esdt_transfer(tx_input) {
        return Some(to_esdt_transfer_builtin_repr(tx_input));
    }

    if tx_is_esdt_nft_transfer(tx_input) {
        return Some(to_esdt_nft_transfer_builtin_repr(tx_input));
    }

    None
}

fn to_esdt_transfer_builtin_repr(tx_input: &TxInput) -> TxInput {
    let mut builtin_tx = tx_input.clone();

    let orig_func = builtin_tx.func_name;
    builtin_tx.func_name = ESDT_TRANSFER_STRING.into();

    let args_to_prepend = [
        builtin_tx.esdt_token_identifier.clone(),
        builtin_tx.esdt_value.to_bytes_be(),
        orig_func,
    ];

    builtin_tx
        .args
        .splice(0..0, args_to_prepend.iter().cloned());

    builtin_tx
}

fn to_esdt_nft_transfer_builtin_repr(tx_input: &TxInput) -> TxInput {
    let mut builtin_tx = tx_input.clone();

    let orig_func = builtin_tx.func_name;
    builtin_tx.func_name = ESDT_NFT_TRANSFER_STRING.into();

    let args_to_prepend = [
        builtin_tx.esdt_token_identifier.clone(),
        builtin_tx.nonce.to_be_bytes().into(),
        builtin_tx.esdt_value.to_bytes_be(),
        builtin_tx.from.to_vec(),
        orig_func,
    ];

    builtin_tx
        .args
        .splice(0..0, args_to_prepend.iter().cloned());

    builtin_tx
}

fn tx_already_builtin_func(tx_input: &TxInput) -> bool {
    if compare(tx_input.func_name.as_slice(), ESDT_TRANSFER_STRING) == 0 {
        return true;
    }

    if compare(tx_input.func_name.as_slice(), ESDT_NFT_TRANSFER_STRING) == 0 {
        return true;
    }

    false
}

fn compare(a: &[u8], b: &[u8]) -> i64 {
    let mut ret = 0;
    for (p1, p2) in a.iter().zip(b.iter()) {
        if p1 != p2 {
            ret = *p1 as i64 - *p2 as i64;
            break;
        }
    }

    if ret == 0 {
        if a.len() < b.len() {
            ret = -1;
        } else if a.len() > b.len() {
            ret = 1;
        }
    }

    ret
}

fn tx_is_esdt_transfer(tx_input: &TxInput) -> bool {
    !tx_input.esdt_token_identifier.is_empty()
        && tx_input.esdt_value.count_ones() != 0
        && tx_input.nonce == 0
}

fn tx_is_esdt_nft_transfer(tx_input: &TxInput) -> bool {
    !tx_input.esdt_token_identifier.is_empty()
        && tx_input.esdt_value.count_ones() != 0
        && tx_input.nonce != 0
}

pub fn sc_call(
    tx_input: TxInput,
    state: &mut BlockchainMock,
    contract_map: &ContractMap<TxContext>,
) -> Result<(TxResult, Option<AsyncCallTxData>), BlockchainMockError> {
    let builtin_tx_input_opt = try_prepare_builtin_function(&tx_input);
    if let Some(builtin_tx_input) = builtin_tx_input_opt {
        let tx_result_opt = try_execute_builtin_function(&builtin_tx_input, state, contract_map);
        if let Some(tx_result) = tx_result_opt {
            return Ok((tx_result, None));
        }
    }

    let from = tx_input.from.clone();
    let to = tx_input.to.clone();
    let call_value = tx_input.call_value.clone();
    let blockchain_info = state.create_tx_info(&to);

    state.subtract_tx_payment(&from, &call_value)?;
    state.subtract_tx_gas(&from, tx_input.gas_limit, tx_input.gas_price);

    let esdt_token_identifier = tx_input.esdt_token_identifier.clone();
    let nonce = tx_input.nonce;
    let esdt_value = tx_input.esdt_value.clone();
    let esdt_used = !esdt_token_identifier.is_empty() && esdt_value > 0u32.into();

    if esdt_used {
        state.substract_esdt_balance(&from, &esdt_token_identifier, nonce, &esdt_value);
    }

    let contract_account = state
        .accounts
        .get_mut(&to)
        .unwrap_or_else(|| panic!("Recipient account not found: {}", address_hex(&to)));

    let contract_path = &contract_account
        .contract_path
        .clone()
        .unwrap_or_else(|| panic!("Recipient account is not a smart contract"));

    let tx_context = TxContext::new(
        blockchain_info,
        tx_input,
        TxOutput {
            contract_storage: contract_account.storage.clone(),
            managed_types: TxManagedTypes::new(),
            result: TxResult::empty(),
            send_balance_list: Vec::new(),
            async_call: None,
        },
    );

    let tx_output = execute_tx(tx_context, contract_path, contract_map);
    let mut tx_result = tx_output.result;

    if tx_result.result_status == 0 {
        // replace storage with new one
        let _ = std::mem::replace(&mut contract_account.storage, tx_output.contract_storage);

        state.increase_balance(&to, &call_value);
        if esdt_used {
            state.increase_esdt_balance(&to, &esdt_token_identifier, nonce, &esdt_value);
        }

        state.send_balance(
            &to,
            tx_output.send_balance_list.as_slice(),
            &mut tx_result.result_logs,
        )?;
    } else {
        state.increase_balance(&from, &call_value);

        if esdt_used {
            state.increase_esdt_balance(&from, &esdt_token_identifier, nonce, &esdt_value);
        }
    }

    Ok((tx_result, tx_output.async_call))
}

pub fn sc_call_with_async_and_callback(
    tx_input: TxInput,
    state: &mut BlockchainMock,
    contract_map: &ContractMap<TxContext>,
) -> Result<TxResult, BlockchainMockError> {
    let contract_address = tx_input.to.clone();
    let (mut tx_result, opt_async_data) = sc_call(tx_input, state, contract_map)?;
    if tx_result.result_status == 0 {
        if let Some(async_data) = opt_async_data {
            if state.accounts.contains_key(&async_data.to) {
                let async_input = async_call_tx_input(&async_data, &contract_address);

                let async_result =
                    sc_call_with_async_and_callback(async_input, state, contract_map)?;

                tx_result = merge_results(tx_result, async_result.clone());

                let callback_input =
                    async_callback_tx_input(&async_data, &contract_address, &async_result);
                let (callback_result, opt_more_async) =
                    sc_call(callback_input, state, contract_map)?;
                assert!(
                    opt_more_async.is_none(),
                    "successive asyncs currently not supported"
                );
                tx_result = merge_results(tx_result, callback_result);
            } else {
                state
                    .subtract_tx_payment(&contract_address, &async_data.call_value)
                    .unwrap();
                state.add_account(AccountData {
                    address: async_data.to.clone(),
                    nonce: 0,
                    balance: async_data.call_value,
                    esdt: AccountEsdt::default(),
                    username: Vec::new(),
                    storage: HashMap::new(),
                    contract_path: None,
                    contract_owner: None,
                });
            }
        }
    }
    Ok(tx_result)
}

pub fn check_tx_output(tx_id: &str, tx_expect: &TxExpect, tx_result: &TxResult) {
    let have_str = std::str::from_utf8(tx_result.result_message.as_slice()).unwrap();
    assert!(
        tx_expect.status.check(tx_result.result_status),
        "result code mismatch. Tx id: {}. Want: {}. Have: {}. Message: {}",
        tx_id,
        tx_expect.status,
        tx_result.result_status,
        have_str,
    );

    assert_eq!(
        tx_expect.out.len(),
        tx_result.result_values.len(),
        "bad out value. Tx id: {}. Want: {:?}. Have: {:?}",
        tx_id,
        tx_expect.out,
        tx_result.result_values
    );
    for (i, expected_out) in tx_expect.out.iter().enumerate() {
        let actual_value = &tx_result.result_values[i];
        assert!(
            expected_out.check(actual_value.as_slice()),
            "bad out value. Tx id: {}. Want: {}. Have: {}",
            tx_id,
            expected_out,
            verbose_hex(actual_value.as_slice())
        );
    }

    assert!(
        tx_expect.message.check(&tx_result.result_message),
        "result message mismatch. Tx id: {}. Want: {}. Have: {}.",
        tx_id,
        &tx_expect.message,
        have_str,
    );

    match &tx_expect.logs {
        CheckLogs::Star => {},
        CheckLogs::List(expected_logs) => {
            assert!(
                expected_logs.len() == tx_result.result_logs.len(),
                "Log amounts do not match. Tx id: {}. Want: {}. Have: {}",
                tx_id,
                expected_logs.len(),
                tx_result.result_logs.len()
            );

            for (expected_log, actual_log) in expected_logs.iter().zip(tx_result.result_logs.iter())
            {
                assert!(
					actual_log.equals(expected_log),
					"Logs do not match. Tx id: {}.\nWant: Address: {}, Identifier: {}, Topics: {:?}, Data: {}\nHave: Address: {}, Identifier: {}, Topics: {:?}, Data: {}",
					tx_id,
					verbose_hex(&expected_log.address.value),
					bytes_to_string(&expected_log.endpoint.value),
					expected_log.topics.iter().map(|topic| verbose_hex(&topic.value)).collect::<String>(),
					verbose_hex(&expected_log.data.value),
					address_hex(&actual_log.address),
					bytes_to_string(&actual_log.endpoint),
					actual_log.topics.iter().map(|topic| verbose_hex(topic)).collect::<String>(),
					verbose_hex(&actual_log.data),
				);
            }
        },
    }
}
