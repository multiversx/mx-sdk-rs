use crate::*;
use elrond_codec::*;

/// Abstracts away the loading of multi-arguments.
/// Acts as an abstract source for these arguments.
/// 
/// The main method, `next_arg_input` will provide a decode input,
/// from which any deserializable object can be deserialized.
/// 
/// Structs implementing this trait are also responsible with
/// error handling, such as:
/// - deserialization errors
/// - insufficient arguments
/// - too many arguments
/// For this reason it also requires the SignalError trait.
/// 
/// There are 2 main scenarios for it:
/// - deserializing endpoint arguments directly from the API
/// - deserializing callback arguments saved to storage, from a call data string
/// 
pub trait DynArgInput<I: TopDecodeInput>: SignalError + Sized {
    fn has_next(&self) -> bool;

    fn next_arg_input(&mut self) -> Option<I>;

    fn assert_no_more_args(&self) {
        if self.has_next() {
            self.signal_arg_wrong_number();
        }
    }
}
