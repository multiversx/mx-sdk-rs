use crate::mandos_system::model::{AddressKey, CheckAccount, CheckAccounts};

#[derive(Debug, Default)]
pub struct CheckStateStep {
    pub comment: Option<String>,
    pub accounts: CheckAccounts,
}

impl CheckStateStep {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn put_account<A>(mut self, address_expr: A, account: CheckAccount) -> Self
    where
        AddressKey: From<A>,
    {
        let address_key = AddressKey::from(address_expr);
        self.accounts.accounts.insert(address_key, account);
        self
    }
}
