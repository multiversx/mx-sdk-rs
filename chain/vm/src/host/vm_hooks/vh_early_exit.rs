use multiversx_chain_core::types::ReturnCode;
use multiversx_chain_vm_executor::VMHooksEarlyExit;

pub fn early_exit_out_of_gas() -> VMHooksEarlyExit {
    VMHooksEarlyExit::new(ReturnCode::OutOfGas.as_u64())
}

pub fn early_exit_vm_error(message: &'static str) -> VMHooksEarlyExit {
    VMHooksEarlyExit::new(ReturnCode::ExecutionFailed.as_u64()).with_const_message(message)
}
