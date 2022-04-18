use crate::interpret_trait::{InterpretableFrom, InterpreterContext};

use super::{
    Account, AddressKey, AddressValue, BigUintValue, BlockInfo, BytesValue, CheckAccount,
    CheckStateStep, NewAddress, ScCallStep, ScDeployStep, ScQueryStep, SetStateStep, TxESDT,
    TxExpect, U64Value,
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

    pub fn block_epoch<N>(mut self, block_epoch_expr: N) -> Self
    where
        U64Value: InterpretableFrom<N>,
    {
        let ctx = InterpreterContext::default();
        let block_epoch = U64Value::interpret_from(block_epoch_expr, &ctx);

        if let Some(block_info) = &mut *self.current_block_info {
            block_info.block_epoch = Some(block_epoch);
        } else {
            *self.current_block_info = Some(BlockInfo {
                block_epoch: Some(block_epoch),
                ..Default::default()
            });
        }

        self
    }

    pub fn block_nonce<N>(mut self, block_nonce_expr: N) -> Self
    where
        U64Value: InterpretableFrom<N>,
    {
        let ctx = InterpreterContext::default();
        let block_nonce = U64Value::interpret_from(block_nonce_expr, &ctx);

        if let Some(block_info) = &mut *self.current_block_info {
            block_info.block_nonce = Some(block_nonce);
        } else {
            *self.current_block_info = Some(BlockInfo {
                block_nonce: Some(block_nonce),
                ..Default::default()
            });
        }

        self
    }

    pub fn block_round<N>(mut self, block_round_expr: N) -> Self
    where
        U64Value: InterpretableFrom<N>,
    {
        let ctx = InterpreterContext::default();
        let block_round = U64Value::interpret_from(block_round_expr, &ctx);

        if let Some(block_info) = &mut *self.current_block_info {
            block_info.block_round = Some(block_round);
        } else {
            *self.current_block_info = Some(BlockInfo {
                block_round: Some(block_round),
                ..Default::default()
            });
        }

        self
    }

    pub fn block_timestamp<N>(mut self, block_timestamp_expr: N) -> Self
    where
        U64Value: InterpretableFrom<N>,
    {
        let ctx = InterpreterContext::default();
        let block_timestamp = U64Value::interpret_from(block_timestamp_expr, &ctx);

        if let Some(block_info) = &mut *self.current_block_info {
            block_info.block_timestamp = Some(block_timestamp);
        } else {
            *self.current_block_info = Some(BlockInfo {
                block_timestamp: Some(block_timestamp),
                ..Default::default()
            });
        }

        self
    }

    pub fn block_random_seed<B>(mut self, block_random_seed_expr: B) -> Self
    where
        BytesValue: InterpretableFrom<B>,
    {
        let ctx = InterpreterContext::default();
        let block_random_seed = BytesValue::interpret_from(block_random_seed_expr, &ctx);

        if let Some(block_info) = &mut *self.current_block_info {
            block_info.block_random_seed = Some(block_random_seed);
        } else {
            *self.current_block_info = Some(BlockInfo {
                block_random_seed: Some(block_random_seed),
                ..Default::default()
            });
        }

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

    pub fn egld_value<A>(mut self, amount: A) -> Self
    where
        BigUintValue: InterpretableFrom<A>,
    {
        if !self.tx.esdt_value.is_empty() {
            panic!("Cannot transfer both EGLD and ESDT");
        }

        self.tx.egld_value = BigUintValue::interpret_from(amount, &InterpreterContext::default());
        self
    }

    pub fn esdt_transfer<T, N, A>(mut self, token_id: T, token_nonce: N, amount: A) -> Self
    where
        BytesValue: InterpretableFrom<T>,
        U64Value: InterpretableFrom<N>,
        BigUintValue: InterpretableFrom<A>,
    {
        if self.tx.egld_value.value > 0u32.into() {
            panic!("Cannot transfer both EGLD and ESDT");
        }

        let ctx = InterpreterContext::default();
        self.tx.esdt_value.push(TxESDT {
            esdt_token_identifier: BytesValue::interpret_from(token_id, &ctx),
            nonce: U64Value::interpret_from(token_nonce, &ctx),
            esdt_value: BigUintValue::interpret_from(amount, &ctx),
        });

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
