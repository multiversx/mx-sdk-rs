use super::*;

#[derive(Debug)]
pub struct CheckLog {
    pub address: BytesValue,
    pub endpoint: BytesValue,
    pub topics: Vec<BytesValue>,
    pub data: BytesValue,
}

impl InterpretableFrom<CheckLogRaw> for CheckLog {
    fn interpret_from(from: CheckLogRaw, context: &InterpreterContext) -> Self {
        CheckLog {
            address: BytesValue::interpret_from(from.address, context),
            endpoint: BytesValue::interpret_from(from.endpoint, context),
            topics: from
                .topics
                .into_iter()
                .map(|t| BytesValue::interpret_from(t, context))
                .collect(),
            data: BytesValue::interpret_from(from.data, context),
        }
    }
}

#[derive(Debug)]
pub enum CheckLogs {
    Star,
    List(Vec<CheckLog>),
}

impl CheckLogs {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckLogs::Star)
    }
}

impl InterpretableFrom<CheckLogsRaw> for CheckLogs {
    fn interpret_from(from: CheckLogsRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckLogsRaw::Star => CheckLogs::Star,
            CheckLogsRaw::List(l) => CheckLogs::List(
                l.into_iter()
                    .map(|c| CheckLog::interpret_from(c, context))
                    .collect(),
            ),
            CheckLogsRaw::Unspecified => CheckLogs::Star,
        }
    }
}
