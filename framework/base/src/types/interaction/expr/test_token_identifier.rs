use multiversx_sc_codec::{EncodeErrorHandler, TopEncode, TopEncodeOutput};

use crate::{
    abi::TypeAbiFrom,
    api::ManagedTypeApi,
    types::{AnnotatedValue, ManagedBuffer, TokenIdentifier, TxEnv},
};

const STR_PREFIX: &str = "str:";

/// Encodes a dummy address, to be used for tests.
///
/// It is designed to be usable from contracts (especiall test contracts), with a minimal footprint.
/// For this reason, its inner structure is subject to change.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TestTokenIdentifier<'a> {
    name: &'a str,
}

impl<'a> TestTokenIdentifier<'a> {
    pub const fn new(name: &'a str) -> Self {
        TestTokenIdentifier { name }
    }

    #[cfg(feature = "alloc")]
    pub fn eval_to_expr(&self) -> alloc::string::String {
        alloc::format!("{STR_PREFIX}{}", self.name)
    }

    pub fn to_token_identifier<Api: ManagedTypeApi>(&self) -> TokenIdentifier<Api> {
        self.name.into()
    }

    pub fn as_str(&self) -> &str {
        self.name
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.name.as_bytes()
    }
}

impl<Env> AnnotatedValue<Env, TokenIdentifier<Env::Api>> for TestTokenIdentifier<'_>
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        let mut result = ManagedBuffer::new_from_bytes(STR_PREFIX.as_bytes());
        result.append_bytes(self.name.as_bytes());
        result
    }

    fn to_value(&self, _env: &Env) -> TokenIdentifier<Env::Api> {
        self.name.into()
    }
}

impl<'a, Api> From<TestTokenIdentifier<'a>> for TokenIdentifier<Api>
where
    Api: ManagedTypeApi,
{
    fn from(value: TestTokenIdentifier<'a>) -> Self {
        TokenIdentifier::from_esdt_bytes(value.name)
    }
}

impl TopEncode for TestTokenIdentifier<'_> {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.name.top_encode_or_handle_err(output, h)
    }
}

impl<Api> TypeAbiFrom<TestTokenIdentifier<'_>> for TokenIdentifier<Api> where Api: ManagedTypeApi {}
