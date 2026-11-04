use alloc::string::String;

use multiversx_chain_core::types::Address;
use multiversx_sc_codec::{DecodeErrorHandler, TopDecodeMulti, TopDecodeMultiInput};

use crate::{api::ManagedTypeApi, types::BigUint};

/// The decoded result of the `viewProposal` view of the governance system SC.
///
/// Stake and vote amounts are expressed in EGLD atto-units.
#[derive(Debug, Clone)]
pub struct ProposalViewResult<M: ManagedTypeApi> {
    pub proposal_cost: BigUint<M>,
    pub commit_hash: String,
    pub proposal_nonce: u64,
    pub issuer_address: Address,
    pub start_vote_epoch: u64,
    pub end_vote_epoch: u64,
    pub quorum_stake: BigUint<M>,
    pub yes: BigUint<M>,
    pub no: BigUint<M>,
    pub veto: BigUint<M>,
    pub abstain: BigUint<M>,
    pub closed: bool,
    pub passed: bool,
}

impl<M> TopDecodeMulti for ProposalViewResult<M>
where
    M: ManagedTypeApi,
{
    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        let is_true = |value: String| value == "true";

        let proposal_cost = BigUint::multi_decode_or_handle_err(input, h)?;
        let commit_hash = String::multi_decode_or_handle_err(input, h)?;
        let proposal_nonce = u64::multi_decode_or_handle_err(input, h)?;
        let issuer_address = Address::multi_decode_or_handle_err(input, h)?;
        let start_vote_epoch = u64::multi_decode_or_handle_err(input, h)?;
        let end_vote_epoch = u64::multi_decode_or_handle_err(input, h)?;
        let quorum_stake = BigUint::multi_decode_or_handle_err(input, h)?;
        let yes = BigUint::multi_decode_or_handle_err(input, h)?;
        let no = BigUint::multi_decode_or_handle_err(input, h)?;
        let veto = BigUint::multi_decode_or_handle_err(input, h)?;
        let abstain = BigUint::multi_decode_or_handle_err(input, h)?;
        let closed = String::multi_decode_or_handle_err(input, h)?;
        let passed = String::multi_decode_or_handle_err(input, h)?;
        Ok(ProposalViewResult {
            proposal_cost,
            commit_hash,
            proposal_nonce,
            issuer_address,
            start_vote_epoch,
            end_vote_epoch,
            quorum_stake,
            yes,
            no,
            veto,
            abstain,
            closed: is_true(closed),
            passed: is_true(passed),
        })
    }
}
