mod delegation_contract_config_result;
mod delegation_node_states_result;

pub use delegation_contract_config_result::DelegationContractConfigResult;
pub use delegation_node_states_result::DelegationNodeStatesResult;

use multiversx_chain_core::types::{BLSKey, BLSSignature};
use multiversx_sc_codec::{
    MultiValueLength,
    multi_types::{MultiValue2, MultiValue3, MultiValue4, MultiValueVec},
};

use crate::types::{
    BigUint, EgldPayment, ManagedAddress, ManagedBuffer, MultiValueEncoded, NotPayable, ProxyArg,
    Tx, TxEnv, TxFrom, TxGas, TxProxyTrait, TxTo, TxTypedCall,
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

#[allow(clippy::type_complexity)]
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

    /// Service fee is expressed in basis points (10000 = 100%).
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

    /// Each BLS key must be accompanied by the signature of the delegation contract address,
    /// produced with the BLS secret key.
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

    /// Unjails nodes. The payment is `unjail_price_per_node` multiplied by the number of keys,
    /// and must match exactly the unjail price configured in the validator system SC
    /// (2.5 EGLD on mainnet at the time of writing).
    pub fn unjail_nodes<Arg0: ProxyArg<MultiValueVec<BLSKey>> + MultiValueLength>(
        self,
        bls_keys: Arg0,
        unjail_price_per_node: BigUint<Env::Api>,
    ) -> TxTypedCall<Env, From, To, EgldPayment<<Env as TxEnv>::Api>, Gas, ()> {
        let num_keys = bls_keys.multi_value_len();
        self.wrapped_tx
            .raw_call("unJailNodes")
            .egld(unjail_price_per_node * num_keys as u64)
            .argument(&bls_keys)
            .original_result()
    }

    /// Delegates EGLD to the contract.
    ///
    /// The value must be at least the minimum delegation amount, configured in the delegation manager
    /// (1 EGLD on mainnet at the time of writing).
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

    /// Undelegates the given EGLD amount. The amount is passed as an argument, not as payment.
    ///
    /// The remaining active stake must be either 0, or at least the minimum delegation amount.
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

    /// Whitelists a validator owner address, allowing it to merge its validator data into this contract.
    pub fn whitelist_for_merge<Arg0: ProxyArg<ManagedAddress<Env::Api>>>(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("whitelistForMerge")
            .argument(&address)
            .original_result()
    }

    /// Removes the address whitelisted for merge.
    pub fn delete_whitelist_for_merge(self) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("deleteWhitelistForMerge")
            .original_result()
    }

    /// Transfers the ownership of the delegation contract.
    pub fn change_owner<Arg0: ProxyArg<ManagedAddress<Env::Api>>>(
        self,
        new_owner: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("changeOwner")
            .argument(&new_owner)
            .original_result()
    }

    /// Synchronizes the owner stored in the contract with the owner of the account.
    pub fn synchronize_owner(self) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("synchronizeOwner")
            .original_result()
    }

    /// Returns all BLS keys of the contract, grouped by state.
    pub fn get_all_node_states(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, DelegationNodeStatesResult> {
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

    pub fn get_total_unstaked(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getTotalUnStaked")
            .original_result()
    }

    pub fn get_num_users(self) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getNumUsers")
            .original_result()
    }

    pub fn get_num_nodes(self) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getNumNodes")
            .original_result()
    }

    pub fn get_contract_config(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, DelegationContractConfigResult<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getContractConfig")
            .original_result()
    }

    /// Returns the staking provider metadata: name, website, identifier.
    pub fn get_meta_data(
        self,
    ) -> TxTypedCall<
        Env,
        From,
        To,
        NotPayable,
        Gas,
        MultiValue3<ManagedBuffer<Env::Api>, ManagedBuffer<Env::Api>, ManagedBuffer<Env::Api>>,
    > {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getMetaData")
            .original_result()
    }

    /// Returns the rewards data for an epoch: rewards to distribute, total active stake, service fee.
    pub fn get_reward_data<Arg0: ProxyArg<u64>>(
        self,
        epoch: Arg0,
    ) -> TxTypedCall<
        Env,
        From,
        To,
        NotPayable,
        Gas,
        MultiValue3<BigUint<Env::Api>, BigUint<Env::Api>, u64>,
    > {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getRewardData")
            .argument(&epoch)
            .original_result()
    }

    /// Returns the address whitelisted for merge, if any.
    pub fn get_whitelist_for_merge(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getWhitelistForMerge")
            .original_result()
    }

    /// Fails if the address is not a delegator in this contract.
    pub fn is_delegator<Arg0: ProxyArg<ManagedAddress<Env::Api>>>(
        self,
        delegator: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("isDelegator")
            .argument(&delegator)
            .original_result()
    }

    /// Fails if the address is not a delegator in this contract.
    pub fn get_user_active_stake<Arg0: ProxyArg<ManagedAddress<Env::Api>>>(
        self,
        delegator: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getUserActiveStake")
            .argument(&delegator)
            .original_result()
    }

    /// Fails if the address is not a delegator in this contract.
    pub fn get_user_unstaked_value<Arg0: ProxyArg<ManagedAddress<Env::Api>>>(
        self,
        delegator: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getUserUnStakedValue")
            .argument(&delegator)
            .original_result()
    }

    /// Fails if the address is not a delegator in this contract.
    pub fn get_user_unbondable<Arg0: ProxyArg<ManagedAddress<Env::Api>>>(
        self,
        delegator: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getUserUnBondable")
            .argument(&delegator)
            .original_result()
    }

    /// Returns the undelegated funds of a delegator, as pairs of (amount, epochs remaining until unbondable).
    ///
    /// Fails if the address is not a delegator in this contract.
    pub fn get_user_undelegated_list<Arg0: ProxyArg<ManagedAddress<Env::Api>>>(
        self,
        delegator: Arg0,
    ) -> TxTypedCall<
        Env,
        From,
        To,
        NotPayable,
        Gas,
        MultiValueEncoded<Env::Api, MultiValue2<BigUint<Env::Api>, u64>>,
    > {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getUserUnDelegatedList")
            .argument(&delegator)
            .original_result()
    }

    /// Fails if the address is not a delegator in this contract.
    pub fn get_claimable_rewards<Arg0: ProxyArg<ManagedAddress<Env::Api>>>(
        self,
        delegator: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getClaimableRewards")
            .argument(&delegator)
            .original_result()
    }

    /// Fails if the address is not a delegator in this contract.
    pub fn get_total_cumulated_rewards_for_user<Arg0: ProxyArg<ManagedAddress<Env::Api>>>(
        self,
        delegator: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getTotalCumulatedRewardsForUser")
            .argument(&delegator)
            .original_result()
    }

    /// Returns: active stake, claimable rewards, unstaked value, unbondable value.
    ///
    /// Fails if the address is not a delegator in this contract.
    pub fn get_delegator_funds_data<Arg0: ProxyArg<ManagedAddress<Env::Api>>>(
        self,
        delegator: Arg0,
    ) -> TxTypedCall<
        Env,
        From,
        To,
        NotPayable,
        Gas,
        MultiValue4<BigUint<Env::Api>, BigUint<Env::Api>, BigUint<Env::Api>, BigUint<Env::Api>>,
    > {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getDelegatorFundsData")
            .argument(&delegator)
            .original_result()
    }
}
