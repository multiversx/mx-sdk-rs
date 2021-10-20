use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::{AddressValue, BigUintValue},
    serde_raw::TxTransferRaw,
};

use super::{tx_interpret_util::interpret_egld_value, TxESDT};

#[derive(Debug)]
pub struct TxTransfer {
    pub from: AddressValue,
    pub to: AddressValue,
    pub egld_value: BigUintValue,
    pub esdt_value: Vec<TxESDT>,
}

impl InterpretableFrom<TxTransferRaw> for TxTransfer {
    fn interpret_from(from: TxTransferRaw, context: &InterpreterContext) -> Self {
        TxTransfer {
            from: AddressValue::interpret_from(from.from, context),
            to: AddressValue::interpret_from(from.to, context),
            egld_value: interpret_egld_value(from.value, from.egld_value, context),
            esdt_value: from
                .esdt_value
                .iter()
                .map(|esdt_value| TxESDT::interpret_from(esdt_value.clone(), context))
                .collect(),
        }
    }
}
