use alloc::string::String;

use multiversx_chain_core::std::Bech32Address;
use multiversx_sc_codec::{
    num_bigint::BigUint, DecodeErrorHandler, TopDecodeMulti, TopDecodeMultiInput,
};

#[derive(Debug, Clone, Default)]
pub struct ProposalViewResult {
    pub proposal_cost: BigUint,
    pub commit_hash: String,
    pub proposal_nonce: u64,
    pub issuer_address: Bech32Address,
    pub start_vote_epoch: u64,
    pub end_vote_epoch: u64,
    pub quorum_stake: u64,
    pub yes: u64,
    pub no: u64,
    pub veto: u64,
    pub abstain: u64,
    pub closed: bool,
    pub passed: bool,
}

impl TopDecodeMulti for ProposalViewResult {
    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        let is_true = |value: String| value == "true";

        let proposal_cost = BigUint::multi_decode_or_handle_err(input, h)?;
        let commit_hash = String::multi_decode_or_handle_err(input, h)?;
        let proposal_nonce = u64::multi_decode_or_handle_err(input, h)?;
        let issuer_address = Bech32Address::multi_decode_or_handle_err(input, h)?;
        let start_vote_epoch = u64::multi_decode_or_handle_err(input, h)?;
        let end_vote_epoch = u64::multi_decode_or_handle_err(input, h)?;
        let quorum_stake = u64::multi_decode_or_handle_err(input, h)?;
        let yes = u64::multi_decode_or_handle_err(input, h)?;
        let no = u64::multi_decode_or_handle_err(input, h)?;
        let veto = u64::multi_decode_or_handle_err(input, h)?;
        let abstain = u64::multi_decode_or_handle_err(input, h)?;
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
