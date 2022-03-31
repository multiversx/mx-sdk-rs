use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::{BytesValue, CheckLogs, CheckValue, CheckValueList, U64Value},
    serde_raw::{CheckBytesValueRaw, TxExpectRaw},
};

#[derive(Debug)]
pub struct TxExpect {
    pub out: CheckValueList,
    pub status: CheckValue<U64Value>,
    pub message: CheckValue<BytesValue>,
    pub logs: CheckLogs,
    pub gas: Option<CheckValue<U64Value>>,
    pub refund: CheckValue<U64Value>,
}

impl TxExpect {
    pub fn ok() -> Self {
        TxExpect {
            out: CheckValue::Star,
            status: CheckValue::Equal(U64Value::zero()),
            message: CheckValue::Star,
            logs: CheckLogs::Star,
            gas: None,
            refund: CheckValue::Star,
        }
    }

    pub fn no_result(mut self) -> Self {
        self.out = CheckValue::Equal(Vec::new());
        self
    }

    pub fn result(mut self, value: &str) -> Self {
        let mut check_results = match self.out {
            CheckValue::Star => Vec::new(),
            CheckValue::Equal(check_results) => check_results,
        };
        check_results.push(CheckValue::Equal(BytesValue::interpret_from(
            value,
            &InterpreterContext::default(),
        )));
        self.out = CheckValue::Equal(check_results);
        self
    }
}

impl InterpretableFrom<TxExpectRaw> for TxExpect {
    fn interpret_from(from: TxExpectRaw, context: &InterpreterContext) -> Self {
        TxExpect {
            out: CheckValueList::interpret_from(from.out, context),
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
