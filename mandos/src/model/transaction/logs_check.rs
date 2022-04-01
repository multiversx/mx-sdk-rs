use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    serde_raw::{CheckLogListRaw, CheckLogsRaw},
};

use super::CheckLog;

#[derive(Debug)]
pub struct CheckLogList {
    pub list: Vec<CheckLog>,
    pub more_allowed_at_end: bool,
}

impl InterpretableFrom<CheckLogListRaw> for CheckLogList {
    fn interpret_from(from: CheckLogListRaw, context: &InterpreterContext) -> Self {
        CheckLogList {
            list: from
                .list
                .into_iter()
                .map(|c| CheckLog::interpret_from(c, context))
                .collect(),
            more_allowed_at_end: from.more_allowed_at_end,
        }
    }
}

#[derive(Debug)]
pub enum CheckLogs {
    Star,
    List(CheckLogList),
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
            CheckLogsRaw::List(l) => CheckLogs::List(CheckLogList::interpret_from(l, context)),
            CheckLogsRaw::Unspecified => CheckLogs::Star,
        }
    }
}
