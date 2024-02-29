#![no_std]

multiversx_sc::imports!();
/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[multiversx_sc::contract]
pub trait Adder {
    #[view(getSum)]
    #[storage_mapper("sum")]
    fn sum(&self) -> SingleValueMapper<BigUint>;

    #[init]
    fn init(&self, initial_value: BigUint) {
        self.sum().set(initial_value);
    }

    #[upgrade]
    fn upgrade(&self, initial_value: BigUint) {
        self.init(initial_value);
    }

    /// Add desired amount to the storage variable.
    #[endpoint]
    fn add(&self, value: BigUint) {
        self.sum().update(|sum| *sum += value);
    }
}

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

pub trait TxProxyMethodsTrait<Env: TxEnv + multiversx_sc::api::CallTypeApi> {
    fn add<Arg0, OriginalResult>(
        self,
        value: Arg0,
    ) -> multiversx_sc::types::Tx<
        Env,
        (),
        (),
        (),
        (),
        FunctionCall<Env::Api>,
        OriginalResultMarker<OriginalResult>,
    >
    where
        Arg0: multiversx_sc::codec::CodecInto<BigUint<Env>>;

    fn upgrade<Arg0, OriginalResult>(
        self,
        initial_value: Arg0,
    ) -> multiversx_sc::types::Tx<
        Env,
        (),
        (),
        (),
        (),
        FunctionCall<Env::Api>,
        OriginalResultMarker<OriginalResult>,
    >
    where
        Arg0: multiversx_sc::codec::CodecInto<BigUint<Env>>;

    fn sum<OriginalResult>(
        self,
    ) -> multiversx_sc::types::Tx<
        Env,
        (),
        (),
        (),
        (),
        FunctionCall<Env::Api>,
        OriginalResultMarker<OriginalResult>,
    >;

    #[allow(clippy::type_complexity)]
    fn init<Arg0, OriginalResult>(
        self,
        initial_value: Arg0,
    ) -> multiversx_sc::types::Tx<
        Env,
        (),
        (),
        (),
        (),
        DeployCall<Env::Api, ()>,
        OriginalResultMarker<OriginalResult>,
    >
    where
        Arg0: multiversx_sc::codec::CodecInto<BigUint<Env>>,
        DeployCall<<Env as TxEnv>::Api, ()>: TxData<Env>,
        <Env as TxEnv>::Api: TxEnv;
}

impl<Env: TxEnv<Api = Env> + multiversx_sc::api::CallTypeApi> TxProxyMethodsTrait<Env>
    for TxProxyMethods<Env>
{
    fn sum<OriginalResult>(
        self,
    ) -> multiversx_sc::types::Tx<
        Env,
        (),
        (),
        (),
        (),
        FunctionCall<Env::Api>,
        OriginalResultMarker<OriginalResult>,
    > {
        Tx::new_with_env(self.env)
            .raw_call()
            .function_name("getSum")
            .original_result()
    }

    fn upgrade<Arg0: multiversx_sc::codec::CodecInto<BigUint<Env>>, OriginalResult>(
        self,
        initial_value: Arg0,
    ) -> multiversx_sc::types::Tx<
        Env,
        (),
        (),
        (),
        (),
        FunctionCall<Env::Api>,
        OriginalResultMarker<OriginalResult>,
    > {
        Tx::new_with_env(self.env)
            .raw_call()
            .function_name("upgrade")
            .argument(&initial_value)
            .original_result()
    }

    //Add desired amount to the storage variable.
    fn add<Arg0: multiversx_sc::codec::CodecInto<BigUint<Env>>, OriginalResult>(
        self,
        value: Arg0,
    ) -> multiversx_sc::types::Tx<
        Env,
        (),
        (),
        (),
        (),
        FunctionCall<Env::Api>,
        OriginalResultMarker<OriginalResult>,
    > {
        Tx::new_with_env(self.env)
            .raw_call()
            .function_name("add")
            .argument(&value)
            .original_result()
    }

    fn init<Arg0, OriginalResult>(
        self,
        initial_value: Arg0,
    ) -> multiversx_sc::types::Tx<
        Env,
        (),
        (),
        (),
        (),
        DeployCall<<Env as TxEnv>::Api, ()>,
        OriginalResultMarker<OriginalResult>,
    >
    where
        Arg0: multiversx_sc::codec::CodecInto<BigUint<Env>>,
        DeployCall<<Env as TxEnv>::Api, ()>: TxData<Env>,
        <Env as TxEnv>::Api: TxEnv,
    {
        Tx::new_with_env(self.env)
            .raw_deploy()
            .argument(&initial_value)
            .original_result()
    }
}
