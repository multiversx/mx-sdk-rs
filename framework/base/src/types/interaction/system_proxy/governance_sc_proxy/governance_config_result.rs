use alloc::string::String;

use multiversx_sc_codec::{DecodeErrorHandler, TopDecodeMulti, TopDecodeMultiInput};

#[derive(Debug, Clone, Default)]
pub struct GovernanceConfigResult {
    pub proposal_fee: String,
    pub lost_proposal_fee: String,
    pub min_quorum: String,
    pub min_veto_threshold: String,
    pub min_pass_threshold: String,
    pub last_proposal_nonce: u64,
}

impl TopDecodeMulti for GovernanceConfigResult {
    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        let proposal_fee = String::multi_decode_or_handle_err(input, h)?;
        let lost_proposal_fee = String::multi_decode_or_handle_err(input, h)?;
        let min_quorum = String::multi_decode_or_handle_err(input, h)?;
        let min_veto_threshold = String::multi_decode_or_handle_err(input, h)?;
        let min_pass_threshold = String::multi_decode_or_handle_err(input, h)?;
        let last_proposal_nonce = u64::multi_decode_or_handle_err(input, h)?;
        Ok(GovernanceConfigResult {
            proposal_fee,
            lost_proposal_fee,
            min_quorum,
            min_veto_threshold,
            min_pass_threshold,
            last_proposal_nonce,
        })
    }
}
