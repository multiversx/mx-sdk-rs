use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    serde_raw::CheckEsdtMapRaw,
};

use super::CheckEsdtMapContents;

#[derive(Debug)]
pub enum CheckEsdtMap {
    Unspecified,
    Star,
    Equal(CheckEsdtMapContents),
}

impl InterpretableFrom<CheckEsdtMapRaw> for CheckEsdtMap {
    fn interpret_from(from: CheckEsdtMapRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckEsdtMapRaw::Unspecified => CheckEsdtMap::Unspecified,
            CheckEsdtMapRaw::Star => CheckEsdtMap::Star,
            CheckEsdtMapRaw::Equal(m) => {
                CheckEsdtMap::Equal(CheckEsdtMapContents::interpret_from(m, context))
            },
        }
    }
}

impl CheckEsdtMap {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckEsdtMap::Star)
    }
}
