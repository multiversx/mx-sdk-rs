use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    model::{BytesValue, CheckLogs, CheckValue, CheckValueList, U64Value},
    serde_raw::TxExpectRaw,
};

#[derive(Debug)]
pub struct TxExpect {
    pub out: CheckValueList,
    pub status: CheckValue<U64Value>,
    pub message: CheckValue<BytesValue>,
    pub logs: CheckLogs,
    pub gas: CheckValue<U64Value>,
    pub refund: CheckValue<U64Value>,
}

impl TxExpect {
    pub fn ok() -> Self {
        TxExpect {
            out: CheckValue::Star,
            status: CheckValue::Equal(U64Value::zero()),
            message: CheckValue::Star,
            logs: CheckLogs::Star,
            gas: CheckValue::Star,
            refund: CheckValue::Star,
        }
    }

    pub fn err<S, E>(status_code_expr: S, err_msg_expr: E) -> Self
    where
        U64Value: InterpretableFrom<S>,
        BytesValue: InterpretableFrom<E>,
    {
        let ctx = InterpreterContext::default();
        let status_code = U64Value::interpret_from(status_code_expr, &ctx);
        let err_msg = BytesValue::interpret_from(err_msg_expr, &ctx);

        TxExpect {
            out: CheckValue::Star,
            status: CheckValue::Equal(status_code),
            message: CheckValue::Equal(err_msg),
            logs: CheckLogs::Star,
            gas: CheckValue::Star,
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
            gas: CheckValue::<U64Value>::interpret_from(from.gas, context),
            refund: CheckValue::<U64Value>::interpret_from(from.refund, context),
        }
    }
}

impl IntoRaw<TxExpectRaw> for TxExpect {
    fn into_raw(self) -> TxExpectRaw {
        TxExpectRaw {
            out: self.out.into_raw(),
            status: self.status.into_raw_explicit(),
            message: self.message.into_raw(),
            logs: self.logs.into_raw(),
            gas: self.gas.into_raw(),
            refund: self.refund.into_raw(),
        }
    }
}

impl TxExpect {
    pub fn out_to_string(&self) -> String {
        match &self.out {
            CheckValue::Star => "*".to_string(),
            CheckValue::Equal(list) => {
                itertools::join(list.iter().map(|val| format!("{}", val)), ", ")
            },
        }
    }
}
