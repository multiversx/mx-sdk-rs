use std::rc::Rc;

use alloc::boxed::Box;

use crate::{
    address_hex,
    tx_mock::{TxContext, TxContextStack, TxFunctionName, TxPanic, TxResult},
    world_mock::ContractContainer,
    DebugApi,
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
    let tx_context_ref = DebugApi::new(tx_context_rc.clone());

    let func_name = &tx_context_ref.tx_input_box.func_name;
    let contract_identifier = get_contract_identifier(&tx_context_ref);
    let contract_map = &tx_context_rc.blockchain_ref().contract_map;

    // Not redundant at all, func_name is borrowed from it...
    #[allow(clippy::redundant_clone)]
    let contract_container =
        contract_map.get_contract(contract_identifier.as_slice(), tx_context_ref.clone());

    TxContextStack::static_push(tx_context_rc.clone());
    let tx_result = execute_contract_instance_endpoint(contract_container, func_name);

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

/// The actual execution and the extraction/wrapping of results.
fn execute_contract_instance_endpoint(
    contract_container: &ContractContainer,
    endpoint_name: &TxFunctionName,
) -> TxResult {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let call_successful = contract_container.call(endpoint_name);
        if !call_successful {
            std::panic::panic_any(TxPanic {
                status: 1,
                message: "invalid function (not found)".to_string(),
            });
        }
        DebugApi::new_from_static().into_tx_result()
    }));
    match result {
        Ok(tx_output) => tx_output,
        Err(panic_any) => interpret_panic_as_tx_result(panic_any),
    }
}

/// Interprets a panic thrown during execution as a tx failure.
/// Note: specific tx outcomes from the debugger are signalled via specific panic objects.
pub fn interpret_panic_as_tx_result(
    panic_any: Box<dyn std::any::Any + std::marker::Send>,
) -> TxResult {
    if panic_any.downcast_ref::<TxResult>().is_some() {
        // async calls panic with the tx output directly
        // it is not a failure, simply a way to kill the execution
        return *panic_any.downcast::<TxResult>().unwrap();
    }

    if let Some(panic_obj) = panic_any.downcast_ref::<TxPanic>() {
        return TxResult::from_panic_obj(panic_obj);
    }

    if let Some(panic_string) = panic_any.downcast_ref::<String>() {
        return TxResult::from_panic_string(panic_string.as_str());
    }

    TxResult::from_unknown_panic()
}
