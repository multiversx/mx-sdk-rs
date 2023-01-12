use std::marker::PhantomData;

use multiversx_sc::codec::{CodecFrom, TopEncodeMulti};

use crate::scenario::model::{
    AddressValue, BigUintValue, BytesValue, TxCall, TxESDT, TxExpect, U64Value,
};

use super::ScCallStep;

/// `SCCallStep` with explicit return type.
#[derive(Debug)]
pub struct TypedScCall<OriginalResult> {
    pub id: String,
    pub tx_id: Option<String>,
    pub comment: Option<String>,
    pub tx: Box<TxCall>,
    pub expect: Option<TxExpect>,
    _return_type: PhantomData<OriginalResult>,
}

impl<OriginalResult> Default for TypedScCall<OriginalResult> {
    fn default() -> Self {
        Self {
            id: Default::default(),
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
            id: typed.id,
            tx_id: typed.tx_id,
            comment: typed.comment,
            tx: typed.tx,
            expect: typed.expect,
        }
    }
}

impl<OriginalResult> From<ScCallStep> for TypedScCall<OriginalResult> {
    fn from(untyped: ScCallStep) -> Self {
        Self {
            id: untyped.id,
            tx_id: untyped.tx_id,
            comment: untyped.comment,
            tx: untyped.tx,
            expect: untyped.expect,
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

/// Helps with syntax. Allows the `TypedScCall` to call the `execute` operation directly.
///
/// The trait defines the connection to the executor.
pub trait TypedScCallExecutor {
    fn execute_typed_sc_call<OriginalResult, RequestedResult>(
        &mut self,
        typed_sc_call: TypedScCall<OriginalResult>,
    ) -> RequestedResult
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>;
}

impl<OriginalResult> TypedScCall<OriginalResult>
where
    OriginalResult: TopEncodeMulti,
{
    /// Executes the operation, on the given executor.
    pub fn execute<E: TypedScCallExecutor, RequestedResult>(
        self,
        executor: &mut E,
    ) -> RequestedResult
    where
        RequestedResult: CodecFrom<OriginalResult>,
    {
        executor.execute_typed_sc_call(self)
    }
}
