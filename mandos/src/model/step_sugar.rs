use crate::interpret_trait::{InterpretableFrom, InterpreterContext};

use super::{
    Account, AddressKey, AddressValue, BigUintValue, BytesValue, CheckAccount, CheckStateStep,
    NewAddress, ScCallStep, ScDeployStep, ScQueryStep, SetStateStep, TxExpect, U64Value,
};

impl SetStateStep {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn put_account(mut self, address_expr: &str, account: Account) -> Self {
        let address_key =
            AddressKey::interpret_from(address_expr.to_string(), &InterpreterContext::default());
        self.accounts.insert(address_key, account);
        self
    }

    pub fn new_address(
        mut self,
        creator_address_expr: &str,
        creator_nonce_expr: u64,
        new_address_expr: &str,
    ) -> Self {
        self.new_addresses.push(NewAddress {
            creator_address: AddressValue::interpret_from(
                creator_address_expr,
                &InterpreterContext::default(),
            ),
            creator_nonce: U64Value::interpret_from(
                creator_nonce_expr,
                &InterpreterContext::default(),
            ),
            new_address: AddressValue::interpret_from(
                new_address_expr,
                &InterpreterContext::default(),
            ),
        });
        self
    }
}

impl ScDeployStep {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(mut self, expr: &str) -> Self {
        self.tx.from = AddressValue::interpret_from(expr, &InterpreterContext::default());
        self
    }

    pub fn egld_value<V>(mut self, expr: V) -> Self
    where
        BigUintValue: InterpretableFrom<V>,
    {
        self.tx.egld_value = BigUintValue::interpret_from(expr, &InterpreterContext::default());
        self
    }

    pub fn contract_code(mut self, expr: &str) -> Self {
        self.tx.contract_code = BytesValue::interpret_from(expr, &InterpreterContext::default());
        self
    }

    pub fn argument(mut self, expr: &str) -> Self {
        self.tx.arguments.push(BytesValue::interpret_from(
            expr,
            &InterpreterContext::default(),
        ));
        self
    }

    pub fn gas_limit<V>(mut self, value: V) -> Self
    where
        U64Value: InterpretableFrom<V>,
    {
        self.tx.gas_limit = U64Value::interpret_from(value, &InterpreterContext::default());
        self
    }

    pub fn expect(mut self, expect: TxExpect) -> Self {
        self.expect = Some(expect);
        self
    }
}

impl ScQueryStep {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to(mut self, expr: &str) -> Self {
        self.tx.to = AddressValue::interpret_from(expr, &InterpreterContext::default());
        self
    }

    pub fn function(mut self, expr: &str) -> Self {
        self.tx.function = expr.to_string();
        self
    }

    pub fn argument(mut self, expr: &str) -> Self {
        self.tx.arguments.push(BytesValue::interpret_from(
            expr,
            &InterpreterContext::default(),
        ));
        self
    }

    pub fn expect(mut self, expect: TxExpect) -> Self {
        self.expect = Some(expect);
        self
    }
}

impl ScCallStep {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(mut self, expr: &str) -> Self {
        self.tx.from = AddressValue::interpret_from(expr, &InterpreterContext::default());
        self
    }

    pub fn to(mut self, expr: &str) -> Self {
        self.tx.to = AddressValue::interpret_from(expr, &InterpreterContext::default());
        self
    }

    pub fn function(mut self, expr: &str) -> Self {
        self.tx.function = expr.to_string();
        self
    }

    pub fn argument(mut self, expr: &str) -> Self {
        self.tx.arguments.push(BytesValue::interpret_from(
            expr,
            &InterpreterContext::default(),
        ));
        self
    }

    pub fn gas_limit<V>(mut self, value: V) -> Self
    where
        U64Value: InterpretableFrom<V>,
    {
        self.tx.gas_limit = U64Value::interpret_from(value, &InterpreterContext::default());
        self
    }

    pub fn expect(mut self, expect: TxExpect) -> Self {
        self.expect = Some(expect);
        self
    }
}

impl CheckStateStep {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn put_account(mut self, address_expr: &str, account: CheckAccount) -> Self {
        let address_key =
            AddressKey::interpret_from(address_expr.to_string(), &InterpreterContext::default());
        self.accounts.accounts.insert(address_key, account);
        self
    }
}
