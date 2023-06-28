use multiversx_chain_vm::tx_mock::TxPanic;
use multiversx_chain_vm_executor::BreakpointValue;
use multiversx_sc::err_msg;

/// Catches all thrown panics, as follows:
/// - BreakpointValue is considered expected abortion of execution and already handled, for them it returns Ok;
/// - TxPanic panics are considered VM failures with additional information. We are trying to get rid of them;
/// - All other panics are treated as user errors;
/// - The closure argument can also opt to return a TxPanic, without having to throw it. This will pe passed on as is.
pub fn catch_tx_panic<F>(panic_message_flag: bool, f: F) -> Result<(), TxPanic>
where
    F: FnOnce() -> Result<(), TxPanic>,
{
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
    match result {
        Ok(result) => result,
        Err(panic_any) => {
            if panic_any.downcast_ref::<BreakpointValue>().is_some() {
                // breakpoints are considered to be already handled
                Ok(())
            } else {
                // fallback, general panics
                Err(interpret_panic_as_tx_panic(panic_any, panic_message_flag))
            }
        },
    }
}

/// Interprets a panic thrown during execution as a tx failure.
/// Note: specific tx outcomes from the debugger are signalled via specific panic objects.
fn interpret_panic_as_tx_panic(
    panic_any: Box<dyn std::any::Any + std::marker::Send>,
    panic_message_flag: bool,
) -> TxPanic {
    if let Some(panic_string) = panic_any.downcast_ref::<String>() {
        return interpret_panic_str_as_tx_result(panic_string.as_str(), panic_message_flag);
    }

    if let Some(panic_string) = panic_any.downcast_ref::<&str>() {
        return interpret_panic_str_as_tx_result(panic_string, panic_message_flag);
    }

    TxPanic::user_error("unknown panic object")
}

pub fn interpret_panic_str_as_tx_result(panic_str: &str, panic_message_flag: bool) -> TxPanic {
    if panic_message_flag {
        TxPanic::user_error(&format!("panic occurred: {panic_str}"))
    } else {
        TxPanic::user_error(err_msg::PANIC_OCCURRED)
    }
}
