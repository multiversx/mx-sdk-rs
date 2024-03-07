use super::{Tx, TxEnv, TxFrom, TxGas, TxTo};

pub trait TxProxyTrait<Env> {
    type TxProxyMethods;

    fn env(self, env: Env) -> Self::TxProxyMethods;
}

pub trait TxProxyTraitV2<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods;

    fn prepare_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods;
}
