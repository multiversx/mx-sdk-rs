use crate::{
    api::ManagedTypeApi,
    formatter::SCLowerHex,
    types::{ManagedBuffer, ManagedBufferCachedBuilder},
};

use super::{FunctionCall, TxEnv};

pub trait TxData<Env>
where
    Env: TxEnv,
{
    fn is_no_call(&self) -> bool;

    fn to_call_data_string(&self) -> ManagedBuffer<Env::Api>;
}

impl<Env> TxData<Env> for ()
where
    Env: TxEnv,
{
    fn is_no_call(&self) -> bool {
        true
    }

    fn to_call_data_string(&self) -> ManagedBuffer<Env::Api> {
        ManagedBuffer::new()
    }
}

impl<Env> TxData<Env> for FunctionCall<Env::Api>
where
    Env: TxEnv,
{
    fn is_no_call(&self) -> bool {
        self.is_empty()
    }

    fn to_call_data_string(&self) -> ManagedBuffer<Env::Api> {
        let mut result = ManagedBufferCachedBuilder::default();
        result.append_managed_buffer(&self.function_name);
        for arg in self.arg_buffer.raw_arg_iter() {
            result.append_bytes(b"@");
            SCLowerHex::fmt(&*arg, &mut result);
        }
        result.into_managed_buffer()
    }
}
