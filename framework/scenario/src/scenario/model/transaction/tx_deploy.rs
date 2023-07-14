use crate::{
    multiversx_sc::types::CodeMetadata,
    scenario::model::{AddressValue, BigUintValue, BytesValue, U64Value},
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
        serde_raw::TxDeployRaw,
    },
};

use super::{tx_interpret_util::interpret_egld_value, DEFAULT_GAS_EXPR};

#[derive(Debug, Clone)]
pub struct TxDeploy {
    pub from: AddressValue,
    pub egld_value: BigUintValue,
    pub code_metadata: CodeMetadata,
    pub contract_code: BytesValue,
    pub arguments: Vec<BytesValue>,
    pub gas_limit: U64Value,
    pub gas_price: U64Value,
}

impl Default for TxDeploy {
    fn default() -> Self {
        Self {
            from: Default::default(),
            egld_value: Default::default(),
            code_metadata: CodeMetadata::all(),
            contract_code: Default::default(),
            arguments: Default::default(),
            gas_limit: U64Value::from(DEFAULT_GAS_EXPR),
            gas_price: Default::default(),
        }
    }
}

impl InterpretableFrom<TxDeployRaw> for TxDeploy {
    fn interpret_from(from: TxDeployRaw, context: &InterpreterContext) -> Self {
        TxDeploy {
            from: AddressValue::interpret_from(from.from, context),
            egld_value: interpret_egld_value(from.value, from.egld_value, context),
            code_metadata: CodeMetadata::empty(), // not yet modelled in scenarios
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

impl TxDeploy {
    pub fn to_tx_data(&self) -> String {
        let mut result = hex::encode(&self.contract_code.value);
        result.push_str("@0500@"); // VM identifier
        result.push_str(hex::encode(self.code_metadata.to_byte_array()).as_str());
        for argument in &self.arguments {
            result.push('@');
            result.push_str(hex::encode(argument.value.as_slice()).as_str());
        }

        result
    }
}
