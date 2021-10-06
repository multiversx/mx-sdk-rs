use super::*;

#[derive(Debug)]
pub enum CheckEsdt {
    Star,
    Equal(Vec<CheckEsdtData>),
}

#[derive(Debug, Default)]
pub struct CheckEsdtData {
    pub token_identifier: BytesValue,
    pub instances: CheckEsdtInstances,
    pub last_nonce: CheckValue<U64Value>,
    pub roles: CheckValue<BytesValue>,
    pub frozen: CheckValue<U64Value>,
}

#[derive(Debug)]
pub enum CheckEsdtInstances {
    Star,
    Equal(Vec<CheckEsdtValue>),
}

#[derive(Debug)]
pub struct CheckEsdtValue {
    pub nonce: U64Value,
    pub balance: CheckValue<BigUintValue>,
    pub creator: CheckValue<BytesValue>,
    pub royalties: CheckValue<U64Value>,
    pub hash: CheckValue<BytesValue>,
    pub uri: CheckValue<BytesValue>,
    pub attributes: CheckValue<BytesValue>,
}

impl CheckEsdt {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckEsdt::Star)
    }

    pub fn contains_identifier(&self, identifier: &Vec<u8>) -> bool {
        match self {
            CheckEsdt::Star => return false,
            CheckEsdt::Equal(x) => {
                for item in x {
                    if item.token_identifier.check(identifier) {
                        return true;
                    }
                }
            },
        }
        false
    }
}

impl InterpretableFrom<CheckEsdtRaw> for CheckEsdt {
    fn interpret_from(from: CheckEsdtRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckEsdtRaw::Unspecified => CheckEsdt::Equal(Vec::new()),
            CheckEsdtRaw::Star => CheckEsdt::Star,
            CheckEsdtRaw::Equal(m) => CheckEsdt::Equal(
                m.into_iter()
                    .map(|v| (CheckEsdtData::interpret_from(v, context)))
                    .collect(),
            ),
        }
    }
}

impl InterpretableFrom<CheckEsdtDataRaw> for CheckEsdtData {
    fn interpret_from(from: CheckEsdtDataRaw, context: &InterpreterContext) -> Self {
        CheckEsdtData {
            token_identifier: BytesValue::interpret_from(from.token_identifier, context),
            instances: CheckEsdtInstances::interpret_from(from.instances, context),
            last_nonce: CheckValue::<U64Value>::interpret_from(from.last_nonce, context),
            roles: CheckValue::<BytesValue>::interpret_from(from.roles, context),
            frozen: CheckValue::<U64Value>::interpret_from(from.frozen, context),
        }
    }
}

impl CheckEsdtInstances {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckEsdtInstances::Star)
    }
}

impl InterpretableFrom<CheckEsdtInstancesRaw> for CheckEsdtInstances {
    fn interpret_from(from: CheckEsdtInstancesRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckEsdtInstancesRaw::Unspecified => CheckEsdtInstances::Equal(Vec::new()),
            CheckEsdtInstancesRaw::Star => CheckEsdtInstances::Star,
            CheckEsdtInstancesRaw::Equal(m) => CheckEsdtInstances::Equal(
                m.into_iter()
                    .map(|v| CheckEsdtValue::interpret_from(v, context))
                    .collect(),
            ),
        }
    }
}

impl Default for CheckEsdtInstances {
    fn default() -> Self {
        CheckEsdtInstances::Equal(Vec::new())
    }
}

impl InterpretableFrom<CheckEsdtValueRaw> for CheckEsdtValue {
    fn interpret_from(from: CheckEsdtValueRaw, context: &InterpreterContext) -> Self {
        CheckEsdtValue {
            nonce: U64Value::interpret_from(from.nonce, context),
            balance: CheckValue::<BigUintValue>::interpret_from(from.balance, context),
            creator: CheckValue::<BytesValue>::interpret_from(from.creator, context),
            royalties: CheckValue::<U64Value>::interpret_from(from.royalties, context),
            hash: CheckValue::<BytesValue>::interpret_from(from.hash, context),
            uri: CheckValue::<BytesValue>::interpret_from(from.uri, context),
            attributes: CheckValue::<BytesValue>::interpret_from(from.attributes, context),
        }
    }
}
