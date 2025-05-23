use multiversx_chain_vm_executor::Executor;

use crate::host::runtime::RuntimeWeakRef;

pub fn new_prod_executor(_runtime_ref: RuntimeWeakRef) -> Box<dyn Executor + Send + Sync> {
    panic!("ExperimentalExecutor not available, need to activate features = [\"wasmer-experimental\"] in multiversx-sc-scenario or multiversx-chain-vm")
}
