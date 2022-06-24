use mandos::interpret_trait::{InterpretableFrom, InterpreterContext};

use crate::mandos_system::model::{
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
        AddressKey: From<A>,
    {
        let address_key = AddressKey::from(address_expr);
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
        AddressValue: From<CA>,
        AddressValue: From<NA>,
    {
        self.new_addresses.push(NewAddress {
            creator_address: AddressValue::from(creator_address_expr),
            creator_nonce: U64Value::from(creator_nonce_expr),
            new_address: AddressValue::from(new_address_expr),
        });
        self
    }

    pub fn block_epoch<N>(mut self, block_epoch_expr: N) -> Self
    where
        U64Value: From<N>,
    {
        let block_epoch = U64Value::from(block_epoch_expr);

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
        U64Value: From<N>,
    {
        let block_nonce = U64Value::from(block_nonce_expr);

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
        U64Value: From<N>,
    {
        let block_round = U64Value::from(block_round_expr);

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
        U64Value: From<N>,
    {
        let block_timestamp = U64Value::from(block_timestamp_expr);

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
        BytesValue: From<B>,
    {
        let block_random_seed = BytesValue::from(block_random_seed_expr);

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
        AddressValue: From<V>,
    {
        self.tx.from = AddressValue::from(expr);
        self
    }

    pub fn egld_value<V>(mut self, expr: V) -> Self
    where
        BigUintValue: From<V>,
    {
        self.tx.egld_value = BigUintValue::from(expr);
        self
    }

    pub fn contract_code(mut self, expr: &str, context: &InterpreterContext) -> Self {
        self.tx.contract_code = BytesValue::interpret_from(expr, context);
        self
    }

    pub fn argument(mut self, expr: &str) -> Self {
        self.tx.arguments.push(BytesValue::from(expr));
        self
    }

    pub fn gas_limit<V>(mut self, value: V) -> Self
    where
        U64Value: From<V>,
    {
        self.tx.gas_limit = U64Value::from(value);
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
        AddressValue: From<A>,
    {
        self.tx.to = AddressValue::from(address);
        self
    }

    pub fn function(mut self, expr: &str) -> Self {
        self.tx.function = expr.to_string();
        self
    }

    pub fn argument(mut self, expr: &str) -> Self {
        self.tx.arguments.push(BytesValue::from(expr));
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
        AddressValue: From<A>,
    {
        self.tx.from = AddressValue::from(address);
        self
    }

    pub fn to<A>(mut self, address: A) -> Self
    where
        AddressValue: From<A>,
    {
        self.tx.to = AddressValue::from(address);
        self
    }

    pub fn egld_value<A>(mut self, amount: A) -> Self
    where
        BigUintValue: From<A>,
    {
        if !self.tx.esdt_value.is_empty() {
            panic!("Cannot transfer both EGLD and ESDT");
        }

        self.tx.egld_value = BigUintValue::from(amount);
        self
    }

    pub fn esdt_transfer<T, N, A>(mut self, token_id: T, token_nonce: N, amount: A) -> Self
    where
        BytesValue: From<T>,
        U64Value: From<N>,
        BigUintValue: From<A>,
    {
        if self.tx.egld_value.value > 0u32.into() {
            panic!("Cannot transfer both EGLD and ESDT");
        }

        self.tx.esdt_value.push(TxESDT {
            esdt_token_identifier: BytesValue::from(token_id),
            nonce: U64Value::from(token_nonce),
            esdt_value: BigUintValue::from(amount),
        });

        self
    }

    pub fn function(mut self, expr: &str) -> Self {
        self.tx.function = expr.to_string();
        self
    }

    pub fn argument<A>(mut self, expr: A) -> Self
    where
        BytesValue: From<A>,
    {
        self.tx.arguments.push(BytesValue::from(expr));
        self
    }

    pub fn gas_limit<V>(mut self, value: V) -> Self
    where
        U64Value: From<V>,
    {
        self.tx.gas_limit = U64Value::from(value);
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
        AddressKey: From<A>,
    {
        let address_key = AddressKey::from(address_expr);
        self.accounts.accounts.insert(address_key, account);
        self
    }
}
