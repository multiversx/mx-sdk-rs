use crate::scenario_format::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::BlockInfoRaw,
};

use super::{BytesValue, U64Value};

#[derive(Debug, Default, Clone)]
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

impl IntoRaw<BlockInfoRaw> for BlockInfo {
    fn into_raw(self) -> BlockInfoRaw {
        BlockInfoRaw {
            block_timestamp: self.block_timestamp.map(|value| value.original),
            block_nonce: self.block_nonce.map(|value| value.original),
            block_round: self.block_round.map(|value| value.original),
            block_epoch: self.block_epoch.map(|value| value.original),
            block_random_seed: self.block_random_seed.map(|value| value.original),
        }
    }
}
