use multiversx_chain_vm_executor::{CompilationOptions, Instance};

use crate::{
    display_util::address_hex,
    tx_mock::{TxContext, TxContextStack},
};

use super::{execute_current_tx_context_input, BlockchainVMRef};

pub const COMPILATION_OPTIONS: CompilationOptions = CompilationOptions {
    gas_limit: 1,
    unmetered_locals: 0,
    max_memory_grow: 0,
    max_memory_grow_delta: 0,
    opcode_trace: false,
    metering: false,
    runtime_breakpoints: false,
};

impl BlockchainVMRef {
    pub fn get_contract_instance(&self, tx_context: &TxContext) -> Box<dyn Instance> {
        let contract_code = get_contract_identifier(tx_context);
        self.executor
            .new_instance(contract_code.as_slice(), &COMPILATION_OPTIONS)
            .expect("error instantiating executor instance")
    }
}

pub fn get_contract_identifier(tx_context: &TxContext) -> Vec<u8> {
    tx_context
        .tx_cache
        .with_account(&tx_context.tx_input_box.to, |account| {
            account.contract_path.clone().unwrap_or_else(|| {
                panic!(
                    "Recipient account is not a smart contract {}",
                    address_hex(&tx_context.tx_input_box.to)
                )
            })
        })
}
