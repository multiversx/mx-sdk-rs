use std::marker::PhantomData;

use elrond_wasm::types::ContractCall;

use crate::{
    mandos_system::model::{
        AddressValue, BigUintValue, BytesValue, TxCall, TxESDT, TxExpect, U64Value,
    },
    DebugApi,
};

use super::ScCallStep;

/// `SCCallStep` with explicit return type.
#[derive(Debug)]
pub struct TypedScCall<OriginalResult> {
    pub tx_id: String,
    pub comment: Option<String>,
    pub tx: Box<TxCall>,
    pub expect: Option<TxExpect>,
    _return_type: PhantomData<OriginalResult>,
}

pub trait IntoBlockchainCall<OriginalResult> {
    fn into_blockchain_call(self) -> TypedScCall<OriginalResult>;
}

impl<OriginalResult> IntoBlockchainCall<OriginalResult> for ContractCall<DebugApi, OriginalResult> {
    fn into_blockchain_call(self) -> TypedScCall<OriginalResult> {
        ScCallStep::new().call(self).into()
    }
}

impl<OriginalResult> Default for TypedScCall<OriginalResult> {
    fn default() -> Self {
        Self {
            tx_id: Default::default(),
            comment: Default::default(),
            tx: Default::default(),
            expect: Default::default(),
            _return_type: PhantomData,
        }
    }
}

impl<OriginalResult> From<TypedScCall<OriginalResult>> for ScCallStep {
    fn from(typed: TypedScCall<OriginalResult>) -> Self {
        ScCallStep {
            tx_id: typed.tx_id,
            comment: typed.comment,
            tx: typed.tx,
            expect: typed.expect,
        }
    }
}

impl<OriginalResult> From<ScCallStep> for TypedScCall<OriginalResult> {
    fn from(typed: ScCallStep) -> Self {
        Self {
            tx_id: typed.tx_id,
            comment: typed.comment,
            tx: typed.tx,
            expect: typed.expect,
            _return_type: PhantomData,
        }
    }
}

impl<OriginalResult> TypedScCall<OriginalResult> {
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
