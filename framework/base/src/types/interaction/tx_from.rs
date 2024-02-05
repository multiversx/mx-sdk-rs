use crate::{api::CallTypeApi, contract_base::BlockchainWrapper, types::ManagedAddress};

use super::{AnnotatedValue, TxEnv};

pub trait TxFrom<Env>
where
    Env: TxEnv,
{
    fn resolve_address(&self, env: &Env) -> ManagedAddress<Env::Api>;
}

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
