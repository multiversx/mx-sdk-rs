use multiversx_sc::codec::{top_encode_to_vec_u8_or_panic, TopEncode};

use crate::{
    scenario::model::{
        BigUintValue, BytesKey, BytesValue, CheckEsdt, CheckEsdtInstances, CheckEsdtMap,
        CheckEsdtMapContents, CheckStorage, CheckStorageDetails, CheckValue, U64Value,
    },
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
        serde_raw::CheckAccountRaw,
    },
    scenario_model::CheckEsdtData,
};
use std::collections::BTreeMap;

#[derive(Debug, Default, Clone)]
pub struct CheckAccount {
    pub comment: Option<String>,
    pub nonce: CheckValue<U64Value>,
    pub balance: CheckValue<BigUintValue>,
    pub esdt: CheckEsdtMap,
    pub username: CheckValue<BytesValue>,
    pub storage: CheckStorage,
    pub code: CheckValue<BytesValue>,
    pub owner: CheckValue<BytesValue>, // WARNING! Not currently checked. TODO: implement check
    pub developer_rewards: CheckValue<BigUintValue>,
    pub async_call_data: CheckValue<BytesValue>,
}

impl CheckAccount {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn nonce<V>(mut self, nonce: V) -> Self
    where
        U64Value: InterpretableFrom<V>,
    {
        self.nonce = CheckValue::Equal(U64Value::interpret_from(
            nonce,
            &InterpreterContext::default(),
        ));
        self
    }

    pub fn balance<V>(mut self, balance_expr: V) -> Self
    where
        BigUintValue: InterpretableFrom<V>,
    {
        self.balance = CheckValue::Equal(BigUintValue::interpret_from(
            balance_expr,
            &InterpreterContext::default(),
        ));
        self
    }

    pub fn code<V>(mut self, code_expr: V) -> Self
    where
        BytesValue: InterpretableFrom<V>,
    {
        self.code = CheckValue::Equal(BytesValue::interpret_from(
            code_expr,
            &InterpreterContext::default(),
        ));
        self
    }

    pub fn esdt_balance<K, V>(mut self, token_id_expr: K, balance_expr: V) -> Self
    where
        BytesKey: From<K>,
        BigUintValue: From<V>,
    {
        let token_id = BytesKey::from(token_id_expr);
        let balance = BigUintValue::from(balance_expr);

        match &mut self.esdt {
            CheckEsdtMap::Unspecified | CheckEsdtMap::Star => {
                let mut new_esdt_map = BTreeMap::new();
                let _ = new_esdt_map.insert(token_id, CheckEsdt::Short(balance));

                let new_check_esdt_map = CheckEsdtMapContents {
                    contents: new_esdt_map,
                    other_esdts_allowed: true,
                };

                self.esdt = CheckEsdtMap::Equal(new_check_esdt_map);
            },
            CheckEsdtMap::Equal(check_esdt_map) => {
                if check_esdt_map.contents.contains_key(&token_id) {
                    let prev_entry = check_esdt_map.contents.get_mut(&token_id).unwrap();
                    match prev_entry {
                        CheckEsdt::Short(prev_balance_check) => *prev_balance_check = balance,
                        CheckEsdt::Full(prev_esdt_check) => match prev_esdt_check.instances {
                            CheckEsdtInstances::Star => todo!(),
                            CheckEsdtInstances::Equal(_) => todo!(),
                        },
                    }
                }
            },
        }

        self
    }

    pub fn esdt_nft_balance_and_attributes<K, N, V, T>(
        mut self,
        token_id_expr: K,
        nonce_expr: N,
        balance_expr: V,
        attributes_expr: Option<T>,
    ) -> Self
    where
        BytesKey: From<K>,
        U64Value: From<N>,
        BigUintValue: From<V>,
        T: TopEncode,
    {
        let token_id = BytesKey::from(token_id_expr);

        if let CheckEsdtMap::Unspecified = &self.esdt {
            let mut check_esdt = CheckEsdt::Full(CheckEsdtData::default());

            if let Some(attributes_expr) = attributes_expr {
                check_esdt.add_balance_and_attributes_check(
                    nonce_expr,
                    balance_expr,
                    top_encode_to_vec_u8_or_panic(&attributes_expr),
                );
            } else {
                check_esdt.add_balance_and_attributes_check(
                    nonce_expr,
                    balance_expr,
                    Vec::<u8>::new(),
                );
            }

            let mut new_esdt_map = BTreeMap::new();
            let _ = new_esdt_map.insert(token_id, check_esdt);

            let new_check_esdt_map = CheckEsdtMapContents {
                contents: new_esdt_map,
                other_esdts_allowed: true,
            };

            self.esdt = CheckEsdtMap::Equal(new_check_esdt_map);
        }

        self
    }

    pub fn check_storage(mut self, key: &str, value: &str) -> Self {
        let mut details = match self.storage {
            CheckStorage::Star => CheckStorageDetails::default(),
            CheckStorage::Equal(details) => details,
        };
        details.storages.insert(
            BytesKey::interpret_from(key, &InterpreterContext::default()),
            CheckValue::Equal(BytesValue::interpret_from(
                value,
                &InterpreterContext::default(),
            )),
        );
        self.storage = CheckStorage::Equal(details);
        self
    }
}

impl InterpretableFrom<Box<CheckAccountRaw>> for CheckAccount {
    fn interpret_from(from: Box<CheckAccountRaw>, context: &InterpreterContext) -> Self {
        CheckAccount {
            comment: from.comment,
            nonce: CheckValue::<U64Value>::interpret_from(from.nonce, context),
            balance: CheckValue::<BigUintValue>::interpret_from(from.balance, context),
            esdt: CheckEsdtMap::interpret_from(from.esdt, context),
            username: CheckValue::<BytesValue>::interpret_from(from.username, context),
            storage: CheckStorage::interpret_from(from.storage, context),
            code: CheckValue::<BytesValue>::interpret_from(from.code, context),
            owner: CheckValue::<BytesValue>::interpret_from(from.owner, context),
            developer_rewards: CheckValue::<BigUintValue>::interpret_from(
                from.developer_rewards,
                context,
            ),
            async_call_data: CheckValue::<BytesValue>::interpret_from(
                from.async_call_data,
                context,
            ),
        }
    }
}

impl IntoRaw<CheckAccountRaw> for CheckAccount {
    fn into_raw(self) -> CheckAccountRaw {
        CheckAccountRaw {
            comment: self.comment,
            nonce: self.nonce.into_raw(),
            balance: self.balance.into_raw(),
            esdt: self.esdt.into_raw(),
            username: self.username.into_raw(),
            storage: self.storage.into_raw(),
            code: self.code.into_raw_explicit(), // TODO: convert back to into_raw after VM CI upgrade
            owner: self.owner.into_raw_explicit(), // TODO: convert back to into_raw after VM CI upgrade
            developer_rewards: self.developer_rewards.into_raw(),
            async_call_data: self.async_call_data.into_raw(),
        }
    }
}
