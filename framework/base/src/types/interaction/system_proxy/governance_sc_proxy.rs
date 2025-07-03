use crate::types::{
    BigUint, ManagedAddress, ManagedBuffer, MultiValueEncoded, NotPayable, ProxyArg, Tx, TxEnv,
    TxFrom, TxGas, TxProxyTrait, TxTo, TxTypedCall,
};

/// Proxy for the Governance system smart contract.
pub struct GovernanceSCProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for GovernanceSCProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = GovernanceSCProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        GovernanceSCProxyMethods { wrapped_tx: tx }
    }
}

/// Method container of the Governance system smart contract proxy.
pub struct GovernanceSCProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

impl<Env, From, To, Gas> GovernanceSCProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn init_v2(self) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("initV2")
            .original_result()
    }

    pub fn proposal<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
        Arg2: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        commit_hash: Arg0,
        start_vote_epoch: Arg1,
        end_vote_epoch: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("proposal")
            .argument(&commit_hash)
            .argument(&start_vote_epoch)
            .argument(&end_vote_epoch)
            .original_result()
    }

    pub fn vote<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        proposal_to_vote: Arg0,
        vote: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("vote")
            .argument(&proposal_to_vote)
            .argument(&vote)
            .original_result()
    }

    pub fn delegate_vote<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg2: ProxyArg<ManagedAddress<Env::Api>>,
        Arg3: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        proposal_to_vote: Arg0,
        vote: Arg1,
        voter: Arg2,
        user_stake: Arg3,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("delegateVote")
            .argument(&proposal_to_vote)
            .argument(&vote)
            .argument(&voter)
            .argument(&user_stake)
            .original_result()
    }

    pub fn change_config<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg2: ProxyArg<BigUint<Env::Api>>,
        Arg3: ProxyArg<BigUint<Env::Api>>,
        Arg4: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        proposal_fee: Arg0,
        lost_proposal_fee: Arg1,
        min_quorum: Arg2,
        min_veto: Arg3,
        min_pass: Arg4,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("changeConfig")
            .argument(&proposal_fee)
            .argument(&lost_proposal_fee)
            .argument(&min_quorum)
            .argument(&min_veto)
            .argument(&min_pass)
            .original_result()
    }

    pub fn close_proposal<Arg0: ProxyArg<BigUint<Env::Api>>>(
        self,
        nonce: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("closeProposal")
            .argument(&nonce)
            .original_result()
    }

    pub fn clear_ended_proposal<
        Arg0: ProxyArg<MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>>,
    >(
        self,
        voters: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("clearEndedProposals")
            .argument(&voters)
            .original_result()
    }

    pub fn view_voting_power<Arg0: ProxyArg<ManagedAddress<Env::Api>>>(
        self,
        validator_address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("viewVotingPower")
            .argument(&validator_address)
            .original_result()
    }

    pub fn view_config(self) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("viewConfig")
            .original_result()
    }

    pub fn view_delegated_vote_info<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        sc_address: Arg0,
        reference: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("viewDelegatedVoteInfo")
            .argument(&sc_address)
            .argument(&reference)
            .original_result()
    }

    pub fn view_proposal<Arg0: ProxyArg<BigUint<Env::Api>>>(
        self,
        nonce: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("viewProposal")
            .argument(&nonce)
            .original_result()
    }

    pub fn claim_accumulated_fees<Arg0: ProxyArg<BigUint<Env::Api>>>(
        self,
        nonce: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("claimAccumulatedFees")
            .argument(&nonce)
            .original_result()
    }
}
