use multiversx_chain_vm_executor::{CompilationOptions, Instance};

use crate::{
    display_util::address_hex,
    tx_mock::{TxContext, TxContextStack},
    with_shared::Shareable,
};

use super::{execute_current_tx_context_input, BlockchainVMRef};

const COMPILATION_OPTIONS: CompilationOptions = CompilationOptions {
    gas_limit: 1,
    unmetered_locals: 0,
    max_memory_grow: 0,
    max_memory_grow_delta: 0,
    opcode_trace: false,
    metering: false,
    runtime_breakpoints: false,
};

impl BlockchainVMRef {
    /// Runs contract code using the auto-generated function selector.
    /// The endpoint name is taken from the tx context.
    /// Catches and wraps any panics thrown in the contract.
    pub fn execute_tx_context(&self, tx_context: TxContext) -> TxContext {
        let mut tx_context_sh = Shareable::new(tx_context);
        TxContextStack::execute_on_vm_stack(&mut tx_context_sh, execute_current_tx_context_input);
        tx_context_sh.into_inner()
    }

    pub fn get_contract_instance(&self, tx_context: &TxContext) -> Box<dyn Instance> {
        let contract_code = get_contract_identifier(tx_context);
        self.executor
            .new_instance(contract_code.as_slice(), &COMPILATION_OPTIONS)
            .expect("error instantiating executor instance")
    }
}

fn get_contract_identifier(tx_context: &TxContext) -> Vec<u8> {
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
