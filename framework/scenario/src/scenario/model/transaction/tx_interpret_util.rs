use crate::{
    scenario::model::BigUintValue,
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext},
        serde_raw::ValueSubTree,
    },
};

pub fn interpret_egld_value(
    opt_legacy_value: Option<ValueSubTree>,
    opt_egld_value: Option<ValueSubTree>,
    context: &InterpreterContext,
) -> BigUintValue {
    let mut egld_value = BigUintValue::default();
    if let Some(parsed_legacy_value) = opt_legacy_value {
        egld_value = BigUintValue::interpret_from(parsed_legacy_value, context);
    }
    if let Some(parsed_egld_value) = opt_egld_value {
        egld_value = BigUintValue::interpret_from(parsed_egld_value, context);
    }
    egld_value
}
