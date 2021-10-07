use crate::{
    AddressKey, BigUintValue, BytesValue, CheckAccountRaw, CheckAccountsRaw, CheckEsdtMap,
    CheckStorage, CheckValue, InterpretableFrom, InterpreterContext, U64Value,
};

use std::collections::BTreeMap;

#[derive(Debug)]
pub struct CheckAccount {
    pub comment: Option<String>,
    pub nonce: CheckValue<U64Value>,
    pub balance: CheckValue<BigUintValue>,
    pub esdt: CheckEsdtMap,
    pub username: CheckValue<BytesValue>,
    pub storage: CheckStorage,
    pub code: CheckValue<BytesValue>,
    pub async_call_data: CheckValue<BytesValue>,
}

impl InterpretableFrom<CheckAccountRaw> for CheckAccount {
    fn interpret_from(from: CheckAccountRaw, context: &InterpreterContext) -> Self {
        CheckAccount {
            comment: from.comment,
            nonce: CheckValue::<U64Value>::interpret_from(from.nonce, context),
            balance: CheckValue::<BigUintValue>::interpret_from(from.balance, context),
            esdt: CheckEsdtMap::interpret_from(from.esdt, context),
            username: CheckValue::<BytesValue>::interpret_from(from.username, context),
            storage: CheckStorage::interpret_from(from.storage, context),
            code: CheckValue::<BytesValue>::interpret_from(from.code, context),
            async_call_data: CheckValue::<BytesValue>::interpret_from(
                from.async_call_data,
                context,
            ),
        }
    }
}

#[derive(Debug)]
pub struct CheckAccounts {
    pub other_accounts_allowed: bool,
    pub accounts: BTreeMap<AddressKey, CheckAccount>,
}

impl InterpretableFrom<CheckAccountsRaw> for CheckAccounts {
    fn interpret_from(from: CheckAccountsRaw, context: &InterpreterContext) -> Self {
        CheckAccounts {
            other_accounts_allowed: from.other_accounts_allowed,
            accounts: from
                .accounts
                .into_iter()
                .map(|(k, v)| {
                    (
                        AddressKey::interpret_from(k, context),
                        CheckAccount::interpret_from(v, context),
                    )
                })
                .collect(),
        }
    }
}
