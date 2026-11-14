use alloc::vec::Vec;

use multiversx_chain_core::types::BLSKey;
use multiversx_sc_codec::{DecodeError, DecodeErrorHandler, TopDecodeMulti, TopDecodeMultiInput};

const STAKED_LABEL: &[u8] = b"staked";
const NOT_STAKED_LABEL: &[u8] = b"notStaked";
const UN_STAKED_LABEL: &[u8] = b"unStaked";

/// The decoded result of the `getAllNodeStates` view of a delegation contract.
///
/// The system SC returns a flat list, where each non-empty group of BLS keys is preceded by a label:
/// `"staked", key1, key2, "notStaked", key3, "unStaked", key4`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DelegationNodeStatesResult {
    pub staked: Vec<BLSKey>,
    pub not_staked: Vec<BLSKey>,
    pub un_staked: Vec<BLSKey>,
}

#[derive(Clone, Copy)]
enum NodeStateGroup {
    None,
    Staked,
    NotStaked,
    UnStaked,
}

impl TopDecodeMulti for DelegationNodeStatesResult {
    fn multi_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeMultiInput,
        H: DecodeErrorHandler,
    {
        let mut result = DelegationNodeStatesResult::default();
        let mut group = NodeStateGroup::None;

        while input.has_next() {
            let value = Vec::<u8>::multi_decode_or_handle_err(input, h)?;
            match value.as_slice() {
                STAKED_LABEL => group = NodeStateGroup::Staked,
                NOT_STAKED_LABEL => group = NodeStateGroup::NotStaked,
                UN_STAKED_LABEL => group = NodeStateGroup::UnStaked,
                _ => {
                    let key = BLSKey::from_vec(value)
                        .ok_or_else(|| h.handle_error(DecodeError::INVALID_VALUE))?;
                    match group {
                        NodeStateGroup::None => {
                            return Err(h.handle_error(DecodeError::from(
                                "BLS key found before node state label",
                            )));
                        }
                        NodeStateGroup::Staked => result.staked.push(key),
                        NodeStateGroup::NotStaked => result.not_staked.push(key),
                        NodeStateGroup::UnStaked => result.un_staked.push(key),
                    }
                }
            }
        }

        Ok(result)
    }
}
