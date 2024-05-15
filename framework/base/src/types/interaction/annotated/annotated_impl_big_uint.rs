use crate::types::{BigUint, ManagedBuffer, ManagedRef, NotPayable};

use super::{AnnotatedValue, TxEnv};

impl<Env> AnnotatedValue<Env, BigUint<Env::Api>> for BigUint<Env::Api>
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.to_display()
    }

    fn to_value(&self, _env: &Env) -> BigUint<Env::Api> {
        self.clone()
    }

    fn into_value(self, _env: &Env) -> BigUint<Env::Api> {
        self
    }

    #[inline]
    fn with_value_ref<F, R>(&self, _env: &Env, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        f(self)
    }
}

impl<Env> AnnotatedValue<Env, BigUint<Env::Api>> for &BigUint<Env::Api>
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.to_display()
    }

    fn to_value(&self, _env: &Env) -> BigUint<Env::Api> {
        (*self).clone()
    }

    #[inline]
    fn into_value(self, _env: &Env) -> BigUint<Env::Api> {
        self.clone()
    }

    #[inline]
    fn with_value_ref<F, R>(&self, _env: &Env, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        f(self)
    }
}

impl<'a, Env> AnnotatedValue<Env, BigUint<Env::Api>> for ManagedRef<'a, Env::Api, BigUint<Env::Api>>
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.to_display()
    }

    #[inline]
    fn to_value(&self, _env: &Env) -> BigUint<Env::Api> {
        (*self).clone_value()
    }

    fn into_value(self, _env: &Env) -> BigUint<Env::Api> {
        self.clone_value()
    }

    #[inline]
    fn with_value_ref<F, R>(&self, _env: &Env, f: F) -> R
    where
        F: FnOnce(&BigUint<Env::Api>) -> R,
    {
        f(self)
    }
}

impl<Env> AnnotatedValue<Env, BigUint<Env::Api>> for u64
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        BigUint::from(*self).to_display()
    }

    fn to_value(&self, _env: &Env) -> BigUint<Env::Api> {
        BigUint::from(*self)
    }
}

impl<Env> AnnotatedValue<Env, BigUint<Env::Api>> for i32
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        BigUint::from(*self as u64).to_display()
    }

    fn to_value(&self, _env: &Env) -> BigUint<Env::Api> {
        BigUint::from(*self as u64)
    }
}

impl<Env> AnnotatedValue<Env, BigUint<Env::Api>> for ()
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        ManagedBuffer::from("0")
    }

    fn to_value(&self, _env: &Env) -> BigUint<Env::Api> {
        BigUint::zero()
    }
}

impl<Env> AnnotatedValue<Env, BigUint<Env::Api>> for NotPayable
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        ManagedBuffer::from("0")
    }

    fn to_value(&self, _env: &Env) -> BigUint<Env::Api> {
        BigUint::zero()
    }
}
