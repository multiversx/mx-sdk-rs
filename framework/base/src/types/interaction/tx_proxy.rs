use super::TxEnv;

pub trait TxProxyTrait<Env> {
    type TxProxyMethods;

    fn env(self, env: Env) -> Self::TxProxyMethods;
}
