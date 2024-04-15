use crate::{
    api::{BlockchainApi, BlockchainApiImpl},
    types::{interaction::display_u64, AnnotatedValue, ManagedBuffer, TxEnv, TxGasValue},
};

/// Indicates that all remaining gas should be sent to a transaction.
///
/// Usually unwise, other than for synchronous calls, you always want to have some gas left in the contract after the call.
pub struct GasLeft;

impl<Env> AnnotatedValue<Env, u64> for GasLeft
where
    Env: TxEnv,
{
    fn annotation(&self, env: &Env) -> ManagedBuffer<Env::Api> {
        display_u64(self.to_value(env))
    }

    fn to_value(&self, _env: &Env) -> u64 {
        Env::Api::blockchain_api_impl().get_gas_left()
    }
}

impl<Env> TxGasValue<Env> for GasLeft where Env: TxEnv {}
