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

use multiversx_chain_core::types::ReturnCode;
use multiversx_chain_vm_executor::VMHooksEarlyExit;

use crate::{blockchain::state::AccountData, schedule::GasSchedule, vm_err_msg};

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

    /// Consume gas computed as `multiplier * base_cost`.
    ///
    /// Returns an [`ExecutionFailed`](ReturnCode::ExecutionFailed) early exit if the multiplication overflows.
    fn use_gas_checked_mul(
        &mut self,
        multiplier: usize,
        base_cost: u64,
    ) -> Result<(), VMHooksEarlyExit> {
        let Some(gas) = (multiplier as u64).checked_mul(base_cost) else {
            return Err(VMHooksEarlyExit::new(ReturnCode::ExecutionFailed.as_u64())
                .with_message(vm_err_msg::MULTIPLICATION_OVERFLOW.to_string()));
        };
        self.context.use_gas(gas)
    }

    /// Shortcut for consuming gas for data copies, based on copied data length.
    fn use_gas_for_data_copy(&mut self, num_bytes_copied: usize) -> Result<(), VMHooksEarlyExit> {
        self.use_gas_checked_mul(
            num_bytes_copied,
            self.context
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
}
