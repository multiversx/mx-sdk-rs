use crate::types::ManagedBuffer;

use super::{AnnotatedValue, TxEnv, display_u64};

impl<Env> AnnotatedValue<Env, u64> for u64
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
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
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        display_u64(*self as u64)
    }

    fn to_value(&self, _env: &Env) -> u64 {
        *self as u64
    }
}
