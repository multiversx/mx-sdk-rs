use super::{Tx, TxEnv, TxFrom, TxGas, TxTo};

/// Defines a proxy object for a smart contract.
pub trait TxProxyTrait<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods;

    /// Creates the associated type that contains the proxy methods implementations.
    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods;
}
