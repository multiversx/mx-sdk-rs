use crate::{
    AddressValue, BigUintValue, BytesValue, InterpretableFrom, InterpreterContext, TxDeployRaw,
    U64Value,
};

#[derive(Debug)]
pub struct TxDeploy {
    pub from: AddressValue,
    pub call_value: BigUintValue,
    pub contract_code: BytesValue,
    pub arguments: Vec<BytesValue>,
    pub gas_limit: U64Value,
    pub gas_price: U64Value,
}

impl InterpretableFrom<TxDeployRaw> for TxDeploy {
    fn interpret_from(from: TxDeployRaw, context: &InterpreterContext) -> Self {
        TxDeploy {
            from: AddressValue::interpret_from(from.from, context),
            call_value: BigUintValue::interpret_from(from.value, context),
            contract_code: BytesValue::interpret_from(from.contract_code, context),
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
