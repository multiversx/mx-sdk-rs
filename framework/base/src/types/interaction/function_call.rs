use multiversx_sc_codec::TopEncodeMulti;

use crate::{
    api::ManagedTypeApi,
    formatter::SCLowerHex,
    types::{ManagedBuffer, ManagedBufferCachedBuilder},
};

use super::ManagedArgBuffer;

/// Encodes a function call on the blockchain, composed of a function name and its encoded arguments.
///
/// Can be used as a multi-argument, to embed a call within a call.
pub struct FunctionCall<Api>
where
    Api: ManagedTypeApi,
{
    pub function_name: ManagedBuffer<Api>,
    pub arg_buffer: ManagedArgBuffer<Api>,
}

impl<Api> FunctionCall<Api>
where
    Api: ManagedTypeApi,
{
    pub fn new<N: Into<ManagedBuffer<Api>>>(function_name: N) -> Self {
        FunctionCall {
            function_name: function_name.into(),
            arg_buffer: ManagedArgBuffer::new(),
        }
    }

    pub fn argument<T: TopEncodeMulti>(mut self, arg: &T) -> Self {
        self.arg_buffer.push_multi_arg(arg);
        self
    }

    pub fn to_call_data_string(&self) -> ManagedBuffer<Api> {
        let mut result = ManagedBufferCachedBuilder::default();
        result.append_managed_buffer(&self.function_name);
        for arg in self.arg_buffer.raw_arg_iter() {
            result.append_bytes(b"@");
            SCLowerHex::fmt(&*arg, &mut result);
        }
        result.into_managed_buffer()
    }
}
