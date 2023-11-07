use crate::{api::ManagedTypeApi, types::ManagedAddress};

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
}

impl<Env> TxTo<Env> for ManagedAddress<Env::Api> where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for ManagedAddress<Env::Api> where Env: TxEnv {}

impl<Env> TxTo<Env> for &ManagedAddress<Env::Api> where Env: TxEnv {}
impl<Env> TxToSpecified<Env> for &ManagedAddress<Env::Api> where Env: TxEnv {}
