use elrond_wasm::types::Address;
use mandos::{TxDeploy, TxExpect};
use num_bigint::BigUint;
use num_traits::Zero;

use crate::{
    execute_helper_functions::*, execute_tx, AsyncCallTxData, BlockchainMock, BlockchainMockError,
    ContractMap, TxContext, TxInput, TxOutput, TxResult,
};

pub fn execute(
    state: &mut BlockchainMock,
    contract_map: &ContractMap<TxContext>,
    tx_id: &str,
    tx: &TxDeploy,
    expect: &Option<TxExpect>,
) {
    let tx_input = TxInput {
        from: tx.from.value.into(),
        to: Address::zero(),
        call_value: tx.call_value.value.clone(),
        esdt_value: BigUint::zero(),
        esdt_token_identifier: Vec::new(),
        nonce: 0,
        func_name: b"init".to_vec(),
        args: tx
            .arguments
            .iter()
            .map(|scen_arg| scen_arg.value.clone())
            .collect(),
        gas_limit: tx.gas_limit.value,
        gas_price: tx.gas_price.value,
        tx_hash: generate_tx_hash_dummy(tx_id),
    };
    state.increase_nonce(&tx_input.from);
    let (tx_result, _) = sc_create(tx_input, &tx.contract_code.value, state, contract_map).unwrap();
    if let Some(tx_expect) = expect {
        check_tx_output(tx_id, tx_expect, &tx_result);
    }
}

pub fn sc_create(
    tx_input: TxInput,
    contract_path: &[u8],
    state: &mut BlockchainMock,
    contract_map: &ContractMap<TxContext>,
) -> Result<(TxResult, Option<AsyncCallTxData>), BlockchainMockError> {
    let from = tx_input.from.clone();
    let to = tx_input.to.clone();
    let call_value = tx_input.call_value.clone();
    let blockchain_info = state.create_tx_info(&to);

    state.subtract_tx_payment(&from, &call_value)?;
    state.subtract_tx_gas(&from, tx_input.gas_limit, tx_input.gas_price);

    let esdt_token_identifier = tx_input.esdt_token_identifier.clone();
    let nonce = tx_input.nonce.clone();
    let esdt_value = tx_input.esdt_value.clone();
    let esdt_used = !esdt_token_identifier.is_empty() && esdt_value > 0u32.into();

    if esdt_used {
        state.substract_esdt_balance(&from, &esdt_token_identifier, nonce, &esdt_value)
    }

    let tx_context = TxContext::new(blockchain_info, tx_input.clone(), TxOutput::default());
    let mut tx_output = execute_tx(tx_context, contract_path, contract_map);

    if tx_output.result.result_status == 0 {
        let new_address = state.create_account_after_deploy(
            &tx_input,
            tx_output.contract_storage,
            contract_path.to_vec(),
        );
        state.send_balance(
            &new_address,
            tx_output.send_balance_list.as_slice(),
            &mut tx_output.result.result_logs,
        )?;
    } else {
        state.increase_balance(&from, &call_value);

        if esdt_used {
            state.increase_esdt_balance(&from, &esdt_token_identifier, nonce, &esdt_value);
        }
    }

    Ok((tx_output.result, tx_output.async_call))
}
