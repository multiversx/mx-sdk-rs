use crate::{
    scenario::model::{AddressValue, BigUintValue},
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
        serde_raw::TxValidatorRewardRaw,
    },
};

use super::tx_interpret_util::interpret_egld_value;

#[derive(Debug, Clone)]
pub struct TxValidatorReward {
    pub to: AddressValue,
    pub egld_value: BigUintValue,
}

impl InterpretableFrom<TxValidatorRewardRaw> for TxValidatorReward {
    fn interpret_from(from: TxValidatorRewardRaw, context: &InterpreterContext) -> Self {
        TxValidatorReward {
            to: AddressValue::interpret_from(from.to, context),
            egld_value: interpret_egld_value(from.value, from.egld_value, context),
        }
    }
}

impl IntoRaw<TxValidatorRewardRaw> for TxValidatorReward {
    fn into_raw(self) -> TxValidatorRewardRaw {
        TxValidatorRewardRaw {
            to: self.to.into_raw(),
            value: None,
            egld_value: Some(self.egld_value.into_raw()),
        }
    }
}
