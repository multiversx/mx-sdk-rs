use crate::api::ManagedTypeApi;
use crate::{api::ErrorApi, err_msg};
use crate::{types::*, DynArgOutput};
use elrond_codec::{TopEncode, TopEncodeOutput};

/// Trait that specifies how arguments are serialized in contract calls.
///
/// TODO: unite with DynArg trait when reorganizing argument handling.
pub trait ContractCallArg: Sized {
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O);
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
    type NestedBuffer = Vec<u8>;

    fn set_slice_u8(self, bytes: &[u8]) {
        self.arg_buffer.push_argument_bytes(bytes);
    }

    fn start_nested_encode(&self) -> Self::NestedBuffer {
        Vec::<u8>::new()
    }

    fn finalize_nested_encode(self, nb: Self::NestedBuffer) {
        self.set_slice_u8(nb.as_slice());
    }
}

impl<T> ContractCallArg for T
where
    T: TopEncode,
{
    #[inline]
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        output.push_single_arg(self);
    }
}
