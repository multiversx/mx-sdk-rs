use std::collections::btree_map::Entry;

use crate::scenario_model::{
    Account, AddressKey, AddressValue, BigUintValue, BytesKey, BytesValue, Esdt, EsdtObject,
    SetStateStep, U64Value,
};

use super::{SetStateBuilder, SetStateBuilderItem};

pub struct AccountItem {
    address: AddressKey,
    account: Account,
}

impl AccountItem {
    pub fn new(address: AddressKey) -> Self {
        AccountItem {
            address,
            account: Account::default(),
        }
    }
}

impl SetStateBuilderItem for AccountItem {
    fn commit_to_step(&mut self, step: &mut SetStateStep) {
        if let Entry::Vacant(entry) = step.accounts.entry(core::mem::take(&mut self.address)) {
            entry.insert(core::mem::take(&mut self.account));
        };
    }
}

impl<'w> SetStateBuilder<'w, AccountItem> {
    pub fn nonce<V>(mut self, nonce: V) -> Self
    where
        U64Value: From<V>,
    {
        self.item.account.nonce = Some(U64Value::from(nonce));
        self
    }

    pub fn balance<V>(mut self, balance_expr: V) -> Self
    where
        BigUintValue: From<V>,
    {
        self.item.account.balance = Some(BigUintValue::from(balance_expr));
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

    #[allow(clippy::too_many_arguments)]
    pub fn esdt_nft_all_properties<K, N, V, T>(
        mut self,
        token_id_expr: K,
        nonce_expr: N,
        balance_expr: V,
        opt_attributes_expr: Option<T>,
        royalties_expr: N,
        creator_expr: Option<T>,
        hash_expr: Option<T>,
        uris_expr: Vec<T>,
    ) -> Self
    where
        BytesKey: From<K>,
        U64Value: From<N>,
        BigUintValue: From<V>,
        BytesValue: From<T>,
    {
        let token_id = BytesKey::from(token_id_expr);

        let esdt_obj_ref = self
            .get_esdt_data_or_create(&token_id)
            .get_mut_esdt_object();

        esdt_obj_ref.set_token_all_properties(
            nonce_expr,
            balance_expr,
            opt_attributes_expr,
            royalties_expr,
            creator_expr,
            hash_expr,
            uris_expr,
        );

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
        if !self.item.account.esdt.contains_key(token_id) {
            self.item
                .account
                .esdt
                .insert(token_id.clone(), Esdt::Full(EsdtObject::default()));
        }

        self.item.account.esdt.get_mut(token_id).unwrap()
    }

    pub fn code<V>(mut self, code_expr: V) -> Self
    where
        BytesValue: From<V>,
    {
        self.item.account.code = Some(BytesValue::from(code_expr));
        self
    }

    pub fn owner<V>(mut self, owner_expr: V) -> Self
    where
        AddressValue: From<V>,
    {
        self.item.account.owner = Some(AddressValue::from(owner_expr));
        self
    }
}
