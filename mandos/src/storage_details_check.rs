use crate::value_key_bytes::BytesKey;

use super::*;
use std::collections::BTreeMap;

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
