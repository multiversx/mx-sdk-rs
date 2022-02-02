use crate::{DecodeError, TopDecodeInput};

pub trait TopDecodeMultiInput {
    type ItemInput: TopDecodeInput;

    /// Check if there are more arguments that can be loaded.
    fn has_next(&self) -> bool;

    /// Retrieves an input for deserializing an argument.
    /// If the loader is out of arguments, it will crash by itself with an appropriate error,
    /// without returning.
    /// Use if the next argument is optional, use `has_next` beforehand.
    fn next_arg_input(&mut self) -> Result<Self::ItemInput, DecodeError>;

    // /// Called after retrieving all arguments to validate that extra arguments were not provided.
    // fn assert_no_more_args(&self) {
    //     if self.has_next() {
    //         Self::ManagedTypeErrorApi::error_api_impl().signal_error(err_msg::ARG_WRONG_NUMBER);
    //     }
    // }

    /// Consumes all inputs and ignores them.
    /// After executing this, assert_no_more_args should not fail.
    fn flush_ignore(&mut self) {
        while self.has_next() {
            let _ = self.next_arg_input();
        }
    }
}
