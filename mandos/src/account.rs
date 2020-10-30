use super::*;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Account {
    pub comment: Option<String>,
    pub nonce: U64Value,
    pub balance: BigUintValue,
    pub storage: BTreeMap<BytesKey, BytesValue>,
    pub esdt: Option<BTreeMap<BytesKey, BigUintValue>>,
    pub code: Option<BytesValue>,
}

impl InterpretableFrom<AccountRaw> for Account {
    fn interpret_from(from: AccountRaw, context: &InterpreterContext) -> Self {
        Account {
            comment: from.comment,
            nonce: U64Value::interpret_from(from.nonce, context),
            balance: BigUintValue::interpret_from(from.balance, context),
            storage: from.storage.into_iter().map(|(k, v)| (
                BytesKey::interpret_from(k, context), 
                BytesValue::interpret_from(v, context))).collect(),
            esdt: from.esdt.map(|tree| tree.into_iter().map(|(k, v)| (
                BytesKey::interpret_from(k, context), 
                BigUintValue::interpret_from(v, context))).collect()),
            code: from.code.map(|c| BytesValue::interpret_from(c, context)),
        }
    }
}

#[derive(Debug)]
pub enum CheckStorage {
    Star,
    Equal(BTreeMap<BytesKey, CheckValue<BytesValue>>),
}

impl InterpretableFrom<CheckStorageRaw> for CheckStorage {
    fn interpret_from(from: CheckStorageRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckStorageRaw::Star => CheckStorage::Star,
            CheckStorageRaw::Equal(m) => CheckStorage::Equal(
                m.into_iter().map(|(k, v)| (
                    BytesKey::interpret_from(k, context), 
                    CheckValue::<BytesValue>::interpret_from(v, context))).collect(),
            )
        }
    }
}

impl CheckStorage {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckStorage::Star)
    }
}

#[derive(Debug)]
pub struct CheckAccount {
    pub comment: Option<String>,
    pub nonce: CheckValue<U64Value>,
    pub balance: CheckValue<BigUintValue>,
    pub storage: CheckStorage,
    pub code: Option<CheckValue<BytesValue>>,
    pub async_call_data: CheckValue<BytesValue>,
}

impl InterpretableFrom<CheckAccountRaw> for CheckAccount {
    fn interpret_from(from: CheckAccountRaw, context: &InterpreterContext) -> Self {
        CheckAccount {
            comment: from.comment,
            nonce: CheckValue::<U64Value>::interpret_from(from.nonce, context),
            balance: CheckValue::<BigUintValue>::interpret_from(from.balance, context),
            storage: CheckStorage::interpret_from(from.storage, context),
            code: from.code.map(|c| CheckValue::<BytesValue>::interpret_from(c, context)),
            async_call_data: CheckValue::<BytesValue>::interpret_from(from.async_call_data, context),
        }
    }
}

#[derive(Debug)]
pub struct CheckAccounts {
    pub other_accounts_allowed: bool,
    pub accounts: BTreeMap<AddressKey, CheckAccount>
}

impl InterpretableFrom<CheckAccountsRaw> for CheckAccounts {
    fn interpret_from(from: CheckAccountsRaw, context: &InterpreterContext) -> Self {
        CheckAccounts {
            other_accounts_allowed: from.other_accounts_allowed,
            accounts: from.accounts.into_iter().map(|(k, v)| (
                AddressKey::interpret_from(k, context), 
                CheckAccount::interpret_from(v, context))).collect(),
        }
    }
}
