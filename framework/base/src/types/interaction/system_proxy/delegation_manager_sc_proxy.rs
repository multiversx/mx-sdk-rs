use crate::types::{
    BigUint, EgldPayment, ManagedAddress, ManagedVec, NotPayable, ProxyArg, Tx, TxEnv, TxFrom,
    TxGas, TxProxyTrait, TxTo, TxTypedCall,
};

/// Proxy for the Delegation Manager smart contract.
pub struct DelegationManagerSCProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for DelegationManagerSCProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = DelegationManagerSCProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        DelegationManagerSCProxyMethods { wrapped_tx: tx }
    }
}

/// Method container of the Delegation Manager smart contract proxy.
pub struct DelegationManagerSCProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[allow(clippy::type_complexity)]
impl<Env, From, To, Gas> DelegationManagerSCProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    /// Field "amount" requires at least 1250 EGLD.
    pub fn create_new_delegation_contract<
        Arg0: ProxyArg<BigUint<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        total_delegation_cap: Arg0,
        service_fee: Arg1,
        amount: BigUint<Env::Api>,
    ) -> TxTypedCall<Env, From, To, EgldPayment<<Env as TxEnv>::Api>, Gas, ()> {
        self.wrapped_tx
            .raw_call("createNewDelegationContract")
            .egld(amount)
            .argument(&total_delegation_cap)
            .argument(&service_fee)
            .original_result()
    }

    pub fn get_all_contract_addresses(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedVec<Env::Api, ManagedAddress<Env::Api>>>
    {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAllContractAddresses")
            .original_result()
    }
}
