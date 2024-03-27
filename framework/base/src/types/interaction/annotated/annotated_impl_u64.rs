use crate::{
    formatter::{FormatBuffer, SCDisplay},
    proxy_imports::ManagedTypeApi,
    types::{
        heap::Address, BigUint, ManagedAddress, ManagedBuffer, ManagedBufferCachedBuilder,
        ManagedRef,
    },
};

use super::{AnnotatedValue, TxEnv};

pub(crate) fn display_u64<Api>(n: u64) -> ManagedBuffer<Api>
where
    Api: ManagedTypeApi,
{
    let mut result = ManagedBufferCachedBuilder::new_from_slice(&[]);
    result.append_display(&n);
    result.into_managed_buffer()
}

impl<Env> AnnotatedValue<Env, u64> for u64
where
    Env: TxEnv,
{
    fn annotation(&self, env: &Env) -> ManagedBuffer<Env::Api> {
        display_u64(*self)
    }

    fn to_value(&self, _env: &Env) -> u64 {
        *self
    }
}

impl<Env> AnnotatedValue<Env, u64> for i32
where
    Env: TxEnv,
{
    fn annotation(&self, env: &Env) -> ManagedBuffer<Env::Api> {
        display_u64(*self as u64)
    }

    fn to_value(&self, _env: &Env) -> u64 {
        *self as u64
    }
}
