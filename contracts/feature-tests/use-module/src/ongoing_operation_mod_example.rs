multiversx_sc::imports!();

use multiversx_sc_modules::ongoing_operation::{
    self, CONTINUE_OP, DEFAULT_MIN_GAS_TO_SAVE_PROGRESS, STOP_OP,
};

/// Example of a module using the ongoing operation pattern
#[multiversx_sc::module]
pub trait OngoingOperationModExample: ongoing_operation::OngoingOperationModule {
    #[endpoint(countTo100)]
    fn count_to_100(&self) -> OperationCompletionStatus {
        let mut current_number = self.load_operation::<usize>();
        let run_result = self.run_while_it_has_gas(DEFAULT_MIN_GAS_TO_SAVE_PROGRESS, || {
            if current_number == 100 {
                return STOP_OP;
            }

            current_number += 1;

            CONTINUE_OP
        });

        if run_result == OperationCompletionStatus::InterruptedBeforeOutOfGas {
            self.save_progress(&current_number);
        }

        run_result
    }
}
