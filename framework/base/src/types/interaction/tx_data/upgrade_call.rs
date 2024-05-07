use multiversx_sc_codec::TopEncodeMulti;

use crate::types::{
    CodeMetadata, ManagedArgBuffer, ManagedBuffer, ManagedBufferCachedBuilder, TxCodeSource,
    TxData, TxEnv,
};

/// Holds deploy data: code, code metadata, and arguments.
pub struct UpgradeCall<Env, CodeSource>
where
    Env: TxEnv,
    CodeSource: TxCodeSource<Env>,
{
    pub code_source: CodeSource,
    pub code_metadata: CodeMetadata,
    pub arg_buffer: ManagedArgBuffer<Env::Api>,
}

impl<Env> Default for UpgradeCall<Env, ()>
where
    Env: TxEnv,
{
    fn default() -> UpgradeCall<Env, ()> {
        UpgradeCall {
            code_source: (),
            code_metadata: CodeMetadata::DEFAULT,
            arg_buffer: ManagedArgBuffer::new(),
        }
    }
}

impl<Env, CodeSource> TxData<Env> for UpgradeCall<Env, CodeSource>
where
    Env: TxEnv,
    CodeSource: TxCodeSource<Env>,
{
    fn is_no_call(&self) -> bool {
        false
    }

    fn to_call_data_string(&self) -> ManagedBuffer<Env::Api> {
        // Implement as needed for deployment-specific data
        let result = ManagedBufferCachedBuilder::default();
        // result.append_managed_buffer(&self.code);
        // Add other fields as needed
        result.into_managed_buffer()
    }
}

impl<Env> UpgradeCall<Env, ()>
where
    Env: TxEnv,
{
    pub fn code_source<CodeSource>(self, code_source: CodeSource) -> UpgradeCall<Env, CodeSource>
    where
        CodeSource: TxCodeSource<Env>,
    {
        UpgradeCall {
            code_source,
            code_metadata: self.code_metadata,
            arg_buffer: self.arg_buffer,
        }
    }
}

impl<Env, CodeSource> UpgradeCall<Env, CodeSource>
where
    Env: TxEnv,
    CodeSource: TxCodeSource<Env>,
{
    pub fn code_metadata(mut self, code_metadata: CodeMetadata) -> Self
    where
        CodeSource: TxCodeSource<Env>,
    {
        self.code_metadata = code_metadata;
        self
    }

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
