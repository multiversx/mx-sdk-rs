use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::{BigUintValue, BytesValue, U64Value},
    serde_raw::{TxESDTRaw, ValueSubTree},
};

#[derive(Debug)]
pub struct TxESDT {
    pub esdt_token_identifier: BytesValue,
    pub nonce: U64Value,
    pub esdt_value: BigUintValue,
}

impl InterpretableFrom<TxESDTRaw> for TxESDT {
    fn interpret_from(from: TxESDTRaw, context: &InterpreterContext) -> Self {
        TxESDT {
            esdt_token_identifier: interpret_esdt_token_identifier(from.token_identifier, context),
            nonce: interpret_opt_u64(from.nonce, context),
            esdt_value: BigUintValue::interpret_from(from.value, context),
        }
    }
}

fn interpret_esdt_token_identifier(
    esdt_token_identifier: Option<ValueSubTree>,
    context: &InterpreterContext,
) -> BytesValue {
    if let Some(esdt_token_identifier_raw) = esdt_token_identifier {
        BytesValue::interpret_from(esdt_token_identifier_raw, context)
    } else {
        BytesValue::empty()
    }
}

fn interpret_opt_u64(opt_u64: Option<ValueSubTree>, context: &InterpreterContext) -> U64Value {
    if let Some(u) = opt_u64 {
        U64Value::interpret_from(u, context)
    } else {
        U64Value::empty()
    }
}
