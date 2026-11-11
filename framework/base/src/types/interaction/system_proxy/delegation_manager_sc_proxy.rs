use multiversx_sc_codec::multi_types::MultiValue6;

use crate::types::{
    BigUint, Egld, ManagedAddress, MultiValueEncoded, NotPayable, ProxyArg, Tx, TxEgldValue, TxEnv,
    TxFrom, TxGas, TxProxyTrait, TxTo, TxTypedCall,
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
    /// Creates a new delegation contract (staking provider). Returns the new contract address.
    ///
    /// - `total_delegation_cap` is in EGLD atto-units, 0 means uncapped;
    /// - `service_fee` is in basis points (10000 = 100%);
    /// - `amount` must be at least the minimum deposit (1250 EGLD on mainnet at the time of writing).
    pub fn create_new_delegation_contract<
        Arg0: ProxyArg<BigUint<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
        EgldValue: TxEgldValue<Env>,
    >(
        self,
        total_delegation_cap: Arg0,
        service_fee: Arg1,
        amount: EgldValue,
    ) -> TxTypedCall<Env, From, To, Egld<EgldValue>, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .raw_call("createNewDelegationContract")
            .egld(amount)
            .argument(&total_delegation_cap)
            .argument(&service_fee)
            .original_result()
    }

    /// Converts the caller's existing validator into a new delegation contract.
    /// Returns the new contract address.
    ///
    /// - `max_delegation_cap` is in EGLD atto-units, 0 means uncapped;
    /// - `service_fee` is in basis points (10000 = 100%).
    pub fn make_new_contract_from_validator_data<
        Arg0: ProxyArg<BigUint<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        max_delegation_cap: Arg0,
        service_fee: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("makeNewContractFromValidatorData")
            .argument(&max_delegation_cap)
            .argument(&service_fee)
            .original_result()
    }

    /// Merges the caller's validator data into a delegation contract owned by the same address.
    pub fn merge_validator_to_delegation_same_owner<Arg0: ProxyArg<ManagedAddress<Env::Api>>>(
        self,
        delegation_contract: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("mergeValidatorToDelegationSameOwner")
            .argument(&delegation_contract)
            .original_result()
    }

    /// Merges the caller's validator data into a delegation contract that whitelisted the caller.
    pub fn merge_validator_to_delegation_with_whitelist<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        delegation_contract: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("mergeValidatorToDelegationWithWhitelist")
            .argument(&delegation_contract)
            .original_result()
    }

    /// Claims rewards from multiple delegation contracts at once. Returns the total amount claimed.
    pub fn claim_multi<Arg0: ProxyArg<MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>>>(
        self,
        delegation_contracts: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("claimMulti")
            .argument(&delegation_contracts)
            .original_result()
    }

    /// Redelegates rewards in multiple delegation contracts at once. Returns the total amount redelegated.
    pub fn redelegate_multi<
        Arg0: ProxyArg<MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>>,
    >(
        self,
        delegation_contracts: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("reDelegateMulti")
            .argument(&delegation_contracts)
            .original_result()
    }

    /// Returns the addresses of all delegation contracts (excluding the initial built-in one).
    pub fn get_all_contract_addresses(
        self,
    ) -> TxTypedCall<
        Env,
        From,
        To,
        NotPayable,
        Gas,
        MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>,
    > {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAllContractAddresses")
            .original_result()
    }

    /// Returns the delegation manager configuration:
    /// number of contracts, last created contract address, min service fee, max service fee,
    /// min deposit (for creating a contract), min delegation amount.
    pub fn get_contract_config(
        self,
    ) -> TxTypedCall<
        Env,
        From,
        To,
        NotPayable,
        Gas,
        MultiValue6<u64, ManagedAddress<Env::Api>, u64, u64, BigUint<Env::Api>, BigUint<Env::Api>>,
    > {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getContractConfig")
            .original_result()
    }
}
