use std::collections::btree_map::Entry;

use multiversx_sc::types::{
    AnnotatedValue, BigUint, ManagedAddress, ManagedBuffer, TokenIdentifier,
};

use crate::{
    imports::StaticApi,
    scenario::tx_to_step::{
        address_annotated, big_uint_annotated, bytes_annotated, token_identifier_annotated,
        u64_annotated,
    },
    scenario_model::{
        Account, AddressKey, BigUintValue, BytesKey, BytesValue, Esdt, EsdtObject, SetStateStep,
        U64Value,
    },
    ScenarioTxEnvData,
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
    pub fn nonce<N>(mut self, nonce: N) -> Self
    where
        N: AnnotatedValue<ScenarioTxEnvData, u64>,
    {
        let env = self.new_env_data();
        self.item.account.nonce = Some(u64_annotated(&env, &nonce));
        self
    }

    pub fn balance<V>(mut self, balance: V) -> Self
    where
        V: AnnotatedValue<ScenarioTxEnvData, BigUint<StaticApi>>,
    {
        let env = self.new_env_data();
        self.item.account.balance = Some(big_uint_annotated(&env, &balance));
        self
    }

    pub fn esdt_balance<K, V>(mut self, token_id: K, balance: V) -> Self
    where
        K: AnnotatedValue<ScenarioTxEnvData, TokenIdentifier<StaticApi>>,
        V: AnnotatedValue<ScenarioTxEnvData, BigUint<StaticApi>>,
    {
        let env = self.new_env_data();
        let token_id_key = token_identifier_annotated(&env, token_id);
        let balance_value = big_uint_annotated(&env, &balance);

        let esdt_data_ref = self.get_esdt_data_or_create(&token_id_key);
        esdt_data_ref.set_balance(0u64, balance_value);

        self
    }

    pub fn esdt_nft_balance<K, N, V, T>(
        mut self,
        token_id: K,
        nonce: N,
        balance: V,
        attributes: T,
    ) -> Self
    where
        K: AnnotatedValue<ScenarioTxEnvData, TokenIdentifier<StaticApi>>,
        N: AnnotatedValue<ScenarioTxEnvData, u64>,
        V: AnnotatedValue<ScenarioTxEnvData, BigUint<StaticApi>>,
        T: AnnotatedValue<ScenarioTxEnvData, ManagedBuffer<StaticApi>>,
    {
        let env = self.new_env_data();
        let token_id_key = token_identifier_annotated(&env, token_id);
        let nonce_value = u64_annotated(&env, &nonce);
        let balance_value = big_uint_annotated(&env, &balance);
        let attributes_value = bytes_annotated(&env, attributes);

        let esdt_obj_ref = self
            .get_esdt_data_or_create(&token_id_key)
            .get_mut_esdt_object();
        esdt_obj_ref.set_balance(nonce_value.clone(), balance_value);

        esdt_obj_ref.set_token_attributes(nonce_value, attributes_value);

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

    pub fn code<C>(mut self, code: C) -> Self
    where
        C: AnnotatedValue<ScenarioTxEnvData, ManagedBuffer<StaticApi>>,
    {
        let env = self.new_env_data();
        let code_value = bytes_annotated(&env, code);
        self.item.account.code = Some(code_value);
        self
    }

    pub fn owner<V>(mut self, owner: V) -> Self
    where
        V: AnnotatedValue<ScenarioTxEnvData, ManagedAddress<StaticApi>>,
    {
        let env = self.new_env_data();
        let owner_value = address_annotated(&env, &owner);
        self.item.account.owner = Some(owner_value);
        self
    }
}
