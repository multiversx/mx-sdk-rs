use crate::{
    api::ManagedTypeApi,
    contract_base::SendRawWrapper,
    formatter::SCLowerHex,
    types::{
        CodeMetadata, EgldPayment, ManagedAddress, ManagedBuffer, ManagedBufferCachedBuilder,
        ManagedVec,
    },
};

use super::{FunctionCall, ManagedArgBuffer, Tx, TxEnv, TxFrom, TxGas, TxPayment, TxTo};

pub trait TxData<Env>
where
    Env: TxEnv,
{
    fn is_no_call(&self) -> bool;

    fn to_call_data_string(&self) -> ManagedBuffer<Env::Api>;
}

pub trait TxDataFunctionCall<Env>: TxData<Env> + Into<FunctionCall<Env::Api>>
where
    Env: TxEnv,
{
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

impl<Env> TxDataFunctionCall<Env> for () where Env: TxEnv {}

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
impl<Env> TxDataFunctionCall<Env> for FunctionCall<Env::Api> where Env: TxEnv {}
