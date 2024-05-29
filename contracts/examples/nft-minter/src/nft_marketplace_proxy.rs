use multiversx_sc::proxy_imports::*;

pub struct NftMarketplaceProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for NftMarketplaceProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = NftMarketplaceProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        NftMarketplaceProxyMethods { wrapped_tx: tx }
    }
}

pub struct NftMarketplaceProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, To, Gas> NftMarketplaceProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn claim_tokens<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<u64>,
        Arg2: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        token_id: Arg0,
        token_nonce: Arg1,
        claim_destination: Arg2,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("claimTokens")
            .argument(&token_id)
            .argument(&token_nonce)
            .argument(&claim_destination)
            .original_result()
    }
}
