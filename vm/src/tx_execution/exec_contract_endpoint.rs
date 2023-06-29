use std::rc::Rc;

use multiversx_chain_vm_executor::CompilationOptions;

use crate::{
    display_util::address_hex,
    tx_mock::{TxContext, TxContextRef, TxContextStack, TxResult},
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

/// Runs contract code using the auto-generated function selector.
/// The endpoint name is taken from the tx context.
/// Catches and wraps any panics thrown in the contract.
pub fn execute_tx_context(tx_context: TxContext) -> (TxContext, TxResult) {
    let tx_context_rc = Rc::new(tx_context);
    let (tx_context_rc, tx_result) = execute_tx_context_rc(tx_context_rc);
    let tx_context = Rc::try_unwrap(tx_context_rc).unwrap();
    (tx_context, tx_result)
}

/// The actual core of the execution.
/// The argument is returned and can be unwrapped,
/// since the lifetimes of all other references created from it cannot outlive this function.
fn execute_tx_context_rc(tx_context_rc: Rc<TxContext>) -> (Rc<TxContext>, TxResult) {
    let tx_context_ref = TxContextRef::new(tx_context_rc.clone());

    let func_name = &tx_context_ref.tx_input_box.func_name;
    let contract_code = get_contract_identifier(&tx_context_ref);
    let executor = &tx_context_rc.blockchain_ref().executor;
    let instance = executor
        .new_instance(contract_code.as_slice(), &COMPILATION_OPTIONS)
        .expect("error instantiating executor instance");

    TxContextStack::static_push(tx_context_rc.clone());

    instance.call(func_name.as_str()).expect("execution error");

    let tx_result = TxContextRef::new_from_static().into_tx_result();
    let tx_context_rc = TxContextStack::static_pop();
    (tx_context_rc, tx_result)
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
