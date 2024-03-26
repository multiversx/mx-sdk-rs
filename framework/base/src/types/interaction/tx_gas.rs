use super::TxEnv;
use crate::api::{BlockchainApi, BlockchainApiImpl};

pub trait TxGas<Env>
where
    Env: TxEnv,
{
    fn resolve_gas(&self, env: &Env) -> u64;

    fn explicit_or_gas_left(&self, env: &Env) -> u64;
}

impl<Env> TxGas<Env> for ()
where
    Env: TxEnv,
{
    fn resolve_gas(&self, env: &Env) -> u64 {
        env.default_gas()
    }

    fn explicit_or_gas_left(&self, _env: &Env) -> u64 {
        Env::Api::blockchain_api_impl().get_gas_left()
    }
}

pub struct ExplicitGas(pub u64);

impl<Env> TxGas<Env> for ExplicitGas
where
    Env: TxEnv,
{
    #[inline]
    fn resolve_gas(&self, _env: &Env) -> u64 {
        self.0
    }

    fn explicit_or_gas_left(&self, _env: &Env) -> u64 {
        self.0
    }
}
