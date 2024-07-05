use super::{AnnotatedValue, TxEnv};
use crate::{
    api::{BlockchainApi, BlockchainApiImpl},
    types::ManagedBuffer,
};

/// All typed that populate the gas field of a transaction need to implement this trait.
pub trait TxGas<Env>
where
    Env: TxEnv,
{
    fn gas_annotation(&self, env: &Env) -> ManagedBuffer<Env::Api>;

    fn gas_value(&self, env: &Env) -> u64;

    fn explicit_or_gas_left(&self, env: &Env) -> u64;
}

impl<Env> TxGas<Env> for ()
where
    Env: TxEnv,
{
    fn gas_annotation(&self, env: &Env) -> ManagedBuffer<<Env as TxEnv>::Api> {
        env.default_gas_annotation()
    }

    fn gas_value(&self, env: &Env) -> u64 {
        env.default_gas_value()
    }

    fn explicit_or_gas_left(&self, _env: &Env) -> u64 {
        Env::Api::blockchain_api_impl().get_gas_left()
    }
}

#[diagnostic::on_unimplemented(
    message = "Type `{Self}` cannot be used as gas value (does not implement `TxGasValue<{Env}>`)",
    label = "not a valid value for gas",
    note = "there are multiple ways to specify the gas value for a transaction, but `{Self}` is not one of them"
)]
pub trait TxGasValue<Env>: AnnotatedValue<Env, u64>
where
    Env: TxEnv,
{
}

impl<Env> TxGasValue<Env> for u64 where Env: TxEnv {}

pub struct ExplicitGas<GasValue>(pub GasValue);

impl<Env, GasValue> TxGas<Env> for ExplicitGas<GasValue>
where
    Env: TxEnv,
    GasValue: TxGasValue<Env>,
{
    fn gas_value(&self, env: &Env) -> u64 {
        self.0.to_value(env)
    }

    fn gas_annotation(&self, env: &Env) -> ManagedBuffer<<Env as TxEnv>::Api> {
        self.0.annotation(env)
    }

    fn explicit_or_gas_left(&self, env: &Env) -> u64 {
        self.gas_value(env)
    }
}
