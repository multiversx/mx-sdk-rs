use multiversx_chain_core::types::ReturnCode;
use multiversx_chain_vm_executor::{BreakpointValue, InstanceCallError, VMHooksEarlyExit};

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
        default_instance_call(instance_call);
    }

    fn override_function_name(&self) -> Option<TxFunctionName> {
        None
    }
}

fn default_instance_call(instance_call: RuntimeInstanceCall<'_>) {
    if !instance_call.instance.has_function(instance_call.func_name) {
        *instance_call.tx_context_ref.result_lock() = TxResult::from_function_not_found();
        return;
    }

    let result = instance_call.instance.call(instance_call.func_name);
    let mut tx_result_ref = instance_call.tx_context_ref.result_lock();
    if let Err(call_error) = result {
        if let Some(error_tx_result) = instance_call_error_result(call_error) {
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

fn instance_call_error_result(call_error: InstanceCallError) -> Option<TxResult> {
    match call_error {
        InstanceCallError::FunctionNotFound => Some(TxResult::from_function_not_found()),
        InstanceCallError::RuntimeError(error) => Some(TxResult::from_vm_error(error.to_string())),
        InstanceCallError::VMHooksEarlyExit(vm_hooks_early_exit) => {
            vm_hooks_early_exit_result(vm_hooks_early_exit)
        },
        InstanceCallError::Breakpoint(breakpoint_value) => {
            breakpoint_error_result(breakpoint_value, "breakpoint".to_owned())
        },
    }
}

fn vm_hooks_early_exit_result(vm_hooks_early_exit: VMHooksEarlyExit) -> Option<TxResult> {
    if vm_hooks_early_exit.code == BreakpointValue::AsyncCall.as_u64() {
        None
    } else {
        Some(TxResult::from_error(
            ReturnCode::from_u64(vm_hooks_early_exit.code).expect("invalid return code"),
            vm_hooks_early_exit.message.into_owned(),
        ))
    }
}
