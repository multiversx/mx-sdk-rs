use alloc::boxed::Box;

use crate::{
    tx_mock::{TxOutput, TxPanic},
    ContractMap, DebugApi,
};

/// Runs contract code using the auto-generated function selector.
/// The endpoint name is taken from the tx context.
/// Catches and wraps any panics thrown in the contract.
pub fn execute_contract_endpoint(
    tx_context: DebugApi,
    contract_identifier: &[u8],
    contract_map: &ContractMap<DebugApi>,
) -> TxOutput {
    let func_name = tx_context.input_ref().func_name.clone();
    let contract_inst = contract_map.new_contract_instance(contract_identifier, tx_context);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let call_successful = contract_inst.call(func_name.as_slice());
        if !call_successful {
            std::panic::panic_any(TxPanic {
                status: 1,
                message: b"invalid function (not found)".to_vec(),
            });
        }
        let context = contract_inst.into_api();
        context.into_output()
    }));
    match result {
        Ok(tx_output) => tx_output,
        Err(panic_any) => panic_result(panic_any),
    }
}

fn panic_result(panic_any: Box<dyn std::any::Any + std::marker::Send>) -> TxOutput {
    if panic_any.downcast_ref::<TxOutput>().is_some() {
        // async calls panic with the tx output directly
        // it is not a failure, simply a way to kill the execution
        return *panic_any.downcast::<TxOutput>().unwrap();
    }

    if let Some(panic_obj) = panic_any.downcast_ref::<TxPanic>() {
        return TxOutput::from_panic_obj(panic_obj);
    }

    if let Some(panic_string) = panic_any.downcast_ref::<String>() {
        return TxOutput::from_panic_string(panic_string.as_str());
    }

    TxOutput::from_panic_string("unknown panic")
}
