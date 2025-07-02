use crate::types::{
    BigUint, EgldPayment, ManagedAddress, ManagedBuffer, NotPayable, ProxyArg, Tx, TxEnv, TxFrom, TxGas, TxProxyTrait, TxTo, TxTypedCall
};

/// Proxy for the Validator smart contract.
pub struct ValidatorSCProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for ValidatorSCProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = ValidatorSCProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        ValidatorSCProxyMethods { wrapped_tx: tx }
    }
}

#[allow(dead_code)]
/// Method container of the validator smart contract proxy.
pub struct ValidatorSCProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[allow(dead_code)]
impl<Env, From, To, Gas> ValidatorSCProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn stake<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        staking_amount: BigUint<Env::Api>,
        bls_key: Arg0,
        signature: Arg1,
    ) -> TxTypedCall<Env, From, To, EgldPayment<<Env as TxEnv>::Api>, Gas, ()> {
        self.wrapped_tx
            .raw_call("stake")
            .egld(staking_amount)
            .argument(&1)
            .argument(&bls_key)
            .argument(&signature)
            .original_result()
    }

    pub fn get_total_staked_top_up_staked_bls_keys<Arg0: ProxyArg<ManagedAddress<Env::Api>>>(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .raw_call("getTotalStakedTopUpStakedBlsKeys")
            .payment(NotPayable)
            .argument(&address)
            .original_result()
    }
}
