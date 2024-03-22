use crate::types::{heap::Address, ManagedAddress};

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
        F: FnOnce(&ManagedAddress<Env::Api>) -> R;
}

impl<Env> TxTo<Env> for ManagedAddress<Env::Api> where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for ManagedAddress<Env::Api>
where
    Env: TxEnv,
{
    fn with_address_ref<F, R>(&self, _env: &Env, f: F) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>) -> R,
    {
        f(self)
    }
}

impl<Env> TxTo<Env> for &ManagedAddress<Env::Api> where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for &ManagedAddress<Env::Api>
where
    Env: TxEnv,
{
    fn with_address_ref<F, R>(&self, _env: &Env, f: F) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>) -> R,
    {
        f(self)
    }
}

impl<Env> TxTo<Env> for Address where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for Address
where
    Env: TxEnv,
{
    fn with_address_ref<F, R>(&self, _env: &Env, f: F) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>) -> R,
    {
        let managed_address = ManagedAddress::from(self);
        f(&managed_address)
    }
}

impl<Env> TxTo<Env> for &Address where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for &Address
where
    Env: TxEnv,
{
    fn with_address_ref<F, R>(&self, _env: &Env, f: F) -> R
    where
        F: FnOnce(&ManagedAddress<Env::Api>) -> R,
    {
        let managed_address = ManagedAddress::from(*self);
        f(&managed_address)
    }
}
