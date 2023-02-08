use crate::scenario_format::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::CheckEsdtMapRaw,
};

use super::CheckEsdtMapContents;

#[derive(Debug, Clone, Default)]
pub enum CheckEsdtMap {
    #[default]
    Unspecified,
    Star,
    Equal(CheckEsdtMapContents),
}

impl CheckEsdtMap {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckEsdtMap::Star)
    }
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

impl IntoRaw<CheckEsdtMapRaw> for CheckEsdtMap {
    fn into_raw(self) -> CheckEsdtMapRaw {
        match self {
            CheckEsdtMap::Unspecified => CheckEsdtMapRaw::Unspecified,
            CheckEsdtMap::Star => CheckEsdtMapRaw::Star,
            CheckEsdtMap::Equal(value) => CheckEsdtMapRaw::Equal(value.into_raw()),
        }
    }
}
