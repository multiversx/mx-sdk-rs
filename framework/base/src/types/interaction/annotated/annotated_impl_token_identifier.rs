use crate::{
    proxy_imports::ManagedBufferBuilder,
    types::{ManagedBuffer, TokenIdentifier},
};

use super::{AnnotatedValue, TxEnv};

impl<Env> AnnotatedValue<Env, TokenIdentifier<Env::Api>> for TokenIdentifier<Env::Api>
where
    Env: TxEnv,
{
    fn annotation(&self, env: &Env) -> ManagedBuffer<Env::Api> {
        (&self).annotation(env)
    }

    fn to_value(&self, _env: &Env) -> TokenIdentifier<Env::Api> {
        self.clone()
    }

    fn into_value(self, _env: &Env) -> TokenIdentifier<Env::Api> {
        self
    }

    fn with_value_ref<F, R>(&self, _env: &Env, f: F) -> R
    where
        F: FnOnce(&TokenIdentifier<Env::Api>) -> R,
    {
        f(self)
    }
}

impl<Env> AnnotatedValue<Env, TokenIdentifier<Env::Api>> for &TokenIdentifier<Env::Api>
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        let mut annot = ManagedBufferBuilder::<Env::Api>::new_from_slice("str:".as_bytes());
        annot.append_managed_buffer(self.as_managed_buffer());
        annot.into_managed_buffer()
    }

    fn to_value(&self, _env: &Env) -> TokenIdentifier<Env::Api> {
        (*self).clone()
    }

    fn into_value(self, _env: &Env) -> TokenIdentifier<Env::Api> {
        (*self).clone()
    }

    fn with_value_ref<F, R>(&self, _env: &Env, f: F) -> R
    where
        F: FnOnce(&TokenIdentifier<Env::Api>) -> R,
    {
        f(self)
    }
}
