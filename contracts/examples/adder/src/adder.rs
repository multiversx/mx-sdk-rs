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

trait TxProxyMethodsTrait<Env: TxEnv + multiversx_sc::api::CallTypeApi> {
    fn add<Arg0, O>(
        self,
        value: Arg0,
    ) -> multiversx_sc::types::Tx<
        Env,
        (),
        (),
        (),
        (),
        FunctionCall<Env::Api>,
        OriginalResultMarker<O>,
    >
    where
        Arg0: multiversx_sc::codec::CodecInto<BigUint<Env>>;

    fn upgrade<Arg0, O>(
        self,
        initial_value: Arg0,
    ) -> multiversx_sc::types::Tx<
        Env,
        (),
        (),
        (),
        (),
        FunctionCall<Env::Api>,
        OriginalResultMarker<O>,
    >
    where
        Arg0: multiversx_sc::codec::CodecInto<BigUint<Env>>;

    fn sum<O>(
        self,
    ) -> multiversx_sc::types::Tx<
        Env,
        (),
        (),
        (),
        (),
        FunctionCall<Env::Api>,
        OriginalResultMarker<O>,
    >;

    fn init<Arg0, O>(
        self,
        initial_value: Arg0,
    ) -> multiversx_sc::types::Tx<Env, (), (), (), (), DeployCall<TxScEnv, FromSource<TxScEnv<Api>>>, OriginalResultMarker<O>>
    where
        Arg0: multiversx_sc::codec::CodecInto<BigUint<Env>>;
}

impl<Env: TxEnv + multiversx_sc::api::CallTypeApi> TxProxyMethodsTrait<Env>
    for TxProxyMethods<Env>
{
    fn init<Arg0: multiversx_sc::codec::CodecInto<BigUint<Env>>, O>(
        self,
        initial_value: Arg0,
    ) -> multiversx_sc::types::Tx<
        Env,
        (),
        (),
        (),
        (),
        FunctionCall<Env::Api>,
        OriginalResultMarker<O>,
    > {
        Tx::new_with_env(self.env)
            .raw_call()
            .function_name("init")
            .argument(&initial_value)
            .original_result()
    }

    fn sum<O>(
        self,
    ) -> multiversx_sc::types::Tx<
        Env,
        (),
        (),
        (),
        (),
        FunctionCall<Env::Api>,
        OriginalResultMarker<O>,
    > {
        Tx::new_with_env(self.env)
            .raw_call()
            .function_name("getSum")
            .original_result()
    }

    fn upgrade<Arg0: multiversx_sc::codec::CodecInto<BigUint<Env>>, O>(
        self,
        initial_value: Arg0,
    ) -> multiversx_sc::types::Tx<
        Env,
        (),
        (),
        (),
        (),
        FunctionCall<Env::Api>,
        OriginalResultMarker<O>,
    > {
        Tx::new_with_env(self.env.clone())
            .raw_call()
            .function_name("upgrade")
            .argument(&initial_value)
            .original_result()
    }

    //Add desired amount to the storage variable.
    fn add<Arg0: multiversx_sc::codec::CodecInto<BigUint<Env>>, O>(
        self,
        value: Arg0,
    ) -> multiversx_sc::types::Tx<
        Env,
        (),
        (),
        (),
        (),
        FunctionCall<Env::Api>,
        OriginalResultMarker<O>,
    > {
        Tx::new_with_env(self.env.clone())
            .raw_call()
            .function_name("add")
            .argument(&value)
            .original_result()
    }
}
