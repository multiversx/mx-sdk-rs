use super::*;
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct Esdt {
    pub token_identifier: Option<BytesValue>,
    pub instances: BTreeMap<BytesKey, Instance>,
    pub last_nonce: Option<U64Value>,
    pub roles: BTreeMap<BytesKey, BytesValue>,
    pub frozen: Option<U64Value>,
}

#[derive(Debug, Default)]
pub struct Instance {
    pub nonce: Option<U64Value>,
    pub balance: Option<BigUintValue>,
    pub creator: Option<BytesValue>,
    pub royalties: Option<U64Value>,
    pub hash: Option<BytesValue>,
    pub uri: Option<BytesValue>,
    pub attributes: Option<BytesValue>,
}

impl InterpretableFrom<EsdtRaw> for Esdt {
    fn interpret_from(from: EsdtRaw, context: &InterpreterContext) -> Self {
        Esdt {
            token_identifier: from
                .token_identifier
                .map(|b| BytesValue::interpret_from(b, context)),
            instances: from
                .instances
                .into_iter()
                .map(|(k, v)| {
                    (
                        BytesKey::interpret_from(k, context),
                        Instance::interpret_from(v, context),
                    )
                })
                .collect(),
            last_nonce: from
                .last_nonce
                .map(|b| U64Value::interpret_from(b, context)),
            roles: from
                .roles
                .iter()
                .map(|(k, b)| {
                    (
                        BytesKey::interpret_from(k.clone(), context),
                        BytesValue::interpret_from(b.clone(), context),
                    )
                })
                .collect(),
            frozen: from.frozen.map(|b| U64Value::interpret_from(b, context)),
        }
    }
}

impl InterpretableFrom<InstanceRaw> for Instance {
    fn interpret_from(from: InstanceRaw, context: &InterpreterContext) -> Self {
        Instance {
            nonce: from.nonce.map(|n| U64Value::interpret_from(n, context)),
            balance: from
                .balance
                .map(|b| BigUintValue::interpret_from(b, context)),
            creator: from.creator.map(|b| BytesValue::interpret_from(b, context)),
            royalties: from.royalties.map(|b| U64Value::interpret_from(b, context)),
            hash: from.hash.map(|b| BytesValue::interpret_from(b, context)),
            uri: from.uri.map(|b| BytesValue::interpret_from(b, context)),
            attributes: from
                .attributes
                .map(|b| BytesValue::interpret_from(b, context)),
        }
    }
}
