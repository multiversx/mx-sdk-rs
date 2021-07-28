use crate::api::ErrorApi;
use crate::types::*;
use elrond_codec::{TopEncode, TopEncodeOutput};

pub fn serialize_contract_call_arg<I, A>(arg: I, arg_buffer: &mut ArgBuffer, error_api: A)
where
    I: ContractCallArg,
    A: ErrorApi,
{
    // TODO: convert to fast exit
    if let Result::Err(sc_err) = arg.push_async_arg(arg_buffer) {
        error_api.signal_error(sc_err.as_bytes());
    }
}

/// Trait that specifies how arguments are serialized in contract calls.
///
/// TODO: unite with DynArg trait when reorganizing argument handling.
pub trait ContractCallArg: Sized {
    fn push_async_arg(&self, serializer: &mut ArgBuffer) -> Result<(), SCError>;
}

/// Local adapter the connects the ArgBuffer to the TopEncode trait.
struct ContractCallArgOutput<'s> {
    arg_buffer: &'s mut ArgBuffer,
}

impl<'c> ContractCallArgOutput<'c> {
    #[inline]
    fn new(arg_buffer: &'c mut ArgBuffer) -> Self {
        ContractCallArgOutput { arg_buffer }
    }
}

impl<'c> TopEncodeOutput for ContractCallArgOutput<'c> {
    fn set_slice_u8(self, bytes: &[u8]) {
        self.arg_buffer.push_argument_bytes(bytes);
    }
}

impl<T> ContractCallArg for T
where
    T: TopEncode,
{
    #[inline]
    #[allow(clippy::redundant_closure)]
    fn push_async_arg(&self, serializer: &mut ArgBuffer) -> Result<(), SCError> {
        self.top_encode(ContractCallArgOutput::new(serializer))
            .map_err(|err| SCError::from(err))
    }
}
