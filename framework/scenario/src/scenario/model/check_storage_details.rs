use crate::scenario_format::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::CheckStorageDetailsRaw,
};

use std::collections::BTreeMap;

use super::{BytesKey, BytesValue, CheckValue};

#[derive(Debug, Default, Clone)]
pub struct CheckStorageDetails {
    pub storages: BTreeMap<BytesKey, CheckValue<BytesValue>>,
    pub other_storages_allowed: bool,
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

impl IntoRaw<CheckStorageDetailsRaw> for CheckStorageDetails {
    fn into_raw(self) -> CheckStorageDetailsRaw {
        CheckStorageDetailsRaw {
            storages: self
                .storages
                .into_iter()
                .map(|(k, v)| (k.into_raw(), v.into_raw_explicit()))
                .collect(),
            other_storages_allowed: self.other_storages_allowed,
        }
    }
}
