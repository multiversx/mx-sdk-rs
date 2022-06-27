use mandos::interpret_trait::{InterpretableFrom, InterpreterContext};

use crate::mandos_system::model::{
    AddressValue, BigUintValue, BytesValue, TxDeploy, TxExpect, U64Value,
};

#[derive(Debug, Default)]
pub struct ScDeployStep {
    pub tx_id: String,
    pub comment: Option<String>,
    pub tx: Box<TxDeploy>,
    pub expect: Option<TxExpect>,
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
