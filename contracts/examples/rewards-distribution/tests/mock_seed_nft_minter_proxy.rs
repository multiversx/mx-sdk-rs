use multiversx_sc::proxy_imports::*;

pub struct MockSeedNftMinterProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for MockSeedNftMinterProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = MockSeedNftMinterProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        MockSeedNftMinterProxyMethods { wrapped_tx: tx }
    }
}

pub struct MockSeedNftMinterProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> MockSeedNftMinterProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init<
        Arg0: ProxyArg<EsdtTokenIdentifier<Env::Api>>,
    >(
        self,
        nft_token_id: Arg0,
    ) -> TxTypedDeploy<Env, From, (), Gas, ()> {
        self.wrapped_tx
            .raw_deploy()
            .argument(&nft_token_id)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> MockSeedNftMinterProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn set_nft_count<
        Arg0: ProxyArg<u64>,
    >(
        self,
        nft_count: Arg0,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("setNftCount")
            .argument(&nft_count)
            .original_result()
    }
}
