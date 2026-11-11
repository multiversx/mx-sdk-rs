pub(super) mod governance_config_result;
pub(super) mod proposal_view_result;

use multiversx_sc_codec::multi_types::MultiValue4;

use crate::types::{
    BigUint, Egld, ManagedAddress, ManagedBuffer, MultiValueEncoded, NotPayable, ProxyArg, Tx,
    TxEgldValue, TxEnv, TxFrom, TxGas, TxProxyTrait, TxTo, TxTypedCall,
};
use governance_config_result::GovernanceConfigResult;
use proposal_view_result::ProposalViewResult;

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

#[allow(clippy::type_complexity)]
impl<Env, From, To, Gas> GovernanceSCProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    /// Submits a new proposal.
    ///
    /// - `commit_hash` must be exactly 40 bytes long (a git commit hash);
    /// - `start_vote_epoch` must not be in the past, and not after `end_vote_epoch`;
    /// - `proposal_fee` must be exactly equal to the current proposal fee
    ///   (see `view_config`, 500 EGLD on mainnet at the time of writing).
    pub fn proposal<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<u64>,
        Arg2: ProxyArg<u64>,
        EgldValue: TxEgldValue<Env>,
    >(
        self,
        commit_hash: Arg0,
        start_vote_epoch: Arg1,
        end_vote_epoch: Arg2,
        proposal_fee: EgldValue,
    ) -> TxTypedCall<Env, From, To, Egld<EgldValue>, Gas, ()> {
        self.wrapped_tx
            .raw_call("proposal")
            .egld(proposal_fee)
            .argument(&commit_hash)
            .argument(&start_vote_epoch)
            .argument(&end_vote_epoch)
            .original_result()
    }

    /// Votes on a proposal, with the full voting power of the caller.
    ///
    /// `vote` must be one of `"yes"`, `"no"`, `"abstain"`, `"veto"`.
    pub fn vote<Arg0: ProxyArg<u64>, Arg1: ProxyArg<ManagedBuffer<Env::Api>>>(
        self,
        proposal_nonce: Arg0,
        vote: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("vote")
            .argument(&proposal_nonce)
            .argument(&vote)
            .original_result()
    }

    /// Votes on behalf of a user. Can only be called by smart contracts (e.g. liquid staking providers).
    ///
    /// `vote` must be one of `"yes"`, `"no"`, `"abstain"`, `"veto"`.
    pub fn delegate_vote<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg2: ProxyArg<ManagedAddress<Env::Api>>,
        Arg3: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        proposal_nonce: Arg0,
        vote: Arg1,
        voter: Arg2,
        user_stake: Arg3,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("delegateVote")
            .argument(&proposal_nonce)
            .argument(&vote)
            .argument(&voter)
            .argument(&user_stake)
            .original_result()
    }

    /// Changes the governance configuration. Can only be called by the governance SC owner.
    ///
    /// - fees are expressed in EGLD atto-units, `proposal_fee` must be at least `lost_proposal_fee`;
    /// - thresholds are expressed in basis points (0..=10000, i.e. 10000 = 100%).
    ///
    /// All values are sent as base 10 strings, as expected by the system SC.
    pub fn change_config(
        self,
        proposal_fee: &BigUint<Env::Api>,
        lost_proposal_fee: &BigUint<Env::Api>,
        min_quorum: u32,
        min_veto: u32,
        min_pass: u32,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("changeConfig")
            .argument(&proposal_fee.to_display())
            .argument(&lost_proposal_fee.to_display())
            .argument(&u32_to_display::<Env>(min_quorum))
            .argument(&u32_to_display::<Env>(min_veto))
            .argument(&u32_to_display::<Env>(min_pass))
            .original_result()
    }

    /// Closes a proposal after its voting period has ended.
    pub fn close_proposal<Arg0: ProxyArg<u64>>(
        self,
        proposal_nonce: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("closeProposal")
            .argument(&proposal_nonce)
            .original_result()
    }

    /// Clears the ended proposals of the given proposers.
    pub fn clear_ended_proposals<
        Arg0: ProxyArg<MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>>,
    >(
        self,
        proposers: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("clearEndedProposals")
            .argument(&proposers)
            .original_result()
    }

    #[deprecated(since = "0.67.0", note = "renamed to clear_ended_proposals")]
    pub fn clear_ended_proposal<
        Arg0: ProxyArg<MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>>,
    >(
        self,
        proposers: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.clear_ended_proposals(proposers)
    }

    /// Transfers the accumulated proposal fees to the caller. Can only be called by the governance SC owner.
    pub fn claim_accumulated_fees(self) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("claimAccumulatedFees")
            .original_result()
    }

    /// Returns the voting power of an address.
    ///
    /// Only works as a query.
    pub fn view_voting_power<Arg0: ProxyArg<ManagedAddress<Env::Api>>>(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("viewVotingPower")
            .argument(&address)
            .original_result()
    }

    /// Returns the current governance configuration.
    ///
    /// Only works as a query.
    pub fn view_config(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, GovernanceConfigResult> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("viewConfig")
            .original_result()
    }

    /// Returns the delegated vote info of a user in a smart contract that votes on their behalf:
    /// used stake, used power, total stake, total power.
    ///
    /// Only works as a query.
    pub fn view_delegated_vote_info<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        sc_address: Arg0,
        user_address: Arg1,
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
            .raw_call("viewDelegatedVoteInfo")
            .argument(&sc_address)
            .argument(&user_address)
            .original_result()
    }

    /// Returns the details of a proposal.
    ///
    /// Only works as a query.
    pub fn view_proposal<Arg0: ProxyArg<u64>>(
        self,
        proposal_nonce: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ProposalViewResult<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("viewProposal")
            .argument(&proposal_nonce)
            .original_result()
    }
}

fn u32_to_display<Env: TxEnv>(value: u32) -> ManagedBuffer<Env::Api> {
    BigUint::<Env::Api>::from(value).to_display()
}
