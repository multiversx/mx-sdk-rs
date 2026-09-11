use super::{
    DeployCall, FunctionCall, OriginalResultMarker, Tx, TxEnv, TxFrom, TxGas, TxTo, UpgradeCall,
};

/// Defines a proxy object for a smart contract.
///
/// Framework-level version, with explicit `Env`, `From`, `To`, `Gas` type parameters.
/// Used by generated proxies. For the abi-level version that abstracts away `Tx`,
/// see `multiversx_sc_abi::AbiProxyTrait`.
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
///
/// Replaced by `TxTypedDeploy`.
pub type TxProxyDeploy<Env, From, Gas, Original> =
    Tx<Env, From, (), (), Gas, DeployCall<Env, ()>, OriginalResultMarker<Original>>;

/// Alias for a `Tx` generated from a proxy, in `init`.
pub type TxTypedDeploy<Env, From, Payment, Gas, Original> =
    Tx<Env, From, (), Payment, Gas, DeployCall<Env, ()>, OriginalResultMarker<Original>>;

/// Alias for a `Tx` generated from a proxy, in an endpoint.
///
/// Replaced by `TxTypedCall`.
pub type TxProxyCall<Env, From, To, Gas, Original> =
    Tx<Env, From, To, (), Gas, FunctionCall<<Env as TxEnv>::Api>, OriginalResultMarker<Original>>;

/// Alias for a `Tx` generated from a proxy, in an endpoint.
pub type TxTypedCall<Env, From, To, Payment, Gas, Original> = Tx<
    Env,
    From,
    To,
    Payment,
    Gas,
    FunctionCall<<Env as TxEnv>::Api>,
    OriginalResultMarker<Original>,
>;

/// Alias for a `Tx` generated from a proxy, in `upgrade`.
///
/// Replaced by `TxTypedUpgrade`.
pub type TxProxyUpgrade<Env, From, To, Gas, Original> =
    Tx<Env, From, To, (), Gas, UpgradeCall<Env, ()>, OriginalResultMarker<Original>>;

/// Alias for a `Tx` generated from a proxy, in `upgrade`.
pub type TxTypedUpgrade<Env, From, To, Payment, Gas, Original> =
    Tx<Env, From, To, Payment, Gas, UpgradeCall<Env, ()>, OriginalResultMarker<Original>>;
