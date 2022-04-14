use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    model::{AddressValue, BigUintValue, BytesKey, BytesValue, Esdt, U64Value},
    serde_raw::AccountRaw,
};

use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct Account {
    pub comment: Option<String>,
    pub nonce: Option<U64Value>,
    pub balance: Option<BigUintValue>,
    pub esdt: BTreeMap<BytesKey, Esdt>,
    pub username: Option<BytesValue>,
    pub storage: BTreeMap<BytesKey, BytesValue>,
    pub code: Option<BytesValue>,
    pub owner: Option<AddressValue>,
}

impl Account {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn nonce<V>(mut self, nonce: V) -> Self
    where
        U64Value: InterpretableFrom<V>,
    {
        self.nonce = Some(U64Value::interpret_from(
            nonce,
            &InterpreterContext::default(),
        ));
        self
    }

    pub fn balance<V>(mut self, balance_expr: V) -> Self
    where
        BigUintValue: InterpretableFrom<V>,
    {
        self.balance = Some(BigUintValue::interpret_from(
            balance_expr,
            &InterpreterContext::default(),
        ));
        self
    }
}

impl InterpretableFrom<AccountRaw> for Account {
    fn interpret_from(from: AccountRaw, context: &InterpreterContext) -> Self {
        Account {
            comment: from.comment,
            nonce: from.nonce.map(|n| U64Value::interpret_from(n, context)),
            balance: from
                .balance
                .map(|b| BigUintValue::interpret_from(b, context)),
            esdt: from
                .esdt
                .into_iter()
                .map(|(k, v)| {
                    (
                        BytesKey::interpret_from(k, context),
                        Esdt::interpret_from(v, context),
                    )
                })
                .collect(),
            username: from
                .username
                .map(|c| BytesValue::interpret_from(c, context)),
            storage: from
                .storage
                .into_iter()
                .map(|(k, v)| {
                    (
                        BytesKey::interpret_from(k, context),
                        BytesValue::interpret_from(v, context),
                    )
                })
                .collect(),
            code: from.code.map(|c| BytesValue::interpret_from(c, context)),
            owner: from.owner.map(|v| AddressValue::interpret_from(v, context)),
        }
    }
}

impl IntoRaw<AccountRaw> for Account {
    fn into_raw(self) -> AccountRaw {
        AccountRaw {
            comment: self.comment,
            nonce: self.nonce.map(|n| n.original),
            balance: self.balance.map(|n| n.original),
            esdt: self
                .esdt
                .into_iter()
                .map(|(k, v)| (k.original, v.into_raw()))
                .collect(),
            username: self.username.map(|n| n.original),
            storage: self
                .storage
                .into_iter()
                .map(|(key, value)| (key.original, value.original))
                .collect(),
            code: self.code.map(|n| n.original),
            owner: self.owner.map(|n| n.original),
        }
    }
}
