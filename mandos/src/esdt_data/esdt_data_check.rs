use crate::{
    BytesValue, CheckEsdtDataRaw, CheckValue, InterpretableFrom, InterpreterContext, U64Value,
};

use super::*;

#[derive(Debug, Default)]
pub struct CheckEsdtData {
    pub instances: CheckEsdtInstances,
    pub last_nonce: CheckValue<U64Value>,
    pub roles: CheckValue<BytesValue>,
    pub frozen: CheckValue<U64Value>,
}

impl InterpretableFrom<CheckEsdtDataRaw> for CheckEsdtData {
    fn interpret_from(from: CheckEsdtDataRaw, context: &InterpreterContext) -> Self {
        CheckEsdtData {
            instances: CheckEsdtInstances::interpret_from(from.instances, context),
            last_nonce: CheckValue::<U64Value>::interpret_from(from.last_nonce, context),
            roles: CheckValue::<BytesValue>::interpret_from(from.roles, context),
            frozen: CheckValue::<U64Value>::interpret_from(from.frozen, context),
        }
    }
}
