use multiversx_chain_core::types::ReturnCode;
use multiversx_chain_vm_executor::VMHooksEarlyExit;

const OUT_OF_GAS_CODE: u64 = ReturnCode::OutOfGas.as_u64();

pub fn early_exit_out_of_gas() -> VMHooksEarlyExit {
    VMHooksEarlyExit::new(OUT_OF_GAS_CODE)
}
