// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct AdderProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for AdderProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = AdderProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        AdderProxyMethods { wrapped_tx: tx }
    }
}

pub struct AdderProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> AdderProxyMethods<Env, From, (), Gas>
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
    ) -> TxProxyDeploy<Env, From, Gas, ()> {
        self.wrapped_tx
            .raw_deploy()
            .argument(&initial_value)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> AdderProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn upgrade<
        Arg0: CodecInto<BigUint<Env::Api>>,
    >(
        self,
        initial_value: Arg0,
    ) -> TxProxyUpgrade<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_upgrade()
            .argument(&initial_value)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> AdderProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn sum(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .raw_call("getSum")
            .original_result()
    }

    /// Add desired amount to the storage variable. 
    pub fn add<
        Arg0: CodecInto<BigUint<Env::Api>>,
    >(
        self,
        value: Arg0,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("add")
            .argument(&value)
            .original_result()
    }
}
