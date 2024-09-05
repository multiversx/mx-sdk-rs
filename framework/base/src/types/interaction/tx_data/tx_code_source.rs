use crate::types::{AnnotatedValue, ManagedAddress, ManagedBuffer, TxEnv};

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

#[diagnostic::on_unimplemented(
    message = "Type `{Self}` cannot be used as code (does not implement `TxCodeValue<{Env}>`)",
    label = "not a valid smart contract byte code",
    note = "there are multiple ways to specify SC byte code, but `{Self}` is not one of them"
)]
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

#[diagnostic::on_unimplemented(
    message = "Type `{Self}` cannot be used as code source value (does not implement `TxFromSourceValue<{Env}>`)",
    label = "not an address from where to copy the code",
    note = "there are multiple ways to specify a code source address, but `{Self}` is not one of them"
)]
pub trait TxFromSourceValue<Env>: AnnotatedValue<Env, ManagedAddress<Env::Api>>
where
    Env: TxEnv,
{
}

impl<Env> TxFromSourceValue<Env> for ManagedAddress<Env::Api> where Env: TxEnv {}

/// Indicates the source of a "deploy from source" or "upgrade from source".
pub struct FromSource<FromSourceValue>(pub FromSourceValue);

impl<Env, FromSourceValue> TxCodeSource<Env> for FromSource<FromSourceValue>
where
    Env: TxEnv,
    FromSourceValue: TxFromSourceValue<Env>,
{
}

impl<Env, FromSourceValue> TxCodeSourceSpecified<Env> for FromSource<FromSourceValue>
where
    Env: TxEnv,
    FromSourceValue: TxFromSourceValue<Env>,
{
}
