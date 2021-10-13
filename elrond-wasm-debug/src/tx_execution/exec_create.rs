use crate::{
    tx_mock::{TxInput, TxOutput, TxResult},
    world_mock::{BlockchainMock, BlockchainMockError},
    AsyncCallTxData, ContractMap, DebugApi,
};

use super::execute_contract_endpoint;

pub fn sc_create(
    tx_input: TxInput,
    contract_path: &[u8],
    state: &mut BlockchainMock,
    contract_map: &ContractMap<DebugApi>,
) -> Result<(TxResult, Option<AsyncCallTxData>), BlockchainMockError> {
    let from = tx_input.from.clone();
    let to = tx_input.to.clone();
    let call_value = tx_input.egld_value.clone();
    let blockchain_info = state.create_tx_info(&to);

    state.subtract_egld_balance(&from, &call_value)?;
    state.subtract_tx_gas(&from, tx_input.gas_limit, tx_input.gas_price);

    let tx_context = DebugApi::new(blockchain_info, tx_input.clone(), TxOutput::default());
    let mut tx_output = execute_contract_endpoint(tx_context, contract_path, contract_map);

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
        state.increase_egld_balance(&from, &call_value);
    }

    Ok((tx_output.result, tx_output.async_call))
}
