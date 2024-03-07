use crate::{
    api::ManagedTypeApi,
    types::{ManagedAddress, ManagedBuffer},
};

use super::{AnnotatedValue, TxEnv};

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

pub trait TxCodeValue<Env>: AnnotatedValue<Env, ManagedBuffer<Env::Api>>
where
    Env: TxEnv,
{
}

impl<Env> TxCodeValue<Env> for ManagedBuffer<Env::Api> where Env: TxEnv {}

/// Contains code for a deploy or upgrade.
pub struct Code<CodeValue>(pub CodeValue);

impl<Env, CodeValue> TxCodeSource<Env> for Code<CodeValue>
where
    Env: TxEnv,
    CodeValue: TxCodeValue<Env>,
{
}

impl<Env, CodeValue> TxCodeSourceSpecified<Env> for Code<CodeValue>
where
    Env: TxEnv,
    CodeValue: TxCodeValue<Env>,
{
}

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
