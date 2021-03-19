use super::*;
use std::collections::BTreeMap;

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
				m.into_iter()
					.map(|(k, v)| {
						(
							BytesKey::interpret_from(k, context),
							CheckValue::<BytesValue>::interpret_from(v, context),
						)
					})
					.collect(),
			),
		}
	}
}

impl CheckStorage {
	pub fn is_star(&self) -> bool {
		matches!(self, CheckStorage::Star)
	}
}

#[derive(Debug)]
pub enum CheckEsdt {
	Star,
	Equal(BTreeMap<BytesKey, CheckValue<BigUintValue>>),
}

impl InterpretableFrom<CheckEsdtRaw> for CheckEsdt {
	fn interpret_from(from: CheckEsdtRaw, context: &InterpreterContext) -> Self {
		match from {
			CheckEsdtRaw::Unspecified => CheckEsdt::Equal(BTreeMap::new()),
			CheckEsdtRaw::Star => CheckEsdt::Star,
			CheckEsdtRaw::Equal(m) => CheckEsdt::Equal(
				m.into_iter()
					.map(|(k, v)| {
						(
							BytesKey::interpret_from(k, context),
							CheckValue::<BigUintValue>::interpret_from(v, context),
						)
					})
					.collect(),
			),
		}
	}
}

impl CheckEsdt {
	pub fn is_star(&self) -> bool {
		matches!(self, CheckEsdt::Star)
	}
}

#[derive(Debug)]
pub struct CheckAccount {
	pub comment: Option<String>,
	pub nonce: CheckValue<U64Value>,
	pub balance: CheckValue<BigUintValue>,
	pub storage: CheckStorage,
	pub esdt: CheckEsdt,
	pub code: CheckValue<BytesValue>,
	pub async_call_data: CheckValue<BytesValue>,
}

impl InterpretableFrom<CheckAccountRaw> for CheckAccount {
	fn interpret_from(from: CheckAccountRaw, context: &InterpreterContext) -> Self {
		CheckAccount {
			comment: from.comment,
			nonce: CheckValue::<U64Value>::interpret_from(from.nonce, context),
			balance: CheckValue::<BigUintValue>::interpret_from(from.balance, context),
			storage: CheckStorage::interpret_from(from.storage, context),
			esdt: CheckEsdt::interpret_from(from.esdt, context),
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
