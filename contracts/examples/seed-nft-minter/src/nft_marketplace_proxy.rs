#![allow(clippy::all)]

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
impl<Env, From, Gas> NftMarketplaceProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init(
        self,
    ) -> TxProxyDeploy<Env, From, Gas, ()> {
        self.wrapped_tx
            .raw_deploy()
            .original_result()
    }
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
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<EgldOrEsdtTokenIdentifier<Env::Api>>,
        Arg2: CodecInto<u64>,
    >(
        self,
        claim_destination: Arg0,
        token_id: Arg1,
        token_nonce: Arg2,
    ) -> TxProxyCall<Env, From, To, Gas, MultiValue2<BigUint<Env::Api>, ManagedVec<Env::Api, EsdtTokenPayment<Env::Api>>>> {
        self.wrapped_tx
            .raw_call()
            .function_name("claimTokens")
            .argument(&claim_destination)
            .argument(&token_id)
            .argument(&token_nonce)
            .original_result()
    }
}
