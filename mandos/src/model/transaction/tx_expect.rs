use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::{BytesValue, CheckLogs, CheckValue, U64Value},
    serde_raw::{CheckBytesValueRaw, TxExpectRaw},
};

#[derive(Debug)]
pub struct TxExpect {
    pub out: Vec<CheckValue<BytesValue>>,
    pub status: CheckValue<U64Value>,
    pub message: CheckValue<BytesValue>,
    pub logs: CheckLogs,
    pub gas: Option<CheckValue<U64Value>>,
    pub refund: CheckValue<U64Value>,
}

impl InterpretableFrom<TxExpectRaw> for TxExpect {
    fn interpret_from(from: TxExpectRaw, context: &InterpreterContext) -> Self {
        TxExpect {
            out: from
                .out
                .into_iter()
                .map(|t| CheckValue::<BytesValue>::interpret_from(t, context))
                .collect(),
            status: CheckValue::<U64Value>::interpret_from(from.status, context),
            logs: CheckLogs::interpret_from(from.logs, context),
            message: CheckValue::<BytesValue>::interpret_from(from.message, context),
            gas: if let CheckBytesValueRaw::Unspecified = from.gas {
                None // gas is an exception: by default it is "*" instead of "0"
            } else {
                Some(CheckValue::<U64Value>::interpret_from(from.gas, context))
            },
            refund: CheckValue::<U64Value>::interpret_from(from.refund, context),
        }
    }
}
