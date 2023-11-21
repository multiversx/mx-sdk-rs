use crate::{
    api::ManagedTypeApi,
    contract_base::SendRawWrapper,
    formatter::SCLowerHex,
    types::{
        CodeMetadata, EgldPayment, ManagedAddress, ManagedBuffer, ManagedBufferCachedBuilder,
        ManagedVec,
    },
};

use super::{
    FunctionCall, ManagedArgBuffer, Tx, TxCallback, TxEnv, TxFrom, TxGas, TxPayment, TxTo,
};

// pub trait TxDataDeploy<Env>: TxData<Env> {
//     fn deploy(
//         self,
//         code: ManagedBuffer<Env>,
//         metadata: CodeMetadata,
//         args: ManagedArgBuffer<Env>,
//     ) -> Self;
//     fn execute_deploy(&self) -> (ManagedAddress<Env>, ManagedVec<Env, ManagedBuffer<Env>>);
// }

// impl<Env, From, To, Payment, Gas, Data, Callback> TxDataDeploy<Env>
//     for Tx<Env, (), (), EgldPayment<Env::Api>, (), Data, Callback>
// where
//     Env: TxEnv,
//     From: TxFrom<Env>,
//     To: TxTo<Env>,
//     Payment: TxPayment<Env>,
//     Gas: TxGas<Env>,
//     Data: TxData<Env>,
//     Callback: TxCallback<Env>,
// {
//     fn deploy(
//         mut self,
//         code: ManagedBuffer<Env::Api>,
//         metadata: CodeMetadata,
//         args: ManagedArgBuffer<Env::Api>,
//     ) -> Self {
//         self.data = TxDataDeploy::new(code, metadata, args).into();
//         self
//     }

//     fn execute_deploy(
//         &self,
//     ) -> (
//         ManagedAddress<Env::Api>,
//         ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
//     ) {
//         // Perform the deployment using the configured parameters
//         let wrap = SendRawWrapper::<Env::Api>::new();
//         wrap.deploy_contract(
//             self.gas,
//             self.payment,
//             &self.code,         // Code from TxDataDeploy
//             self.code_metadata, // Code metadata from TxDataDeploy
//             &self.arg_buffer,   // Arg buffer from TxDataDeploy
//         )
//     }
// }

pub struct TxDataDeploy<Env>
where
    Env: TxEnv,
{
    pub code: ManagedBuffer<Env::Api>,
    pub metadata: CodeMetadata,
    pub arg_buffer: ManagedArgBuffer<Env::Api>,
}

impl<Env> TxDataDeploy<Env>
where
    Env: TxEnv,
{
    pub fn new(
        code: ManagedBuffer<Env::Api>,
        metadata: CodeMetadata,
        arg_buffer: ManagedArgBuffer<Env::Api>,
    ) -> Self {
        TxDataDeploy {
            code,
            metadata,
            arg_buffer,
        }
    }
}

impl<Env> TxData<Env> for TxDataDeploy<Env>
where
    Env: TxEnv,
{
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

    fn get_code(&self) -> ManagedBuffer<Env::Api> {
        self.code
    }

    fn get_metadata(&self) -> CodeMetadata {
        self.metadata
    }

    fn get_args(&self) -> ManagedArgBuffer<Env::Api> {
        self.arg_buffer
    }
}

pub trait TxData<Env>
where
    Env: TxEnv,
{
    fn is_no_call(&self) -> bool;

    fn to_call_data_string(&self) -> ManagedBuffer<Env::Api>;

    fn get_code(&self) -> ManagedBuffer<Env::Api>;

    fn get_metadata(&self) -> CodeMetadata;

    fn get_args(&self) -> ManagedArgBuffer<Env::Api>;
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

    fn get_code(&self) -> ManagedBuffer<Env::Api> {
        ManagedBuffer::new()
    }

    fn get_metadata(&self) -> CodeMetadata {
        CodeMetadata::default()
    }

    fn get_args(&self) -> ManagedArgBuffer<Env::Api> {
        ManagedArgBuffer {
            data: ManagedVec::new(),
        }
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

    fn get_code(&self) -> ManagedBuffer<Env::Api> {
        ManagedBuffer::new()
    }

    fn get_metadata(&self) -> CodeMetadata {
        CodeMetadata::default()
    }

    fn get_args(&self) -> ManagedArgBuffer<Env::Api> {
        ManagedArgBuffer {
            data: ManagedVec::new(),
        }
    }
}
impl<Env> TxDataFunctionCall<Env> for FunctionCall<Env::Api> where Env: TxEnv {}
