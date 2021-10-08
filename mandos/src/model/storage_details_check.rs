use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::{BytesValue, CheckValue},
    serde_raw::CheckStorageDetailsRaw,
};

use std::collections::BTreeMap;

use super::BytesKey;

#[derive(Debug)]
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
