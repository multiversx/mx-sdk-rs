use alloc::boxed::Box;
use elrond_wasm::contract_base::CallableContract;

use crate::{
    tx_mock::{TxContext, TxContextRef, TxOutput, TxPanic, TxResult},
    ContractMap, DebugApi,
};

/// Runs contract code using the auto-generated function selector.
/// The endpoint name is taken from the tx context.
/// Catches and wraps any panics thrown in the contract.
pub fn execute_tx_context(tx_context_ref: TxContextRef) -> TxResult {
    let func_name = tx_context_ref.tx_input_box.func_name.as_slice();
    let contract_identifier = get_contract_identifier(&tx_context_ref);
    let contract_map = &tx_context_ref.blockchain_ref().contract_map;
    let contract_instance =
        contract_map.new_contract_instance(contract_identifier.as_slice(), tx_context_ref.clone());
    execute_contract_instance_endpoint(contract_instance, func_name)
}

fn get_contract_identifier(tx_context: &TxContext) -> Vec<u8> {
    tx_context
        .blockchain_cache
        .with_account(&tx_context.tx_input_box.to, |account| {
            account
                .contract_path
                .clone()
                .unwrap_or_else(|| panic!("Recipient account is not a smart contract"))
        })
}

/// The actual execution and the extraction/wrapping of results.
fn execute_contract_instance_endpoint(
    contract_instance: Box<dyn CallableContract<DebugApi>>,
    endpoint_name: &[u8],
) -> TxResult {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let call_successful = contract_instance.call(endpoint_name);
        if !call_successful {
            std::panic::panic_any(TxPanic {
                status: 1,
                message: b"invalid function (not found)".to_vec(),
            });
        }
        let debug_api = contract_instance.into_api();
        debug_api.into_tx_result()
    }));
    match result {
        Ok(tx_output) => tx_output,
        Err(panic_any) => panic_result(panic_any),
    }
}

fn panic_result(panic_any: Box<dyn std::any::Any + std::marker::Send>) -> TxResult {
    if panic_any.downcast_ref::<TxResult>().is_some() {
        // async calls panic with the tx output directly
        // it is not a failure, simply a way to kill the execution
        return *panic_any.downcast::<TxResult>().unwrap();
    }

    if let Some(panic_obj) = panic_any.downcast_ref::<TxPanic>() {
        return TxResult::from_panic_obj(panic_obj);
    }

    if let Some(panic_string) = panic_any.downcast_ref::<String>() {
        return TxResult::from_panic_string(panic_string.as_str());
    }

    TxResult::from_unknown_panic()
}
