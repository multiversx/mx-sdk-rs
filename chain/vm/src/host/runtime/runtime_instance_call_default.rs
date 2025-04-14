use multiversx_chain_core::types::ReturnCode;
use multiversx_chain_vm_executor::BreakpointValue;

use crate::{
    host::context::{GasUsed, TxFunctionName, TxResult},
    vm_err_msg,
};

use super::{RuntimeInstanceCall, RuntimeInstanceCallLambda};

/// Default implementation of `RuntimeInstanceCallLambda`.
///
/// Simply calls the instance as expected.
pub struct RuntimeInstanceCallLambdaDefault;

impl RuntimeInstanceCallLambda for RuntimeInstanceCallLambdaDefault {
    fn call(self, instance_call: RuntimeInstanceCall<'_>) {
        if !instance_call.instance.has_function(instance_call.func_name) {
            *instance_call.tx_context_ref.result_lock() = TxResult::from_function_not_found();
            return;
        }

        let result = instance_call.instance.call(instance_call.func_name);
        let mut tx_result_ref = instance_call.tx_context_ref.result_lock();
        if let Err(err) = result {
            let breakpoint = instance_call
                .instance
                .get_breakpoint_value()
                .expect("error retrieving instance breakpoint value");
            if let Some(error_tx_result) = breakpoint_error_result(breakpoint, err) {
                *tx_result_ref = error_tx_result;
            }
        }

        if tx_result_ref.result_status.is_success() {
            let gas_used = instance_call
                .instance
                .get_points_used()
                .expect("error retrieving gas used");
            tx_result_ref.gas_used = GasUsed::SomeGas(gas_used);
        } else {
            tx_result_ref.gas_used =
                GasUsed::AllGas(instance_call.tx_context_ref.tx_input_box.gas_limit);
        }
    }

    fn override_function_name(&self) -> Option<TxFunctionName> {
        None
    }
}

fn breakpoint_error_result(breakpoint: BreakpointValue, err: String) -> Option<TxResult> {
    match breakpoint {
        BreakpointValue::None => Some(TxResult::from_vm_error(err)),
        BreakpointValue::ExecutionFailed => Some(TxResult::from_vm_error(err)),
        BreakpointValue::AsyncCall => None,   // not an error
        BreakpointValue::SignalError => None, // already handled
        BreakpointValue::OutOfGas => Some(TxResult::from_error(
            ReturnCode::OutOfGas,
            vm_err_msg::NOT_ENOUGH_GAS,
        )),
        BreakpointValue::MemoryLimit => Some(TxResult::from_vm_error(err)),
    }
}
