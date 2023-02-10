use crate::{
    scenario::model::BytesKey,
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
        serde_raw::CheckEsdtMapContentsRaw,
    },
};

use std::collections::BTreeMap;

use super::CheckEsdt;

#[derive(Debug, Clone)]
pub struct CheckEsdtMapContents {
    pub contents: BTreeMap<BytesKey, CheckEsdt>,
    pub other_esdts_allowed: bool,
}

impl CheckEsdtMapContents {
    pub fn contains_token(&self, token_identifier: &[u8]) -> bool {
        let token_id_conv = BytesKey::from(token_identifier.to_vec());
        self.contents.contains_key(&token_id_conv)
    }
}

impl InterpretableFrom<CheckEsdtMapContentsRaw> for CheckEsdtMapContents {
    fn interpret_from(from: CheckEsdtMapContentsRaw, context: &InterpreterContext) -> Self {
        CheckEsdtMapContents {
            contents: from
                .contents
                .into_iter()
                .map(|(k, v)| {
                    (
                        BytesKey::interpret_from(k, context),
                        CheckEsdt::interpret_from(v, context),
                    )
                })
                .collect(),
            other_esdts_allowed: from.other_esdts_allowed,
        }
    }
}

impl IntoRaw<CheckEsdtMapContentsRaw> for CheckEsdtMapContents {
    fn into_raw(self) -> CheckEsdtMapContentsRaw {
        CheckEsdtMapContentsRaw {
            contents: self
                .contents
                .into_iter()
                .map(|(k, v)| (k.into_raw(), v.into_raw()))
                .collect(),
            other_esdts_allowed: self.other_esdts_allowed,
        }
    }
}
