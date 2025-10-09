use multiversx_chain_core::types::{BLSKey, BLSSignature};
use multiversx_sc_codec::{
    multi_types::{MultiValue2, MultiValueVec},
    MultiValueLength,
};

use crate::types::{
    BigUint, EgldPayment, ManagedAddress, ManagedBuffer, NotPayable, ProxyArg, Tx, TxEnv, TxFrom,
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

    pub fn add_nodes<Arg0: ProxyArg<MultiValueVec<MultiValue2<BLSKey, BLSSignature>>>>(
        self,
        bls_keys_signatures: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("addNodes")
            .argument(&bls_keys_signatures)
            .original_result()
    }

    pub fn stake_nodes<Arg0: ProxyArg<MultiValueVec<BLSKey>>>(
        self,
        bls_keys: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .raw_call("stakeNodes")
            .payment(NotPayable)
            .argument(&bls_keys)
            .original_result()
    }

    pub fn unstake_nodes<Arg0: ProxyArg<MultiValueVec<BLSKey>>>(
        self,
        bls_keys: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .raw_call("unStakeNodes")
            .payment(NotPayable)
            .argument(&bls_keys)
            .original_result()
    }

    pub fn restake_unstaked_nodes<Arg0: ProxyArg<MultiValueVec<BLSKey>>>(
        self,
        bls_keys: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .raw_call("reStakeUnStakedNodes")
            .payment(NotPayable)
            .argument(&bls_keys)
            .original_result()
    }

    pub fn unbond_nodes<Arg0: ProxyArg<MultiValueVec<BLSKey>>>(
        self,
        bls_keys: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .raw_call("unBondNodes")
            .payment(NotPayable)
            .argument(&bls_keys)
            .original_result()
    }

    pub fn remove_nodes<Arg0: ProxyArg<MultiValueVec<BLSKey>>>(
        self,
        bls_keys: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .raw_call("removeNodes")
            .payment(NotPayable)
            .argument(&bls_keys)
            .original_result()
    }

    pub fn unjail_nodes<Arg0: ProxyArg<MultiValueVec<BLSKey>> + MultiValueLength>(
        self,
        bls_keys: Arg0,
    ) -> TxTypedCall<Env, From, To, EgldPayment<<Env as TxEnv>::Api>, Gas, ()> {
        let num_keys = bls_keys.multi_value_len();
        self.wrapped_tx
            .raw_call("unJailNodes")
            .egld(BigUint::from(2500000000000000000u128) * num_keys as u64)
            .argument(&bls_keys)
            .original_result()
    }

    /// The minimum value for creating a new delegation contract is 1 EGLD
    pub fn delegate(
        self,
        egld_value: BigUint<Env::Api>,
    ) -> TxTypedCall<Env, From, To, EgldPayment<<Env as TxEnv>::Api>, Gas, ()> {
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

    pub fn set_check_cap_on_redelegate_rewards(
        self,
        state: bool,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        let state_str = if state { "true" } else { "false" };

        self.wrapped_tx
            .raw_call("setCheckCapOnReDelegateRewards")
            .payment(NotPayable)
            .argument(&state_str)
            .original_result()
    }

    /// The minimum value for undelegating is 1 EGLD
    pub fn undelegate<Arg0: ProxyArg<BigUint<Env::Api>>>(
        self,
        undelegate_egld_amount: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
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

    pub fn get_all_node_states(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedBuffer<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAllNodeStates")
            .original_result()
    }

    pub fn get_total_active_stake(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getTotalActiveStake")
            .original_result()
    }

    pub fn get_user_active_stake(
        self,
        owner: &ManagedAddress<Env::Api>,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getUserActiveStake")
            .argument(owner)
            .original_result()
    }
}
