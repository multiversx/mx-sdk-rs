use std::collections::HashMap;

use crate::{
    address_hex, async_call_tx_input, async_callback_tx_input, merge_results,
    try_execute_builtin_function,
    tx_mock::{TxInput, TxOutput, TxResult},
    world_mock::{AccountData, AccountEsdt, BlockchainMock, BlockchainMockError},
    AsyncCallTxData, ContractMap, DebugApi,
};

use super::execute_contract_endpoint;

pub fn sc_call(
    tx_input: TxInput,
    state: &mut BlockchainMock,
    contract_map: &ContractMap<DebugApi>,
) -> Result<(TxResult, Option<AsyncCallTxData>), BlockchainMockError> {
    if let Some(tx_result) = try_execute_builtin_function(&tx_input, state) {
        return Ok((tx_result, None));
    }

    let from = tx_input.from.clone();
    let to = tx_input.to.clone();
    let egld_value = tx_input.egld_value.clone();
    let esdt_values = tx_input.esdt_values.clone();
    let blockchain_info = state.create_tx_info(&to);

    state.subtract_egld_balance(&from, &egld_value)?;
    state.subtract_tx_gas(&from, tx_input.gas_limit, tx_input.gas_price);
    state.subtract_multi_esdt_balance(&from, tx_input.esdt_values.as_slice());

    let contract_account = state
        .accounts
        .get_mut(&to)
        .unwrap_or_else(|| panic!("Recipient account not found: {}", address_hex(&to)));

    let contract_path = &contract_account
        .contract_path
        .clone()
        .unwrap_or_else(|| panic!("Recipient account is not a smart contract"));

    let tx_context = DebugApi::new(
        blockchain_info,
        tx_input,
        TxOutput {
            contract_storage: contract_account.storage.clone(),
            result: TxResult::empty(),
            send_balance_list: Vec::new(),
            async_call: None,
        },
    );

    let tx_output = execute_contract_endpoint(tx_context, contract_path, contract_map);
    let mut tx_result = tx_output.result;

    if tx_result.result_status == 0 {
        // replace storage with new one
        let _ = std::mem::replace(&mut contract_account.storage, tx_output.contract_storage);

        state.increase_egld_balance(&to, &egld_value);
        state.increase_multi_esdt_balance(&to, esdt_values.as_slice());

        state.send_balance(
            &to,
            tx_output.send_balance_list.as_slice(),
            &mut tx_result.result_logs,
        )?;
    } else {
        // revert
        state.increase_egld_balance(&from, &egld_value);
        state.increase_multi_esdt_balance(&from, esdt_values.as_slice());
    }

    Ok((tx_result, tx_output.async_call))
}

pub fn sc_call_with_async_and_callback(
    tx_input: TxInput,
    state: &mut BlockchainMock,
    contract_map: &ContractMap<DebugApi>,
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
                    .subtract_egld_balance(&contract_address, &async_data.call_value)
                    .unwrap();
                state.add_account(AccountData {
                    address: async_data.to.clone(),
                    nonce: 0,
                    egld_balance: async_data.call_value,
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
