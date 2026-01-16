use multiversx_chain_core::types::{TimestampMillis, TimestampSeconds};

use crate::types::ManagedBuffer;

use super::{AnnotatedValue, TxEnv, display_u64};

impl<Env> AnnotatedValue<Env, TimestampMillis> for u64
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        display_u64(*self)
    }

    fn to_value(&self, _env: &Env) -> TimestampMillis {
        TimestampMillis::new(*self)
    }
}

impl<Env> AnnotatedValue<Env, TimestampMillis> for TimestampMillis
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        display_u64(self.as_u64_millis())
    }

    fn to_value(&self, _env: &Env) -> TimestampMillis {
        *self
    }
}

impl<Env> AnnotatedValue<Env, TimestampSeconds> for u64
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        display_u64(*self)
    }

    fn to_value(&self, _env: &Env) -> TimestampSeconds {
        TimestampSeconds::new(*self)
    }
}

impl<Env> AnnotatedValue<Env, TimestampSeconds> for TimestampSeconds
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        display_u64(self.as_u64_seconds())
    }

    fn to_value(&self, _env: &Env) -> TimestampSeconds {
        *self
    }
}
