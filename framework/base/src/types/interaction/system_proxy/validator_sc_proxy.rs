use multiversx_chain_core::types::{BLSKey, BLSSignature};
use multiversx_sc_codec::multi_types::{MultiValue2, MultiValue4, MultiValueVec};

use crate::types::{
    BigUint, Egld, EgldPayment, ManagedAddress, ManagedBuffer, MultiValueEncoded, NotPayable,
    ProxyArg, Tx, TxEgldValue, TxEnv, TxFrom, TxGas, TxProxyTrait, TxTo, TxTypedCall,
};

/// Proxy for the Validator system smart contract (also known as the auction or staking SC).
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

/// Method container of the Validator system smart contract proxy.
pub struct ValidatorSCProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[allow(clippy::type_complexity)]
impl<Env, From, To, Gas> ValidatorSCProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    /// Stakes new validator nodes.
    ///
    /// Each BLS key must be accompanied by the signature of the sender address, produced with the BLS secret key.
    /// The number of nodes is derived from the list.
    ///
    /// The total stake of the sender must cover the node price for all nodes
    /// (2500 EGLD per node on mainnet at the time of writing).
    pub fn stake<EgldValue: TxEgldValue<Env>>(
        self,
        bls_keys_signatures: MultiValueVec<MultiValue2<BLSKey, BLSSignature>>,
        amount: EgldValue,
    ) -> TxTypedCall<Env, From, To, Egld<EgldValue>, Gas, ()> {
        let num_nodes = bls_keys_signatures.len() as u64;
        self.wrapped_tx
            .raw_call("stake")
            .egld(amount)
            .argument(&num_nodes)
            .argument(&bls_keys_signatures)
            .original_result()
    }

    /// Same as `stake`, but also sets the reward address.
    ///
    /// The reward address is only taken into account the first time the sender stakes.
    /// Afterwards, use `change_reward_address`.
    pub fn stake_with_reward_address<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        EgldValue: TxEgldValue<Env>,
    >(
        self,
        bls_keys_signatures: MultiValueVec<MultiValue2<BLSKey, BLSSignature>>,
        reward_address: Arg0,
        amount: EgldValue,
    ) -> TxTypedCall<Env, From, To, Egld<EgldValue>, Gas, ()> {
        let num_nodes = bls_keys_signatures.len() as u64;
        self.wrapped_tx
            .raw_call("stake")
            .egld(amount)
            .argument(&num_nodes)
            .argument(&bls_keys_signatures)
            .argument(&reward_address)
            .original_result()
    }

    /// Adds stake (top-up) without registering new nodes.
    pub fn stake_top_up<EgldValue: TxEgldValue<Env>>(
        self,
        amount: EgldValue,
    ) -> TxTypedCall<Env, From, To, Egld<EgldValue>, Gas, ()> {
        self.wrapped_tx
            .raw_call("stake")
            .egld(amount)
            .original_result()
    }

    /// Unstakes nodes, together with their corresponding stake.
    pub fn unstake<Arg0: ProxyArg<MultiValueVec<BLSKey>>>(
        self,
        bls_keys: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("unStake")
            .argument(&bls_keys)
            .original_result()
    }

    /// Unstakes nodes, but leaves the tokens staked (as top-up).
    pub fn unstake_nodes<Arg0: ProxyArg<MultiValueVec<BLSKey>>>(
        self,
        bls_keys: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("unStakeNodes")
            .argument(&bls_keys)
            .original_result()
    }

    /// Unstakes the given amount of EGLD (from the top-up).
    pub fn unstake_tokens<Arg0: ProxyArg<BigUint<Env::Api>>>(
        self,
        amount: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("unStakeTokens")
            .argument(&amount)
            .original_result()
    }

    /// Unbonds previously unstaked nodes, together with their corresponding stake.
    pub fn unbond<Arg0: ProxyArg<MultiValueVec<BLSKey>>>(
        self,
        bls_keys: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("unBond")
            .argument(&bls_keys)
            .original_result()
    }

    /// Unbonds previously unstaked nodes, without unbonding tokens.
    pub fn unbond_nodes<Arg0: ProxyArg<MultiValueVec<BLSKey>>>(
        self,
        bls_keys: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("unBondNodes")
            .argument(&bls_keys)
            .original_result()
    }

    /// Unbonds the given amount of previously unstaked EGLD.
    pub fn unbond_tokens<Arg0: ProxyArg<BigUint<Env::Api>>>(
        self,
        amount: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("unBondTokens")
            .argument(&amount)
            .original_result()
    }

    /// Unjails nodes. The payment is `unjail_price_per_node` multiplied by the number of keys,
    /// and must match exactly the configured unjail price (2.5 EGLD on mainnet at the time of writing).
    pub fn unjail(
        self,
        bls_keys: MultiValueVec<BLSKey>,
        unjail_price_per_node: BigUint<Env::Api>,
    ) -> TxTypedCall<Env, From, To, EgldPayment<Env::Api>, Gas, ()> {
        let num_keys = bls_keys.len() as u64;
        self.wrapped_tx
            .raw_call("unJail")
            .egld(unjail_price_per_node * num_keys)
            .argument(&bls_keys)
            .original_result()
    }

    pub fn change_reward_address<Arg0: ProxyArg<ManagedAddress<Env::Api>>>(
        self,
        reward_address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("changeRewardAddress")
            .argument(&reward_address)
            .original_result()
    }

    /// Restakes nodes that were previously unstaked, but not yet unbonded.
    pub fn restake_unstaked_nodes<Arg0: ProxyArg<MultiValueVec<BLSKey>>>(
        self,
        bls_keys: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("reStakeUnStakedNodes")
            .argument(&bls_keys)
            .original_result()
    }

    /// Removes duplicate BLS keys from the sender's registration data.
    pub fn clean_registered_data(self) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("cleanRegisteredData")
            .original_result()
    }

    /// Returns the total stake of a validator owner, as a base 10 string.
    ///
    /// Fails if the address never staked.
    pub fn get_total_staked<Arg0: ProxyArg<ManagedAddress<Env::Api>>>(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedBuffer<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getTotalStaked")
            .argument(&address)
            .original_result()
    }

    /// Returns: top-up, total stake, number of active nodes, followed by the staked BLS keys.
    pub fn get_total_staked_top_up_staked_bls_keys<Arg0: ProxyArg<ManagedAddress<Env::Api>>>(
        self,
        address: Arg0,
    ) -> TxTypedCall<
        Env,
        From,
        To,
        NotPayable,
        Gas,
        MultiValue4<
            BigUint<Env::Api>,
            BigUint<Env::Api>,
            u64,
            MultiValueEncoded<Env::Api, ManagedBuffer<Env::Api>>,
        >,
    > {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getTotalStakedTopUpStakedBlsKeys")
            .argument(&address)
            .original_result()
    }

    /// Returns the unstaked tokens of a validator owner, as pairs of (amount, epochs remaining until unbondable).
    pub fn get_unstaked_tokens_list<Arg0: ProxyArg<ManagedAddress<Env::Api>>>(
        self,
        address: Arg0,
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
            .raw_call("getUnStakedTokensList")
            .argument(&address)
            .original_result()
    }

    /// Returns the status of each BLS key of a validator owner, as pairs of (BLS key, status).
    ///
    /// Only works as a query.
    pub fn get_bls_keys_status<Arg0: ProxyArg<ManagedAddress<Env::Api>>>(
        self,
        address: Arg0,
    ) -> TxTypedCall<
        Env,
        From,
        To,
        NotPayable,
        Gas,
        MultiValueEncoded<Env::Api, MultiValue2<ManagedBuffer<Env::Api>, ManagedBuffer<Env::Api>>>,
    > {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getBlsKeysStatus")
            .argument(&address)
            .original_result()
    }
}
