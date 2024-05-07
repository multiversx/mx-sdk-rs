use crate::types::{heap::Address, ManagedAddress};

use super::{AnnotatedValue, TxEnv};

/// Marks the sender of any transaction.
pub trait TxFrom<Env>
where
    Env: TxEnv,
{
    fn resolve_address(&self, env: &Env) -> ManagedAddress<Env::Api>;
}

/// Marks the non-empty sender of a transaction.
///
/// Enforces the reciipent to be explicitly specified.
pub trait TxFromSpecified<Env>:
    TxFrom<Env> + AnnotatedValue<Env, ManagedAddress<Env::Api>>
where
    Env: TxEnv,
{
}

impl<Env> TxFrom<Env> for ()
where
    Env: TxEnv,
{
    fn resolve_address(&self, env: &Env) -> ManagedAddress<Env::Api> {
        env.resolve_sender_address()
    }
}

impl<Env> TxFrom<Env> for ManagedAddress<Env::Api>
where
    Env: TxEnv,
{
    fn resolve_address(&self, _env: &Env) -> ManagedAddress<Env::Api> {
        self.clone()
    }
}
impl<Env> TxFromSpecified<Env> for ManagedAddress<Env::Api> where Env: TxEnv {}

impl<Env> TxFrom<Env> for &ManagedAddress<Env::Api>
where
    Env: TxEnv,
{
    fn resolve_address(&self, _env: &Env) -> ManagedAddress<Env::Api> {
        (*self).clone()
    }
}
impl<Env> TxFromSpecified<Env> for &ManagedAddress<Env::Api> where Env: TxEnv {}

impl<Env> TxFrom<Env> for Address
where
    Env: TxEnv,
{
    fn resolve_address(&self, _env: &Env) -> ManagedAddress<Env::Api> {
        self.into()
    }
}

impl<Env> TxFromSpecified<Env> for Address where Env: TxEnv {}

impl<Env> TxFrom<Env> for &Address
where
    Env: TxEnv,
{
    fn resolve_address(&self, _env: &Env) -> ManagedAddress<Env::Api> {
        ManagedAddress::from_address(self)
    }
}

impl<Env> TxFromSpecified<Env> for &Address where Env: TxEnv {}
