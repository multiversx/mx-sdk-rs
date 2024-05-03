use crate::types::ManagedBuffer;

use super::{AnnotatedValue, TxEnv};

impl<Env> AnnotatedValue<Env, ManagedBuffer<Env::Api>> for ManagedBuffer<Env::Api>
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.hex_expr()
    }

    fn to_value(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.clone()
    }

    fn into_value(self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self
    }

    fn with_value_ref<F, R>(&self, _env: &Env, f: F) -> R
    where
        F: FnOnce(&ManagedBuffer<Env::Api>) -> R,
    {
        f(self)
    }
}

impl<Env> AnnotatedValue<Env, ManagedBuffer<Env::Api>> for ()
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        ManagedBuffer::new()
    }

    fn to_value(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        ManagedBuffer::new()
    }
}
