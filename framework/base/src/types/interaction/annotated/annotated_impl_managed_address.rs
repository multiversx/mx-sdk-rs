use crate::types::{ManagedAddress, ManagedBuffer, heap::Address};

use super::{AnnotatedValue, TxEnv};

impl<Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for ManagedAddress<Env::Api>
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.hex_expr()
    }

    fn to_value(&self, _env: &Env) -> ManagedAddress<Env::Api> {
        self.clone()
    }

    fn into_value(self, _env: &Env) -> ManagedAddress<Env::Api> {
        self
    }

    fn with_value_ref<F, R>(&self, _env: &Env, f: F) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>) -> R,
    {
        f(self)
    }
}

impl<Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for &ManagedAddress<Env::Api>
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.hex_expr()
    }

    fn to_value(&self, _env: &Env) -> ManagedAddress<Env::Api> {
        (*self).clone()
    }

    fn into_value(self, _env: &Env) -> ManagedAddress<Env::Api> {
        self.clone()
    }

    fn with_value_ref<F, R>(&self, _env: &Env, f: F) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>) -> R,
    {
        f(self)
    }
}

impl<Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for Address
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        ManagedAddress::from(self).hex_expr()
    }

    fn to_value(&self, _env: &Env) -> ManagedAddress<Env::Api> {
        ManagedAddress::from(self)
    }
}

impl<Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for &Address
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        ManagedAddress::from(*self).hex_expr()
    }

    fn to_value(&self, _env: &Env) -> ManagedAddress<Env::Api> {
        ManagedAddress::from(*self)
    }
}
