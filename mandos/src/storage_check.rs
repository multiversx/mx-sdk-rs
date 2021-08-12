use super::*;
use std::collections::BTreeMap;
#[derive(Debug)]
pub enum CheckStorage {
    Star,
    Equal(CheckStorageDetails),
}

#[derive(Debug)]
pub struct CheckStorageDetails {
    pub storages: BTreeMap<BytesKey, CheckValue<BytesValue>>,
    pub other_storages_allowed: bool,
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

impl CheckStorage {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckStorage::Star)
    }
}

impl InterpretableFrom<CheckStorageDetailsRaw> for CheckStorageDetails {
    fn interpret_from(from: CheckStorageDetailsRaw, context: &InterpreterContext) -> Self {
        CheckStorageDetails {
            storages: from
                .storages
                .into_iter()
                .map(|(k, v)| {
                    (
                        BytesKey::interpret_from(k, context),
                        CheckValue::<BytesValue>::interpret_from(v, context),
                    )
                })
                .collect(),
            other_storages_allowed: from.other_storages_allowed,
        }
    }
}
