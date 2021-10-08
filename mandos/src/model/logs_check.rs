use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    serde_raw::CheckLogsRaw,
};

use super::CheckLog;

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
