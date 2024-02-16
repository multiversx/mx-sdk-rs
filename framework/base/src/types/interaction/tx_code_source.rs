use crate::{
    api::ManagedTypeApi,
    types::{ManagedAddress, ManagedBuffer},
};

use super::TxEnv;

pub trait TxCodeSource<Env>
where
    Env: TxEnv,
{
}

impl<Env> TxCodeSource<Env> for () where Env: TxEnv {}

pub trait TxCodeSourceSpecified<Env>: TxCodeSource<Env>
where
    Env: TxEnv,
{
}

/// Contains code for a deploy or upgrade.
pub struct Code<Env>
where
    Env: TxEnv,
{
    pub code: ManagedBuffer<Env::Api>,
}

impl<Env> Code<Env>
where
    Env: TxEnv,
{
    pub fn new(code: ManagedBuffer<Env::Api>) -> Self {
        Code { code }
    }
}

impl<Env> TxCodeSource<Env> for Code<Env> where Env: TxEnv {}

impl<Env> TxCodeSourceSpecified<Env> for Code<Env> where Env: TxEnv {}

/// Indicates the source of a "deploy from source" or "upgrade from source".
pub struct FromSource<Env>
where
    Env: TxEnv,
{
    pub address: ManagedAddress<Env::Api>,
}

impl<Env> FromSource<Env>
where
    Env: TxEnv,
{
    pub fn new(address: ManagedAddress<Env::Api>) -> Self {
        FromSource { address }
    }
}

impl<Env> TxCodeSource<Env> for FromSource<Env> where Env: TxEnv {}

impl<Env> TxCodeSourceSpecified<Env> for FromSource<Env> where Env: TxEnv {}
