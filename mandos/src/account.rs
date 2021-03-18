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
			esdt: from.esdt.map(|tree| {
				tree.into_iter()
					.map(|(k, v)| {
						(
							BytesKey::interpret_from(k, context),
							BigUintValue::interpret_from(v, context),
						)
					})
					.collect()
			}),
			code: from.code.map(|c| BytesValue::interpret_from(c, context)),
		}
	}
}
