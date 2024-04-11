use std::marker::PhantomData;

use crate::multiversx_sc::codec::{CodecFrom, TopEncodeMulti};

use crate::{
    scenario::model::{AddressValue, BytesValue, TxExpect},
    scenario_model::TxResponse,
};

use super::{format_expect, ScQueryStep};

#[derive(Debug, Default)]
pub struct TypedScQuery<OriginalResult> {
    pub sc_query_step: ScQueryStep,
    _return_type: PhantomData<OriginalResult>,
}

impl<OriginalResult> From<TypedScQuery<OriginalResult>> for ScQueryStep {
    fn from(typed: TypedScQuery<OriginalResult>) -> Self {
        typed.sc_query_step
    }
}

impl<OriginalResult> From<ScQueryStep> for TypedScQuery<OriginalResult> {
    fn from(untyped: ScQueryStep) -> Self {
        Self {
            sc_query_step: untyped,
            _return_type: PhantomData,
        }
    }
}

impl<OriginalResult> TypedScQuery<OriginalResult> {
    pub fn function(mut self, expr: &str) -> Self {
        self.sc_query_step.tx.function = expr.to_string();
        self
    }

    pub fn argument<A>(mut self, expr: A) -> Self
    where
        BytesValue: From<A>,
    {
        self.sc_query_step.tx.arguments.push(BytesValue::from(expr));
        self
    }

    pub fn to<A>(mut self, address: A) -> Self
    where
        AddressValue: From<A>,
    {
        self.sc_query_step.tx.to = AddressValue::from(address);
        self
    }

    /// Adds a custom expect section to the tx.
    pub fn expect(mut self, expect: TxExpect) -> Self {
        self.sc_query_step = self.sc_query_step.expect(expect);
        self
    }

    /// Explicitly states that no tx expect section should be added and no checks should be performed.
    ///
    /// Note: by default a basic `TxExpect::ok()` is added, which checks that status is 0 and nothing else.
    pub fn no_expect(mut self) -> Self {
        self.sc_query_step.expect = None;
        self
    }

    /// Shorthand for creating a tx expect with status "Ok" and the given value.
    ///
    /// The given value is type-checked agains the tx return type.
    pub fn expect_value<ExpectedResult>(self, expected_value: ExpectedResult) -> Self
    where
        OriginalResult: TopEncodeMulti,
        ExpectedResult: CodecFrom<OriginalResult> + TopEncodeMulti,
    {
        self.expect(format_expect(expected_value))
    }

    /// Unwraps the response, if available.
    pub fn response(&self) -> &TxResponse {
        self.sc_query_step.response()
    }
}

impl AsMut<ScQueryStep> for ScQueryStep {
    fn as_mut(&mut self) -> &mut ScQueryStep {
        self
    }
}

impl<OriginalResult> AsMut<ScQueryStep> for TypedScQuery<OriginalResult> {
    fn as_mut(&mut self) -> &mut ScQueryStep {
        &mut self.sc_query_step
    }
}

/// Helps with syntax. Allows the `TypedScQuery` to call the `execute` operation directly.
///
/// The trait defines the connection to the executor.
pub trait TypedScQueryExecutor {
    fn execute_typed_sc_query<OriginalResult, RequestedResult>(
        &mut self,
        typed_sc_call: TypedScQuery<OriginalResult>,
    ) -> RequestedResult
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>;
}

impl<OriginalResult> TypedScQuery<OriginalResult>
where
    OriginalResult: TopEncodeMulti,
{
    /// Executes the operation, on the given executor.
    pub fn execute<E: TypedScQueryExecutor, RequestedResult>(
        self,
        executor: &mut E,
    ) -> RequestedResult
    where
        RequestedResult: CodecFrom<OriginalResult>,
    {
        executor.execute_typed_sc_query(self)
    }
}
