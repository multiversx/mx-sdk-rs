use multiversx_sc_codec::TopEncodeMulti;

use crate::types::{CodeMetadata, ManagedBuffer, ManagedBufferCachedBuilder};

use super::{FunctionCall, ManagedArgBuffer, Tx, TxData, TxEnv, TxFrom, TxGas, TxPayment, TxTo};

/// Holds deploy data: code, code metadata, and arguments.
pub struct DeployCall<Env>
where
    Env: TxEnv,
{
    pub code: ManagedBuffer<Env::Api>,
    pub code_metadata: CodeMetadata,
    pub arg_buffer: ManagedArgBuffer<Env::Api>,
}

impl<Env> Default for DeployCall<Env>
where
    Env: TxEnv,
{
    fn default() -> DeployCall<Env> {
        DeployCall {
            code: ManagedBuffer::new(),
            code_metadata: CodeMetadata::DEFAULT,
            arg_buffer: ManagedArgBuffer::new(),
        }
    }
}

impl<Env> TxData<Env> for DeployCall<Env>
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
}

impl<Env> DeployCall<Env>
where
    Env: TxEnv,
{
    /// Adds an argument of any serializable type.
    ///
    /// Multi-values are accepted. No type checking performed.
    pub fn argument<T: TopEncodeMulti>(mut self, arg: &T) -> Self {
        self.arg_buffer.push_multi_arg(arg);
        self
    }

    pub fn arguments_raw(mut self, raw: ManagedArgBuffer<Env::Api>) -> Self {
        self.arg_buffer = raw;
        self
    }
}
