use crate::types::{heap::Address, ManagedAddress, ManagedBuffer};

use super::TxEnv;

pub trait AnnotatedValue<Env, T>
where
    Env: TxEnv,
{
    fn annotation(&self, env: &Env) -> ManagedBuffer<Env::Api>;

    fn into_value(self, env: &Env) -> T;
}

impl<Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for ManagedAddress<Env::Api>
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.hex_expr()
    }

    fn into_value(self, _env: &Env) -> ManagedAddress<Env::Api> {
        self
    }
}

impl<Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for &ManagedAddress<Env::Api>
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> crate::types::ManagedBuffer<Env::Api> {
        self.hex_expr()
    }

    fn into_value(self, _env: &Env) -> ManagedAddress<Env::Api> {
        self.clone()
    }
}

impl<Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for Address
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        ManagedAddress::from(self).hex_expr()
    }

    fn into_value(self, _env: &Env) -> ManagedAddress<Env::Api> {
        self.into()
    }
}

impl<Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for &Address
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        ManagedAddress::from(*self).hex_expr()
    }

    fn into_value(self, _env: &Env) -> ManagedAddress<Env::Api> {
        self.into()
    }
}

impl<Env> AnnotatedValue<Env, ManagedBuffer<Env::Api>> for ManagedBuffer<Env::Api>
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> crate::types::ManagedBuffer<Env::Api> {
        self.hex_expr()
    }

    fn into_value(self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self
    }
}
