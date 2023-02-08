use crate::scenario_format::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::CheckStorageRaw,
};

use super::CheckStorageDetails;

#[derive(Debug, Clone, Default)]
pub enum CheckStorage {
    #[default]
    Star,
    Equal(CheckStorageDetails),
}

impl CheckStorage {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckStorage::Star)
    }
}

impl InterpretableFrom<CheckStorageRaw> for CheckStorage {
    fn interpret_from(from: CheckStorageRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckStorageRaw::Star => CheckStorage::Star,
            CheckStorageRaw::Equal(m) => {
                CheckStorage::Equal(CheckStorageDetails::interpret_from(m, context))
            },
        }
    }
}

impl IntoRaw<CheckStorageRaw> for CheckStorage {
    fn into_raw(self) -> CheckStorageRaw {
        match self {
            CheckStorage::Star => CheckStorageRaw::Star,
            CheckStorage::Equal(details) => CheckStorageRaw::Equal(details.into_raw()),
        }
    }
}
