use crate::types::{
    BigUint, EgldPayment, ManagedBuffer, ManagedVec, NotPayable, ProxyArg, Tx, TxEnv, TxFrom,
    TxGas, TxProxyTrait, TxTo, TxTypedCall,
};

/// Proxy for the Delegation smart contract.
pub struct DelegationSCProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for DelegationSCProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = DelegationSCProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        DelegationSCProxyMethods { wrapped_tx: tx }
    }
}

#[allow(dead_code)]
/// Method container of the Delegation smart contract proxy.
pub struct DelegationSCProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[allow(dead_code)]
impl<Env, From, To, Gas> DelegationSCProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn set_metadata<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg2: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        name: Arg0,
        website: Arg1,
        identifier: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .raw_call("setMetaData")
            .payment(NotPayable)
            .argument(&name)
            .argument(&website)
            .argument(&identifier)
            .original_result()
    }

    pub fn change_service_fee<Arg0: ProxyArg<BigUint<Env::Api>>>(
        self,
        service_fee: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .raw_call("changeServiceFee")
            .payment(NotPayable)
            .argument(&service_fee)
            .original_result()
    }

    pub fn set_automatic_activation(
        self,
        automatic_activation: bool,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        let automatic_activation_str = if automatic_activation {
            "true"
        } else {
            "false"
        };

        self.wrapped_tx
            .raw_call("setAutomaticActivation")
            .payment(NotPayable)
            .argument(&automatic_activation_str)
            .original_result()
    }

    pub fn modify_total_delegation_cap<Arg0: ProxyArg<BigUint<Env::Api>>>(
        self,
        new_total_delegation_cap: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .raw_call("modifyTotalDelegationCap")
            .payment(NotPayable)
            .argument(&new_total_delegation_cap)
            .original_result()
    }

    pub fn add_nodes(
        self,
        bls_keys: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
        signed_messages: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        if bls_keys.len() != signed_messages.len() {
            panic!("BLS keys and nodes must have the same length")
        }

        let mut tx = self.wrapped_tx.raw_call("addNodes").payment(NotPayable);

        for i in 0..bls_keys.len() {
            tx = tx
                .argument(&bls_keys.get(i))
                .argument(&signed_messages.get(i));
        }

        tx.original_result()
    }

    pub fn stake_nodes(
        self,
        bls_keys: &ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        let mut tx = self.wrapped_tx.raw_call("stakeNodes").payment(NotPayable);

        for i in 0..bls_keys.len() {
            tx = tx.argument(&bls_keys.get(i));
        }

        tx.original_result()
    }

    pub fn unstake_nodes(
        self,
        bls_keys: ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        let mut tx = self.wrapped_tx.raw_call("unStakeNodes").payment(NotPayable);

        for bls_key in bls_keys {
            tx = tx.argument(&bls_key);
        }

        tx.original_result()
    }

    pub fn restake_unstaked_nodes(
        self,
        bls_keys: ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        let mut tx = self
            .wrapped_tx
            .raw_call("reStakeUnStakedNodes")
            .payment(NotPayable);

        for bls_key in bls_keys {
            tx = tx.argument(&bls_key);
        }

        tx.original_result()
    }

    pub fn unbond_nodes(
        self,
        bls_keys: ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        let mut tx = self.wrapped_tx.raw_call("unBondNodes").payment(NotPayable);

        for bls_key in bls_keys {
            tx = tx.argument(&bls_key);
        }

        tx.original_result()
    }

    pub fn remove_nodes(
        self,
        bls_keys: ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        let mut tx = self.wrapped_tx.raw_call("removeNodes").payment(NotPayable);

        for bls_key in bls_keys {
            tx = tx.argument(&bls_key);
        }

        tx.original_result()
    }

    pub fn unjail_nodes(
        self,
        bls_keys: ManagedVec<Env::Api, ManagedBuffer<Env::Api>>,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        let mut tx = self.wrapped_tx.raw_call("unJailNodes").payment(NotPayable);

        for bls_key in bls_keys {
            tx = tx.argument(&bls_key);
        }

        tx.original_result()
    }

    pub fn delegate(
        self,
        egld_value: BigUint<Env::Api>,
    ) -> TxTypedCall<Env, From, To, EgldPayment<<Env as TxEnv>::Api>, Gas, ()> {
        if egld_value < BigUint::from(1000000000000000000u128) {
            panic!("The minimum value for creating a new delegation contract is 1 EGLD");
        }

        self.wrapped_tx
            .raw_call("delegate")
            .egld(egld_value)
            .original_result()
    }

    pub fn claim_rewards(self) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .raw_call("claimRewards")
            .payment(NotPayable)
            .original_result()
    }

    pub fn redelegate_rewards(self) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .raw_call("reDelegateRewards")
            .payment(NotPayable)
            .original_result()
    }

    pub fn undelegate(
        self,
        undelegate_egld_amount: BigUint<Env::Api>,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        if undelegate_egld_amount < BigUint::from(1000000000000000000u128) {
            panic!("The minimum value for undelegating is 1 EGLD");
        }

        self.wrapped_tx
            .raw_call("unDelegate")
            .payment(NotPayable)
            .argument(&undelegate_egld_amount)
            .original_result()
    }

    pub fn withdraw(self) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .raw_call("withdraw")
            .payment(NotPayable)
            .original_result()
    }
}
