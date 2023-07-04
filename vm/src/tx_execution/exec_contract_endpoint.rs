use std::rc::Rc;

use multiversx_chain_vm_executor::{CompilationOptions, Instance};

use crate::{
    display_util::address_hex,
    tx_mock::{TxContext, TxContextStack},
};

const COMPILATION_OPTIONS: CompilationOptions = CompilationOptions {
    gas_limit: 1,
    unmetered_locals: 0,
    max_memory_grow: 0,
    max_memory_grow_delta: 0,
    opcode_trace: false,
    metering: false,
    runtime_breakpoints: false,
};

/// Manages the stack.
///
/// Pushes the context to the stack, executes closure, pops after.
pub fn execute_on_vm_stack<F>(tx_context: TxContext, f: F) -> TxContext
where
    F: FnOnce(),
{
    let tx_context_rc = Rc::new(tx_context);
    TxContextStack::static_push(tx_context_rc);

    f();

    let tx_context_rc = TxContextStack::static_pop();
    Rc::try_unwrap(tx_context_rc).unwrap()
}

/// Runs contract code using the auto-generated function selector.
/// The endpoint name is taken from the tx context.
/// Catches and wraps any panics thrown in the contract.
pub fn execute_tx_context(tx_context: TxContext) -> TxContext {
    let func_name = tx_context.tx_input_box.func_name.clone();
    let instance = get_contract_instance(&tx_context);

    execute_on_vm_stack(tx_context, || {
        instance.call(func_name.as_str()).expect("execution error");
    })
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

fn get_contract_instance(tx_context: &TxContext) -> Box<dyn Instance> {
    let contract_code = get_contract_identifier(tx_context);
    let executor = &tx_context.blockchain_ref().executor;
    executor
        .new_instance(contract_code.as_slice(), &COMPILATION_OPTIONS)
        .expect("error instantiating executor instance")
}
