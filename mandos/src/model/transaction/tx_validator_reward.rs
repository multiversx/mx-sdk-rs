use crate::{
    AddressValue, BigUintValue, InterpretableFrom, InterpreterContext, TxValidatorRewardRaw,
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
