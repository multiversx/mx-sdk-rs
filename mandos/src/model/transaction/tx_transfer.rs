use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::{AddressValue, BigUintValue},
    serde_raw::TxTransferRaw,
};

use super::TxESDT;

#[derive(Debug)]
pub struct TxTransfer {
    pub from: AddressValue,
    pub to: AddressValue,
    pub value: BigUintValue,
    pub esdt_value: Option<TxESDT>,
}

impl InterpretableFrom<TxTransferRaw> for TxTransfer {
    fn interpret_from(from: TxTransferRaw, context: &InterpreterContext) -> Self {
        TxTransfer {
            from: AddressValue::interpret_from(from.from, context),
            to: AddressValue::interpret_from(from.to, context),
            value: BigUintValue::interpret_from(from.value, context),
            esdt_value: from
                .esdt
                .map(|esdt_value| TxESDT::interpret_from(esdt_value, context)),
        }
    }
}
