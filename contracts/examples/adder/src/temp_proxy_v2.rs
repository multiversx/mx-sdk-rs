use multiversx_sc::api::VMApi;

multiversx_sc::imports!();

pub struct TxProxy;

impl<Env, From, To, Gas> TxProxyTraitV2<Env, From, To, Gas> for TxProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = TxProxyMethods<Env, From, To, Gas>;

    fn prepare_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        TxProxyMethods { wrapped_tx: tx }
    }
}

pub struct TxProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

impl<Env, From, To, Gas> TxProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn init<Arg0: multiversx_sc::codec::CodecInto<BigUint<Env::Api>>>(
        self,
        initial_value: Arg0,
    ) -> multiversx_sc::types::Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<()>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("init")
            .argument(&initial_value)
            .original_result()
    }

    pub fn sum(
        self,
    ) -> multiversx_sc::types::Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<SingleValueMapper<Env::Api, multiversx_sc::types::BigUint<Env::Api>>>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("getSum")
            .original_result()
    }

    //Add desired amount to the storage variable.
    pub fn add<Arg0: multiversx_sc::codec::CodecInto<BigUint<Env::Api>>>(
        self,
        value: Arg0,
    ) -> multiversx_sc::types::Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<()>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("add")
            .argument(&value)
            .original_result()
    }
}
