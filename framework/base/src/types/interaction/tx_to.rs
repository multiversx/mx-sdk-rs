use crate::types::{heap::Address, ManagedAddress};

use super::{AnnotatedValue, TxEnv};

/// Marks the recipient of any transaction.
pub trait TxTo<Env>
where
    Env: TxEnv,
{
}

impl<Env> TxTo<Env> for () where Env: TxEnv {}

/// Marks the non-empty recipient of a transaction.
///
/// Enforces the reciipent to be explicitly specified.
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

impl<Env> TxTo<Env> for ManagedAddress<Env::Api> where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for ManagedAddress<Env::Api> where Env: TxEnv {}

impl<Env> TxTo<Env> for &ManagedAddress<Env::Api> where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for &ManagedAddress<Env::Api> where Env: TxEnv {}

impl<Env> TxTo<Env> for Address where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for Address where Env: TxEnv {}

impl<Env> TxTo<Env> for &Address where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for &Address where Env: TxEnv {}
