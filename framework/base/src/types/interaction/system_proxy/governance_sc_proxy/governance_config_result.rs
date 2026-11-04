use alloc::string::String;

use multiversx_sc_codec::{DecodeError, DecodeErrorHandler, TopDecodeMulti, TopDecodeMultiInput};

/// The decoded result of the `viewConfig` view of the governance system SC.
///
/// All values are returned by the system SC as strings.
#[derive(Debug, Clone, Default)]
pub struct GovernanceConfigResult {
    /// The fee for submitting a proposal, in EGLD atto-units, as a base 10 string.
    pub proposal_fee: String,
    /// The part of the proposal fee lost if a proposal is vetoed, in EGLD atto-units, as a base 10 string.
    pub lost_proposal_fee: String,
    /// Minimum quorum, as a fraction with 4 decimals (e.g. `"0.2000"` for 20%).
    pub min_quorum: String,
    /// Minimum veto threshold, as a fraction with 4 decimals (e.g. `"0.3300"` for 33%).
    pub min_veto_threshold: String,
    /// Minimum pass threshold, as a fraction with 4 decimals (e.g. `"0.5000"` for 50%).
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
        // the last proposal nonce is also returned as a base 10 string
        let last_proposal_nonce_str = String::multi_decode_or_handle_err(input, h)?;
        let last_proposal_nonce = last_proposal_nonce_str
            .parse::<u64>()
            .map_err(|_| h.handle_error(DecodeError::INVALID_VALUE))?;
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
