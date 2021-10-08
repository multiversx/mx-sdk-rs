use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::{AddressValue, BigUintValue},
    serde_raw::TxValidatorRewardRaw,
};

#[derive(Debug)]
pub struct TxValidatorReward {
    pub to: AddressValue,
    pub value: BigUintValue,
}

impl InterpretableFrom<TxValidatorRewardRaw> for TxValidatorReward {
    fn interpret_from(from: TxValidatorRewardRaw, context: &InterpreterContext) -> Self {
        TxValidatorReward {
            to: AddressValue::interpret_from(from.to, context),
            value: BigUintValue::interpret_from(from.value, context),
        }
    }
}
