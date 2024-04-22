use crate::{
    proxy_imports::ManagedRef,
    types::{heap::Address, ManagedAddress},
};

use super::{AnnotatedValue, TxEnv};

pub trait TxTo<Env>
where
    Env: TxEnv,
{
}

impl<Env> TxTo<Env> for () where Env: TxEnv {}

pub trait TxToSpecified<Env>: TxTo<Env> + AnnotatedValue<Env, ManagedAddress<Env::Api>>
where
    Env: TxEnv,
{
    /// Avoids a clone when performing transfer-execute.
    ///
    /// Other than that, does thesame as `AnnotatedValue::into_value`.
    fn with_address_ref<F, R>(&self, env: &Env, f: F) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>) -> R,
    {
        self.with_value_ref(env, f)
    }
}

pub trait TxToInto<Env>
where
    Env: TxEnv,
{
    type Into: TxToSpecified<Env>;

    fn into_recipient(self) -> Self::Into;
}

impl<Env> TxTo<Env> for ManagedAddress<Env::Api> where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for ManagedAddress<Env::Api> where Env: TxEnv {}
impl<Env> TxToInto<Env> for ManagedAddress<Env::Api>
where
    Env: TxEnv,
{
    type Into = Self;

    fn into_recipient(self) -> Self::Into {
        self
    }
}

impl<Env> TxTo<Env> for ManagedRef<'_, Env::Api, ManagedAddress<Env::Api>> where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for ManagedRef<'_, Env::Api, ManagedAddress<Env::Api>> where Env: TxEnv {}
impl<Env> TxToInto<Env> for ManagedRef<'_, Env::Api, ManagedAddress<Env::Api>>
where
    Env: TxEnv,
{
    type Into = Self;

    fn into_recipient(self) -> Self::Into {
        self
    }
}

impl<Env> TxTo<Env> for &ManagedAddress<Env::Api> where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for &ManagedAddress<Env::Api> where Env: TxEnv {}
impl<'a, Env> TxToInto<Env> for &'a ManagedAddress<Env::Api>
where
    Env: TxEnv,
{
    type Into = ManagedRef<'a, Env::Api, ManagedAddress<Env::Api>>;

    fn into_recipient(self) -> Self::Into {
        ManagedRef::from(self)
    }
}

impl<Env> TxTo<Env> for Address where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for Address where Env: TxEnv {}
impl<Env> TxToInto<Env> for Address
where
    Env: TxEnv,
{
    type Into = Self;

    fn into_recipient(self) -> Self::Into {
        self
    }
}

impl<Env> TxTo<Env> for &Address where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for &Address where Env: TxEnv {}
impl<Env> TxToInto<Env> for &Address
where
    Env: TxEnv,
{
    type Into = Self;

    fn into_recipient(self) -> Self::Into {
        self
    }
}
