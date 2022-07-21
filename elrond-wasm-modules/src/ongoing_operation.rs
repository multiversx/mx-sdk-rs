elrond_wasm::imports!();

use elrond_wasm::elrond_codec::TopEncode;

pub const DEFAULT_MIN_GAS_TO_SAVE_PROGRESS: u64 = 1_000_000;
static mut MIN_GAS_TO_SAVE_PROGRESS: u64 = DEFAULT_MIN_GAS_TO_SAVE_PROGRESS;

pub type LoopOp = bool;
pub const CONTINUE_OP: bool = true;
pub const STOP_OP: bool = false;

#[elrond_wasm::module]
pub trait OngoingOperationModule {
    /// Run the given lambda function until it's either completed or it runs out of gas.
    /// min_gas_to_save_progress should be a reasonable value to save gas.
    /// This can vary a lot based on the given ongoing operation data structures.
    ///
    /// # Usage example: Counting to 100
    /// ```
    /// fn count_to_100(&self) -> OperationCompletionStatus {
    ///     let mut current_number = self.load_operation::<usize>();
    ///     let run_result = self.run_while_it_has_gas(DEFAULT_MIN_GAS_TO_SAVE_PROGRESS, || {
    ///         if current_number == 100 {
    ///             return STOP_OP;
    ///         }
    ///
    ///         current_number += 1;
    ///         
    ///         CONTINUE_OP
    ///     });
    ///     
    ///     if run_result == OperationCompletionStatus::InterruptedBeforeOutOfGas {
    ///         self.save_progress(&current_number);
    ///     }
    ///
    ///     run_result
    /// }
    /// ```
    fn run_while_it_has_gas<Process>(
        &self,
        min_gas_to_save_progress: u64,
        mut process: Process,
    ) -> OperationCompletionStatus
    where
        Process: FnMut() -> LoopOp,
    {
        unsafe {
            MIN_GAS_TO_SAVE_PROGRESS = min_gas_to_save_progress;
        }

        let mut gas_per_iteration = 0;
        let mut gas_before = self.blockchain().get_gas_left();
        loop {
            let loop_op = process();
            if loop_op == STOP_OP {
                break;
            }

            let gas_after = self.blockchain().get_gas_left();
            let current_iteration_cost = gas_before - gas_after;
            if current_iteration_cost > gas_per_iteration {
                gas_per_iteration = current_iteration_cost;
            }

            if !self.can_continue_operation(gas_per_iteration) {
                return OperationCompletionStatus::InterruptedBeforeOutOfGas;
            }

            gas_before = gas_after;
        }

        self.clear_operation();

        OperationCompletionStatus::Completed
    }

    fn can_continue_operation(&self, operation_cost: u64) -> bool {
        let gas_left = self.blockchain().get_gas_left();

        unsafe { gas_left > MIN_GAS_TO_SAVE_PROGRESS + operation_cost }
    }

    /// Load the current ongoing operation.
    /// Will return the default value if no operation is saved.
    fn load_operation<T: TopDecode + Default>(&self) -> T {
        let raw_buffer = self.current_ongoing_operation().get();
        if raw_buffer.is_empty() {
            return T::default();
        }

        match T::top_decode(raw_buffer) {
            Result::Ok(op) => op,
            Result::Err(err) => sc_panic!(err.message_str()),
        }
    }

    /// Save progress for the current operation. The given value can be any serializable type.
    fn save_progress<T: TopEncode>(&self, op: &T) {
        let mut encoded_op = ManagedBuffer::new();
        if let Result::Err(err) = op.top_encode(&mut encoded_op) {
            sc_panic!(err.message_str());
        }

        self.current_ongoing_operation().set(&encoded_op);
    }

    /// Clears the currently stored operation. This is for internal use.
    #[inline]
    fn clear_operation(&self) {
        self.current_ongoing_operation().clear();
    }

    #[storage_mapper("ongoing_operation:currentOngoingOperation")]
    fn current_ongoing_operation(&self) -> SingleValueMapper<ManagedBuffer>;
}
