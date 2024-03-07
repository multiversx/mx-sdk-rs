use crate::types::ManagedAddress;

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
    fn with_address_ref<F: FnOnce(&ManagedAddress<Env::Api>)>(&self, env: &Env, f: F);
}

impl<Env> TxTo<Env> for ManagedAddress<Env::Api> where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for ManagedAddress<Env::Api>
where
    Env: TxEnv,
{
    fn with_address_ref<F: FnOnce(&ManagedAddress<Env::Api>)>(&self, _env: &Env, f: F) {
        f(&self)
    }
}

impl<Env> TxTo<Env> for &ManagedAddress<Env::Api> where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for &ManagedAddress<Env::Api>
where
    Env: TxEnv,
{
    fn with_address_ref<F: FnOnce(&ManagedAddress<Env::Api>)>(&self, _env: &Env, f: F) {
        f(self)
    }
}
