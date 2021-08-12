use super::*;
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum CheckEsdt {
    Star,
    Equal(BTreeMap<BytesKey, CheckValue<BigUintValue>>),
}

impl InterpretableFrom<CheckEsdtRaw> for CheckEsdt {
    fn interpret_from(from: CheckEsdtRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckEsdtRaw::Unspecified => CheckEsdt::Equal(BTreeMap::new()),
            CheckEsdtRaw::Star => CheckEsdt::Star,
            CheckEsdtRaw::Equal(m) => CheckEsdt::Equal(
                m.into_iter()
                    .map(|(k, v)| {
                        (
                            BytesKey::interpret_from(k, context),
                            CheckValue::<BigUintValue>::interpret_from(v, context),
                        )
                    })
                    .collect(),
            ),
        }
    }
}

impl CheckEsdt {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckEsdt::Star)
    }
}
