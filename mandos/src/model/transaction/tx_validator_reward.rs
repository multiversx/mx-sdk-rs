use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::{AddressValue, BigUintValue},
    serde_raw::TxValidatorRewardRaw,
};

use super::tx_interpret_util::interpret_egld_value;

#[derive(Debug)]
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
