use crate::{
    scenario::model::{
        AddressValue, BigUintValue, BytesKey, BytesValue, Esdt, EsdtObject, U64Value,
    },
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
        serde_raw::AccountRaw,
    },
};

use std::collections::BTreeMap;

#[derive(Debug, Default, Clone)]
pub struct Account {
    pub comment: Option<String>,
    pub nonce: Option<U64Value>,
    pub balance: Option<BigUintValue>,
    pub esdt: BTreeMap<BytesKey, Esdt>,
    pub username: Option<BytesValue>,
    pub storage: BTreeMap<BytesKey, BytesValue>,
    pub code: Option<BytesValue>,
    pub owner: Option<AddressValue>,
    pub developer_rewards: Option<BigUintValue>,
}

impl Account {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn nonce<V>(mut self, nonce: V) -> Self
    where
        U64Value: From<V>,
    {
        self.nonce = Some(U64Value::from(nonce));
        self
    }

    pub fn balance<V>(mut self, balance_expr: V) -> Self
    where
        BigUintValue: From<V>,
    {
        self.balance = Some(BigUintValue::from(balance_expr));
        self
    }

    pub fn esdt_balance<K, V>(mut self, token_id_expr: K, balance_expr: V) -> Self
    where
        BytesKey: From<K>,
        BigUintValue: From<V>,
    {
        let token_id = BytesKey::from(token_id_expr);
        let esdt_data_ref = self.get_esdt_data_or_create(&token_id);
        esdt_data_ref.set_balance(0u64, balance_expr);

        self
    }

    pub fn esdt_nft_balance<K, N, V, T>(
        mut self,
        token_id_expr: K,
        nonce_expr: N,
        balance_expr: V,
        opt_attributes_expr: Option<T>,
    ) -> Self
    where
        N: Clone,
        BytesKey: From<K>,
        U64Value: From<N>,
        BigUintValue: From<V>,
        BytesValue: From<T>,
    {
        let token_id = BytesKey::from(token_id_expr);

        let esdt_obj_ref = self
            .get_esdt_data_or_create(&token_id)
            .get_mut_esdt_object();
        esdt_obj_ref.set_balance(nonce_expr.clone(), balance_expr);

        if let Some(attributes_expr) = opt_attributes_expr {
            esdt_obj_ref.set_token_attributes(nonce_expr, attributes_expr);
        }

        self
    }

    pub fn esdt_nft_last_nonce<K, N>(mut self, token_id_expr: K, last_nonce_expr: N) -> Self
    where
        BytesKey: From<K>,
        U64Value: From<N>,
    {
        let token_id = BytesKey::from(token_id_expr);

        let esdt_obj_ref = self
            .get_esdt_data_or_create(&token_id)
            .get_mut_esdt_object();
        esdt_obj_ref.set_last_nonce(last_nonce_expr);

        self
    }

    // TODO: Find a better way to pass roles
    pub fn esdt_roles<K>(mut self, token_id_expr: K, roles: Vec<String>) -> Self
    where
        BytesKey: From<K>,
    {
        let token_id = BytesKey::from(token_id_expr);

        let esdt_obj_ref = self
            .get_esdt_data_or_create(&token_id)
            .get_mut_esdt_object();
        esdt_obj_ref.set_roles(roles);

        self
    }

    fn get_esdt_data_or_create(&mut self, token_id: &BytesKey) -> &mut Esdt {
        if !self.esdt.contains_key(token_id) {
            let _ = self
                .esdt
                .insert(token_id.clone(), Esdt::Full(EsdtObject::default()));
        }

        self.esdt.get_mut(token_id).unwrap()
    }

    pub fn code<V>(mut self, code_expr: V) -> Self
    where
        BytesValue: From<V>,
    {
        self.code = Some(BytesValue::from(code_expr));
        self
    }

    pub fn owner<V>(mut self, owner_expr: V) -> Self
    where
        AddressValue: From<V>,
    {
        self.owner = Some(AddressValue::from(owner_expr));
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
            developer_rewards: from
                .developer_rewards
                .map(|b| BigUintValue::interpret_from(b, context)),
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
            developer_rewards: self.developer_rewards.map(|n| n.original),
        }
    }
}
