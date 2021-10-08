use std::collections::BTreeMap;

use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::AddressKey,
    serde_raw::CheckAccountsRaw,
};

use super::CheckAccount;

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
