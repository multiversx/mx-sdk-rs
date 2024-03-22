use crate::types::{heap::Address, BigUint, ManagedAddress, ManagedBuffer};

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
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
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
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.hex_expr()
    }

    fn into_value(self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self
    }
}

impl<Env> AnnotatedValue<Env, BigUint<Env::Api>> for BigUint<Env::Api>
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.to_display()
    }

    fn into_value(self, _env: &Env) -> BigUint<Env::Api> {
        self
    }
}

impl<Env> AnnotatedValue<Env, BigUint<Env::Api>> for &BigUint<Env::Api>
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.to_display()
    }

    fn into_value(self, _env: &Env) -> BigUint<Env::Api> {
        self.clone()
    }
}

impl<Env> AnnotatedValue<Env, BigUint<Env::Api>> for u64
where
    Env: TxEnv,
{
    fn annotation(&self, env: &Env) -> ManagedBuffer<Env::Api> {
        self.into_value(env).to_display()
    }

    fn into_value(self, _env: &Env) -> BigUint<Env::Api> {
        BigUint::<Env::Api>::from(self)
    }
}
