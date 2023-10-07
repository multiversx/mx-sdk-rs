use crate::{
    scenario::model::{CheckValue, U64Value},
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
        serde_raw::CheckEsdtDataRaw,
    },
    scenario_model::CheckValueList,
};

use super::CheckEsdtInstances;

#[derive(Debug, Default, Clone)]
pub struct CheckEsdtData {
    pub instances: CheckEsdtInstances,
    pub last_nonce: CheckValue<U64Value>,
    pub frozen: CheckValue<U64Value>,
    pub roles: CheckValueList,
}

impl InterpretableFrom<CheckEsdtDataRaw> for CheckEsdtData {
    fn interpret_from(from: CheckEsdtDataRaw, context: &InterpreterContext) -> Self {
        CheckEsdtData {
            instances: CheckEsdtInstances::interpret_from(from.instances, context),
            last_nonce: CheckValue::<U64Value>::interpret_from(from.last_nonce, context),
            frozen: CheckValue::<U64Value>::interpret_from(from.frozen, context),
            roles: CheckValueList::interpret_from(from.roles, context),
        }
    }
}

impl IntoRaw<CheckEsdtDataRaw> for CheckEsdtData {
    fn into_raw(self) -> CheckEsdtDataRaw {
        CheckEsdtDataRaw {
            instances: self.instances.into_raw(),
            last_nonce: self.last_nonce.into_raw(),
            roles: self.roles.into_raw(),
            frozen: self.frozen.into_raw(),
        }
    }
}
