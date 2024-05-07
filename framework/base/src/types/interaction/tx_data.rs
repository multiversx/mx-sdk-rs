mod deploy_call;
mod function_call;
mod tx_code_source;
mod upgrade_call;

pub use deploy_call::DeployCall;
pub use function_call::FunctionCall;
pub use tx_code_source::*;
pub use upgrade_call::UpgradeCall;

use crate::{
    formatter::SCLowerHex,
    types::{ManagedBuffer, ManagedBufferBuilder},
};

use super::TxEnv;

/// Marks the data field of a transaction in `Tx`.
///
/// Can be nothing, deploy data, call data, etc.
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
        let mut result = ManagedBufferBuilder::default();
        result.append_managed_buffer(&self.function_name);
        for arg in self.arg_buffer.raw_arg_iter() {
            result.append_bytes(b"@");
            SCLowerHex::fmt(&*arg, &mut result);
        }
        result.into_managed_buffer()
    }
}
impl<Env> TxDataFunctionCall<Env> for FunctionCall<Env::Api> where Env: TxEnv {}
