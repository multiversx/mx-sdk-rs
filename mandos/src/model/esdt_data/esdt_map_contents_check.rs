use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::BytesKey,
    serde_raw::CheckEsdtMapContentsRaw,
};

use std::collections::BTreeMap;

use super::CheckEsdt;

#[derive(Debug)]
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
