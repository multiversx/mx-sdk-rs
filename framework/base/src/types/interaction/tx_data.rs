use crate::{
    api::ManagedTypeApi,
    contract_base::SendRawWrapper,
    formatter::SCLowerHex,
    types::{CodeMetadata, ManagedAddress, ManagedBuffer, ManagedBufferCachedBuilder, ManagedVec},
};

use super::{FunctionCall, ManagedArgBuffer, TxEnv};

#[derive(Default)]
pub struct TxDataDeploy<Env>
where
    Env: TxEnv,
{
    code: ManagedBuffer<Env::Api>,
    code_metadata: CodeMetadata,
    arg_buffer: ManagedArgBuffer<Env::Api>,
}

impl<Env> TxDataDeploy<Env>
where
    Env: TxEnv,
{
    pub fn new(
        code: ManagedBuffer<Env::Api>,
        code_metadata: CodeMetadata,
        arg_buffer: ManagedArgBuffer<Env::Api>,
    ) -> Self {
        TxDataDeploy {
            code,
            code_metadata,
            arg_buffer,
        }
    }

    pub fn execute_deploy(
        &self,
    ) -> (
        ManagedAddress<Env::Api>,
        ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) {
        let wrap = SendRawWrapper::<Env::Api>::new();
        wrap.deploy_contract(
            gas,
            payment,
            &self.code,
            self.code_metadata,
            &self.arg_buffer,
        )
    }
}

impl<Env> TxData<Env> for TxDataDeploy<Env>
where
    Env: TxEnv,
{
    type Deploy = TxDataDeploy<Env>;

    fn is_no_call(&self) -> bool {
        false
    }

    fn to_call_data_string(&self) -> ManagedBuffer<Env::Api> {
        // Implement as needed for deployment-specific data
        let mut result = ManagedBufferCachedBuilder::default();
        result.append_managed_buffer(&self.code);
        // Add other fields as needed
        result.into_managed_buffer()
    }

    fn deploy(&self) -> Option<Self::Deploy> {
        Some(Self {
            code: self.code.clone(),
            code_metadata: self.code_metadata.clone(),
            arg_buffer: self.arg_buffer.clone(),
        })
    }
}

pub trait TxData<Env>
where
    Env: TxEnv,
{
    type Deploy = TxDataDeploy<Env>;

    fn is_no_call(&self) -> bool;

    fn to_call_data_string(&self) -> ManagedBuffer<Env::Api>;

    fn deploy(&self) -> Option<Self::Deploy>;
    // fn is_deploy(&self) -> bool;
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

    type Deploy = TxDataDeploy<Env>;

    fn deploy(&self) -> Option<Self::Deploy> {
        todo!()
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

    fn deploy(&self) -> Option<Self::Deploy> {
        todo!()
    }
}
impl<Env> TxDataFunctionCall<Env> for FunctionCall<Env::Api> where Env: TxEnv {}
