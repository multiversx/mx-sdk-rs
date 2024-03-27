use crate::{
    proxy_imports::ManagedRef,
    types::{heap::Address, BigUint, ManagedAddress, ManagedBuffer},
};

use super::TxEnv;

pub trait AnnotatedValue<Env, T>: Sized
where
    Env: TxEnv,
{
    fn annotation(&self, env: &Env) -> ManagedBuffer<Env::Api>;

    /// Produces the value from a reference of the annotated type. Might involve a `.clone()` in some cases.
    fn to_value(&self, env: &Env) -> T;

    /// Consumes annotated value to produce actual value.
    ///
    /// Override whenever it helps to avoid an unnecessary clone.
    fn into_value(self, env: &Env) -> T {
        self.to_value(env)
    }

    /// Can be used when working with references only.
    ///
    /// Override whenever it helps to avoid an unnecessary clone.
    fn with_value_ref<F, R>(&self, env: &Env, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        f(&self.to_value(env))
    }
}

impl<Env> AnnotatedValue<Env, ManagedAddress<Env::Api>> for ManagedAddress<Env::Api>
where
    Env: TxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.hex_expr()
    }

    fn to_value(&self, env: &Env) -> ManagedAddress<Env::Api> {
        self.clone()
    }

    fn into_value(self, _env: &Env) -> ManagedAddress<Env::Api> {
        self
    }

    fn with_value_ref<F, R>(&self, env: &Env, f: F) -> R
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

    fn to_value(&self, env: &Env) -> ManagedAddress<Env::Api> {
        (*self).clone()
    }

    fn into_value(self, _env: &Env) -> ManagedAddress<Env::Api> {
        self.clone()
    }

    fn with_value_ref<F, R>(&self, env: &Env, f: F) -> R
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

    fn to_value(&self, env: &Env) -> ManagedAddress<Env::Api> {
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

    fn to_value(&self, env: &Env) -> ManagedAddress<Env::Api> {
        ManagedAddress::from(*self)
    }
}

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

    fn with_value_ref<F, R>(&self, env: &Env, f: F) -> R
    where
        F: FnOnce(&ManagedBuffer<Env::Api>) -> R,
    {
        f(self)
    }
}

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

    fn with_value_ref<F, R>(&self, env: &Env, f: F) -> R
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

    fn into_value(self, _env: &Env) -> BigUint<Env::Api> {
        self.clone()
    }

    fn with_value_ref<F, R>(&self, env: &Env, f: F) -> R
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

    fn to_value(&self, _env: &Env) -> BigUint<Env::Api> {
        (*self).clone_value()
    }

    fn into_value(self, _env: &Env) -> BigUint<Env::Api> {
        self.clone_value()
    }

    fn with_value_ref<F, R>(&self, env: &Env, f: F) -> R
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
    fn annotation(&self, env: &Env) -> ManagedBuffer<Env::Api> {
        self.to_value(env).to_display()
    }

    fn to_value(&self, _env: &Env) -> BigUint<Env::Api> {
        BigUint::from(*self)
    }
}

impl<Env> AnnotatedValue<Env, BigUint<Env::Api>> for ()
where
    Env: TxEnv,
{
    fn annotation(&self, env: &Env) -> ManagedBuffer<Env::Api> {
        ManagedBuffer::from("0")
    }

    fn to_value(&self, _env: &Env) -> BigUint<Env::Api> {
        BigUint::zero()
    }
}
