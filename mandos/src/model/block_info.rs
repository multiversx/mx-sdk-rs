use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    serde_raw::BlockInfoRaw,
};

use super::{BytesValue, U64Value};

#[derive(Debug)]
pub struct BlockInfo {
    pub block_timestamp: Option<U64Value>,
    pub block_nonce: Option<U64Value>,
    pub block_round: Option<U64Value>,
    pub block_epoch: Option<U64Value>,
    pub block_random_seed: Option<BytesValue>,
}

impl InterpretableFrom<BlockInfoRaw> for BlockInfo {
    fn interpret_from(from: BlockInfoRaw, context: &InterpreterContext) -> Self {
        BlockInfo {
            block_timestamp: from
                .block_timestamp
                .map(|v| U64Value::interpret_from(v, context)),
            block_nonce: from
                .block_nonce
                .map(|v| U64Value::interpret_from(v, context)),
            block_round: from
                .block_round
                .map(|v| U64Value::interpret_from(v, context)),
            block_epoch: from
                .block_epoch
                .map(|v| U64Value::interpret_from(v, context)),
            block_random_seed: from
                .block_random_seed
                .map(|v| BytesValue::interpret_from(v, context)),
        }
    }
}
