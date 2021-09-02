use super::*;
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum CheckEsdt {
    Star,
    Equal(CheckEsdtData),
}

#[derive(Debug)]
pub struct CheckEsdtValue {
    pub nonce: CheckValue<U64Value>,
    pub balance: CheckValue<BigUintValue>,
    pub creator: CheckValue<BytesValue>,
    pub royalties: CheckValue<U64Value>,
    pub hash: CheckValue<BytesValue>,
    pub uri: CheckValue<BytesValue>,
    pub attributes: CheckValue<BytesValue>,
}

impl InterpretableFrom<CheckEsdtRaw> for CheckEsdt {
    fn interpret_from(from: CheckEsdtRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckEsdtRaw::Unspecified => CheckEsdt::Equal(CheckEsdtData::default()),
            CheckEsdtRaw::Star => CheckEsdt::Star,
            CheckEsdtRaw::Equal(m) => CheckEsdt::Equal(CheckEsdtData::interpret_from(m, context)),
        }
    }
}

impl CheckEsdt {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckEsdt::Star)
    }
}

impl InterpretableFrom<CheckEsdtValueRaw> for CheckEsdtValue {
    fn interpret_from(from: CheckEsdtValueRaw, context: &InterpreterContext) -> Self {
        CheckEsdtValue {
            nonce: CheckValue::<U64Value>::interpret_from(from.nonce, context),
            balance: CheckValue::<BigUintValue>::interpret_from(from.balance, context),
            creator: CheckValue::<BytesValue>::interpret_from(from.creator, context),
            royalties: CheckValue::<U64Value>::interpret_from(from.royalties, context),
            hash: CheckValue::<BytesValue>::interpret_from(from.hash, context),
            uri: CheckValue::<BytesValue>::interpret_from(from.uri, context),
            attributes: CheckValue::<BytesValue>::interpret_from(from.attributes, context),
        }
    }
}

#[derive(Debug)]
pub enum CheckEsdtValues {
    Star,
    Equal(BTreeMap<BytesKey, CheckEsdtValue>),
}

impl InterpretableFrom<CheckEsdtValuesRaw> for CheckEsdtValues {
    fn interpret_from(from: CheckEsdtValuesRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckEsdtValuesRaw::Unspecified => CheckEsdtValues::Equal(BTreeMap::new()),
            CheckEsdtValuesRaw::Star => CheckEsdtValues::Star,
            CheckEsdtValuesRaw::Equal(m) => CheckEsdtValues::Equal(
                m.into_iter()
                    .map(|(k, v)| {
                        (
                            BytesKey::interpret_from(k, context),
                            CheckEsdtValue::interpret_from(v, context),
                        )
                    })
                    .collect(),
            ),
        }
    }
}

impl Default for CheckEsdtValues {
    fn default() -> Self {
        CheckEsdtValues::Equal(BTreeMap::new())
    }
}

#[derive(Debug, Default)]
pub struct CheckEsdtData {
    pub token_identifier: CheckValue<BytesValue>,
    pub instances: CheckEsdtValues,
    pub last_nonce: CheckValue<U64Value>,
    pub roles: CheckValue<BytesValue>,
    pub frozen: CheckValue<U64Value>,
}

impl InterpretableFrom<CheckEsdtDataRaw> for CheckEsdtData {
    fn interpret_from(from: CheckEsdtDataRaw, context: &InterpreterContext) -> Self {
        CheckEsdtData {
            token_identifier: CheckValue::<BytesValue>::interpret_from(
                from.token_identifier,
                context,
            ),
            instances: CheckEsdtValues::interpret_from(from.instances, context),
            last_nonce: CheckValue::<U64Value>::interpret_from(from.last_nonce, context),
            roles: CheckValue::<BytesValue>::interpret_from(from.roles, context),
            frozen: CheckValue::<U64Value>::interpret_from(from.frozen, context),
        }
    }
}
