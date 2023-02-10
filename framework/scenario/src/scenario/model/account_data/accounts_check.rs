use std::collections::BTreeMap;

use crate::scenario_format::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    serde_raw::CheckAccountsRaw,
};

use crate::scenario::model::AddressKey;

use super::CheckAccount;

#[derive(Debug, Default, Clone)]
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

impl IntoRaw<CheckAccountsRaw> for CheckAccounts {
    fn into_raw(self) -> CheckAccountsRaw {
        CheckAccountsRaw {
            other_accounts_allowed: self.other_accounts_allowed,
            accounts: self
                .accounts
                .into_iter()
                .map(|(k, v)| (k.into_raw(), Box::new(v.into_raw())))
                .collect(),
        }
    }
}
