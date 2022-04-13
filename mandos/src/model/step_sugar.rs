use crate::interpret_trait::{InterpretableFrom, InterpreterContext};

use super::{
    Account, AddressKey, AddressValue, BigUintValue, BytesValue, CheckAccount, CheckStateStep,
    NewAddress, ScCallStep, ScDeployStep, ScQueryStep, SetStateStep, TxExpect, U64Value,
};

impl SetStateStep {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn put_account<A>(mut self, address_expr: A, account: Account) -> Self
    where
        AddressKey: InterpretableFrom<A>,
    {
        let address_key = AddressKey::interpret_from(address_expr, &InterpreterContext::default());
        self.accounts.insert(address_key, account);
        self
    }

    pub fn new_address<CA, NA>(
        mut self,
        creator_address_expr: CA,
        creator_nonce_expr: u64,
        new_address_expr: NA,
    ) -> Self
    where
        AddressValue: InterpretableFrom<CA>,
        AddressValue: InterpretableFrom<NA>,
    {
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

    pub fn from<V>(mut self, expr: V) -> Self
    where
        AddressValue: InterpretableFrom<V>,
    {
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

    pub fn contract_code(mut self, expr: &str, context: &InterpreterContext) -> Self {
        self.tx.contract_code = BytesValue::interpret_from(expr, context);
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

    pub fn to<A>(mut self, address: A) -> Self
    where
        AddressValue: InterpretableFrom<A>,
    {
        self.tx.to = AddressValue::interpret_from(address, &InterpreterContext::default());
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

    pub fn from<A>(mut self, address: A) -> Self
    where
        AddressValue: InterpretableFrom<A>,
    {
        self.tx.from = AddressValue::interpret_from(address, &InterpreterContext::default());
        self
    }

    pub fn to<A>(mut self, address: A) -> Self
    where
        AddressValue: InterpretableFrom<A>,
    {
        self.tx.to = AddressValue::interpret_from(address, &InterpreterContext::default());
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

    pub fn put_account<A>(mut self, address_expr: A, account: CheckAccount) -> Self
    where
        AddressKey: InterpretableFrom<A>,
    {
        let address_key = AddressKey::interpret_from(address_expr, &InterpreterContext::default());
        self.accounts.accounts.insert(address_key, account);
        self
    }
}
