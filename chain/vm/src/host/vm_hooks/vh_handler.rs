mod vh_blockchain;
mod vh_call_value;
mod vh_crypto;
mod vh_endpoint_arg;
mod vh_endpoint_finish;
mod vh_error;
mod vh_log;
mod vh_managed_types;
mod vh_send;
mod vh_storage;

use multiversx_chain_vm_executor::VMHooksEarlyExit;

use crate::{blockchain::state::AccountData, host::context::TxErrorTrace, schedule::GasSchedule};

use super::VMHooksContext;

/// Defines all methods that can handle VM hooks. They are spread out over several files.
#[derive(Debug)]
pub struct VMHooksHandler<C: VMHooksContext> {
    pub(crate) context: C,
}

impl<C: VMHooksContext> VMHooksHandler<C> {
    pub fn new(context: C) -> Self {
        VMHooksHandler { context }
    }

    /// Reference to the gas schedule. Provided for convenience.
    fn gas_schedule(&self) -> &GasSchedule {
        self.context.gas_schedule()
    }

    /// Consume amount of gas. Provided for convenience.
    fn use_gas(&mut self, gas: u64) -> Result<(), VMHooksEarlyExit> {
        self.context.use_gas(gas)
    }

    /// Shortcut for consuming gas for data copies, based on copied data length.
    fn use_gas_for_data_copy(&mut self, num_bytes_copied: usize) -> Result<(), VMHooksEarlyExit> {
        self.context.use_gas(
            num_bytes_copied as u64
                * self
                    .context
                    .gas_schedule()
                    .base_operation_cost
                    .data_copy_per_byte,
        )
    }

    /// For ownership reasons, needs to return a clone.
    ///
    /// Can be optimized, but is not a priority right now.
    fn current_account_data(&self) -> AccountData {
        self.context
            .account_data(&self.context.input_ref().to)
            .expect("missing current account")
    }

    /// Logs an error, in 2 ways:
    /// - in the `error_trace` field of the `TxResult`,
    /// - in the standard log, at info level.
    ///
    /// TODO: call from more places in the codebase, similar to the Go VM.
    ///
    /// TODO: consider re-design in both Go and Rust VM,
    /// the current implementation is very implementation-dependent.
    fn error_trace(&mut self, trace_message: &str) {
        let func_name = self.context.input_ref().func_name.clone();

        log::info!("Error in {func_name}: {trace_message}");

        let mut tx_result = self.context.result_lock();
        tx_result.error_trace.push(TxErrorTrace {
            function_name: func_name,
            error_trace_message: trace_message.to_string(),
            additional_info: Vec::new(),
        });
    }
}
