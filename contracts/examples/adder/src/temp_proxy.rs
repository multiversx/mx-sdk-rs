use multiversx_sc::api::VMApi;

multiversx_sc::imports!();

pub struct TxProxy;

impl<Env> TxProxyTrait<Env> for TxProxy
where
    Env: TxEnv,
{
    type TxProxyMethods = TxProxyMethods<Env>;

    fn env(self, env: Env) -> Self::TxProxyMethods {
        TxProxyMethods { env }
    }
}

pub struct TxProxyMethods<Env: TxEnv> {
    pub env: Env,
}

impl<Env> TxProxyMethods<Env>
where
    Env: TxEnv,
    Env::Api: VMApi,
{
    pub fn init<Arg0: multiversx_sc::codec::CodecInto<BigUint<Env::Api>>>(
        self,
        initial_value: Arg0,
    ) -> multiversx_sc::types::Tx<
        Env,
        (),
        (),
        (),
        (),
        DeployCall<Env, ()>,
        OriginalResultMarker<()>,
    > {
        Tx::new_with_env(self.env)
            .raw_deploy()
            .argument(&initial_value)
            .original_result()
    }

    pub fn sum(
        self,
    ) -> multiversx_sc::types::Tx<
        Env,
        (),
        (),
        (),
        (),
        FunctionCall<Env::Api>,
        OriginalResultMarker<SingleValueMapper<Env::Api, multiversx_sc::types::BigUint<Env::Api>>>,
    > {
        Tx::new_with_env(self.env)
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
        (),
        (),
        (),
        (),
        FunctionCall<Env::Api>,
        OriginalResultMarker<()>,
    > {
        Tx::new_with_env(self.env)
            .raw_call()
            .function_name("add")
            .argument(&value)
            .original_result()
    }
}
