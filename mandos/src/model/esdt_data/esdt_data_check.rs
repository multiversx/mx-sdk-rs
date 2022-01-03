use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::{CheckValue, U64Value},
    serde_raw::CheckEsdtDataRaw,
};

use super::CheckEsdtInstances;

#[derive(Debug, Default)]
pub struct CheckEsdtData {
    pub instances: CheckEsdtInstances,
    pub last_nonce: CheckValue<U64Value>,
    pub frozen: CheckValue<U64Value>,
}

impl InterpretableFrom<CheckEsdtDataRaw> for CheckEsdtData {
    fn interpret_from(from: CheckEsdtDataRaw, context: &InterpreterContext) -> Self {
        CheckEsdtData {
            instances: CheckEsdtInstances::interpret_from(from.instances, context),
            last_nonce: CheckValue::<U64Value>::interpret_from(from.last_nonce, context),
            frozen: CheckValue::<U64Value>::interpret_from(from.frozen, context),
        }
    }
}
