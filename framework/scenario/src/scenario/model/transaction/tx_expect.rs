use super::TxResponse;
use crate::{
    scenario::model::{BytesValue, CheckLogs, CheckValue, CheckValueList, U64Value},
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
        serde_raw::TxExpectRaw,
    },
    scenario_model::Checkable,
};
use multiversx_chain_vm::tx_mock::result_values_to_string;

const USER_ERROR_CODE: u64 = 4;

#[derive(Debug, Clone)]
pub struct TxExpect {
    pub out: CheckValueList,
    pub status: CheckValue<U64Value>,
    pub message: CheckValue<BytesValue>,
    pub logs: CheckLogs,
    pub gas: CheckValue<U64Value>,
    pub refund: CheckValue<U64Value>,
    pub build_from_response: bool,
    pub additional_error_message: String,
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
            build_from_response: true,
            additional_error_message: Default::default(),
        }
    }

    pub fn err<S, E>(status_code_expr: S, err_msg_expr: E) -> Self
    where
        U64Value: From<S>,
        BytesValue: From<E>,
    {
        let status_code = U64Value::from(status_code_expr);
        let err_msg = BytesValue::from(err_msg_expr);

        TxExpect {
            out: CheckValue::Star,
            status: CheckValue::Equal(status_code),
            message: CheckValue::Equal(err_msg),
            logs: CheckLogs::Star,
            gas: CheckValue::Star,
            refund: CheckValue::Star,
            build_from_response: true,
            additional_error_message: Default::default(),
        }
    }

    pub fn user_error<E>(err_msg_expr: E) -> Self
    where
        BytesValue: From<E>,
    {
        Self::err(USER_ERROR_CODE, err_msg_expr)
    }

    pub fn no_result(mut self) -> Self {
        self.out = CheckValue::Equal(Vec::new());
        self.build_from_response = false;
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
        self.build_from_response = false;
        self
    }

    pub fn additional_error_message<A>(mut self, message: A) -> Self
    where
        A: AsRef<str>,
    {
        self.additional_error_message = message.as_ref().to_string();
        self
    }

    fn check_response(&self, tx_response: &TxResponse) {
        assert!(
            self.status.check(tx_response.tx_error.status),
            "{}result code mismatch. Want: {}. Have: {}. Message: {}",
            &self.additional_error_message,
            self.status,
            tx_response.tx_error.status,
            &tx_response.tx_error.message,
        );

        assert!(
            self.out.check(&tx_response.out),
            "{}bad out value. Want: [{}]. Have: [{}]",
            &self.additional_error_message,
            self.out_to_string(),
            result_values_to_string(&tx_response.out),
        );

        assert!(
            self.message.check(tx_response.tx_error.message.as_str()),
            "{}result message mismatch. Want: {}. Have: {}.",
            &self.additional_error_message,
            &self.status,
            &tx_response.tx_error.message,
        );
    }

    pub(crate) fn update_from_response(&mut self, tx_response: &TxResponse) {
        if self.build_from_response {
            self.check_response(tx_response);
            *self = tx_response.to_expect();
        }
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
            build_from_response: false,
            additional_error_message: Default::default(),
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
                itertools::join(list.iter().map(|val| format!("{val}")), ", ")
            },
        }
    }
}
