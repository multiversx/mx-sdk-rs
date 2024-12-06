use multiversx_chain_core::builtin_func_names::ESDT_SET_TOKEN_TYPE_FUNC_NAME;

use crate::types::{
    EsdtTokenType, NotPayable, ProxyArg, TokenIdentifier, Tx, TxEnv, TxFrom, TxGas, TxProxyTrait,
    TxTo, TxTypedCall,
};

/// Proxy for the system smart contract.
pub struct SystemSCProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for SystemSCProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = SystemSCProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        SystemSCProxyMethods { wrapped_tx: tx }
    }
}

/// Method container of the ESDT system smart contract proxy.
pub struct SystemSCProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

impl<Env, From, To, Gas> SystemSCProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    /// Sets the token type for a specific token.
    pub fn esdt_set_token_type<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<EsdtTokenType>,
    >(
        self,
        token_id: Arg0,
        token_type: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        let tx = self
            .wrapped_tx
            .payment(NotPayable)
            .raw_call(ESDT_SET_TOKEN_TYPE_FUNC_NAME)
            .argument(&token_id)
            .argument(&token_type);

        tx.original_result()
    }
}
