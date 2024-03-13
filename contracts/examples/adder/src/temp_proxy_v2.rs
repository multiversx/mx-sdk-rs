////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(clippy::all)]

use multiversx_sc::imports::*;

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

impl<Env, From, Gas> TxProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init<
        Arg0: CodecInto<BigUint<Env::Api>>,
    >(
        self,
        initial_value: Arg0,
    ) -> Tx<
        Env,
        From,
        (),
        (),
        Gas,
        DeployCall<Env, ()>,
        OriginalResultMarker<()>,
    > {
        self.wrapped_tx
            .raw_deploy()
            .argument(&initial_value)
            .original_result()
    }

}
impl<Env, From, To, Gas> TxProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn sum(
        self,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<multiversx_sc::types::BigUint<Env::Api>>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("getSum")
            .original_result()
    }

    /// Add desired amount to the storage variable. 
    pub fn add<
        Arg0: CodecInto<BigUint<Env::Api>>,
    >(
        self,
        value: Arg0,
    ) -> Tx<
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
