use crate::{
    AddressValue, BigUintValue, BytesValue, InterpretableFrom, InterpreterContext, TxCallRaw,
    U64Value,
};

use super::*;

#[derive(Debug)]
pub struct TxCall {
    pub from: AddressValue,
    pub to: AddressValue,
    pub call_value: BigUintValue,
    pub esdt_value: Option<TxESDT>,
    pub function: String,
    pub arguments: Vec<BytesValue>,
    pub gas_limit: U64Value,
    pub gas_price: U64Value,
}

impl InterpretableFrom<TxCallRaw> for TxCall {
    fn interpret_from(from: TxCallRaw, context: &InterpreterContext) -> Self {
        TxCall {
            from: AddressValue::interpret_from(from.from, context),
            to: AddressValue::interpret_from(from.to, context),
            call_value: BigUintValue::interpret_from(from.value, context),
            esdt_value: from
                .esdt
                .map(|esdt_value| TxESDT::interpret_from(esdt_value, context)),
            function: from.function,
            arguments: from
                .arguments
                .into_iter()
                .map(|t| BytesValue::interpret_from(t, context))
                .collect(),
            gas_limit: U64Value::interpret_from(from.gas_limit, context),
            gas_price: U64Value::interpret_from(from.gas_price, context),
        }
    }
}
