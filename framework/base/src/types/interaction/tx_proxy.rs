use super::{
    DeployCall, FunctionCall, OriginalResultMarker, Tx, TxEnv, TxFrom, TxGas, TxTo, UpgradeCall,
};

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

/// Alias for a `Tx` generated from a proxy, in `init`.
pub type TxProxyDeploy<Env, From, Gas, Original> =
    Tx<Env, From, (), (), Gas, DeployCall<Env, ()>, OriginalResultMarker<Original>>;

/// Alias for a `Tx` generated from a proxy, in an endpoint.
pub type TxProxyCall<Env, From, To, Gas, Original> =
    Tx<Env, From, To, (), Gas, FunctionCall<<Env as TxEnv>::Api>, OriginalResultMarker<Original>>;

/// Alias for a `Tx` generated from a proxy, in `upgrade`.
pub type TxProxyUpgrade<Env, From, To, Gas, Original> =
    Tx<Env, From, To, (), Gas, UpgradeCall<Env, ()>, OriginalResultMarker<Original>>;
