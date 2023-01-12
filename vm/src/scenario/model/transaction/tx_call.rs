use crate::{
    scenario::model::{AddressValue, BigUintValue, BytesValue, U64Value},
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
        serde_raw::TxCallRaw,
    },
    DebugApi,
};
use multiversx_sc::types::{ContractCall, ContractCallWithEgld, EsdtTokenPayment};

use super::{tx_interpret_util::interpret_egld_value, TxESDT};

#[derive(Debug, Default)]
pub struct TxCall {
    pub from: AddressValue,
    pub to: AddressValue,
    pub egld_value: BigUintValue,
    pub esdt_value: Vec<TxESDT>,
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
            egld_value: interpret_egld_value(from.value, from.egld_value, context),
            esdt_value: from
                .esdt_value
                .into_iter()
                .map(|esdt_value| TxESDT::interpret_from(esdt_value, context))
                .collect(),
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

impl IntoRaw<TxCallRaw> for TxCall {
    fn into_raw(self) -> TxCallRaw {
        TxCallRaw {
            from: self.from.into_raw(),
            to: self.to.into_raw(),
            value: None,
            egld_value: self.egld_value.into_raw_opt(),
            esdt_value: self
                .esdt_value
                .into_iter()
                .map(|esdt_value| esdt_value.into_raw())
                .collect(),
            function: self.function,
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

impl TxCall {
    pub fn to_contract_call(&self) -> ContractCallWithEgld<DebugApi, ()> {
        let mut contract_call = ContractCallWithEgld::new(
            (&self.to.value).into(),
            self.function.as_bytes(),
            (&self.egld_value.value).into(),
        )
        .convert_to_esdt_transfer_call(
            self.esdt_value
                .iter()
                .map(|esdt| {
                    EsdtTokenPayment::new(
                        esdt.esdt_token_identifier.value.as_slice().into(),
                        esdt.nonce.value,
                        (&esdt.esdt_value.value).into(),
                    )
                })
                .collect(),
        );

        for argument in &self.arguments {
            contract_call.push_raw_argument(argument.value.as_slice());
        }
        contract_call
    }
}
