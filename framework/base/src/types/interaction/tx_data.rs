use crate::{
    api::ManagedTypeApi,
    formatter::SCLowerHex,
    types::{ManagedBuffer, ManagedBufferCachedBuilder},
};

use super::FunctionCall;

pub trait TxData<Api>
where
    Api: ManagedTypeApi,
{
    fn is_no_call(&self) -> bool;

    fn to_call_data_string(&self) -> ManagedBuffer<Api>;
}

impl<Api> TxData<Api> for ()
where
    Api: ManagedTypeApi,
{
    fn is_no_call(&self) -> bool {
        true
    }

    fn to_call_data_string(&self) -> ManagedBuffer<Api> {
        ManagedBuffer::new()
    }
}

impl<Api> TxData<Api> for FunctionCall<Api>
where
    Api: ManagedTypeApi,
{
    fn is_no_call(&self) -> bool {
        self.is_empty()
    }

    fn to_call_data_string(&self) -> ManagedBuffer<Api> {
        let mut result = ManagedBufferCachedBuilder::default();
        result.append_managed_buffer(&self.function_name);
        for arg in self.arg_buffer.raw_arg_iter() {
            result.append_bytes(b"@");
            SCLowerHex::fmt(&*arg, &mut result);
        }
        result.into_managed_buffer()
    }
}
