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

impl<Env: TxEnv + multiversx_sc::api::CallTypeApi> TxProxyMethods<Env> {
    pub fn add<Arg0: multiversx_sc::codec::CodecInto<BigUint<Env>>>(
        self,
        arg0: Arg0,
    ) -> multiversx_sc::types::Tx<
        Env,
        (),
        (),
        (),
        (),
        FunctionCall<<Env as multiversx_sc::types::TxEnv>::Api>,
        (),
    > {
        Tx::new_with_env(self.env)
            .raw_call()
            .function_name("add")
            .argument(&arg0)
    }
}
