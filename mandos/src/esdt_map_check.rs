use crate::value_key_bytes::BytesKey;

use super::*;
use std::collections::BTreeMap;
#[derive(Debug)]
pub enum CheckEsdtMap {
    Star,
    Equal(CheckEsdtMapContents),
}

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

impl InterpretableFrom<CheckEsdtMapRaw> for CheckEsdtMap {
    fn interpret_from(from: CheckEsdtMapRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckEsdtMapRaw::Unspecified => CheckEsdtMap::Star,
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
