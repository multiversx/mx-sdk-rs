use super::TxEnv;

pub trait TxGas<Env>
where
    Env: TxEnv,
{
    fn resolve_gas(&self, env: &Env) -> u64;
}

impl<Env> TxGas<Env> for ()
where
    Env: TxEnv,
{
    fn resolve_gas(&self, env: &Env) -> u64 {
        env.default_gas()
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
}
