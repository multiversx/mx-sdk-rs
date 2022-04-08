use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    model::{AddressValue, BigUintValue, BytesValue, U64Value},
    serde_raw::TxDeployRaw,
};

use super::tx_interpret_util::interpret_egld_value;

#[derive(Debug, Default)]
pub struct TxDeploy {
    pub from: AddressValue,
    pub egld_value: BigUintValue,
    pub contract_code: BytesValue,
    pub arguments: Vec<BytesValue>,
    pub gas_limit: U64Value,
    pub gas_price: U64Value,
}

impl InterpretableFrom<TxDeployRaw> for TxDeploy {
    fn interpret_from(from: TxDeployRaw, context: &InterpreterContext) -> Self {
        TxDeploy {
            from: AddressValue::interpret_from(from.from, context),
            egld_value: interpret_egld_value(from.value, from.egld_value, context),
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

impl IntoRaw<TxDeployRaw> for TxDeploy {
    fn into_raw(self) -> TxDeployRaw {
        TxDeployRaw {
            from: self.from.into_raw(),
            value: None,
            egld_value: self.egld_value.into_raw_opt(),
            contract_code: self.contract_code.into_raw(),
            arguments: self
                .arguments
                .into_iter()
                .map(|arg| arg.into_raw())
                .collect(),
            gas_limit: self.gas_limit.into_raw(),
            gas_price: self.gas_price.into_raw(),
        }
    }
}
