use std::marker::PhantomData;

use multiversx_sc::codec::PanicErrorHandler;

use crate::multiversx_sc::codec::{CodecFrom, TopEncodeMulti};

use crate::{
    scenario::model::{AddressValue, U64Value},
    scenario_model::{BigUintValue, BytesValue, TxExpect, TxResponse, TxResponseStatus},
};

use super::{format_expect, ScCallStep};

/// `SCCallStep` with explicit return type.
#[derive(Default, Debug)]
pub struct TypedScCall<OriginalResult> {
    pub sc_call_step: ScCallStep,
    _phantom: PhantomData<OriginalResult>,
}

impl<OriginalResult> TypedScCall<OriginalResult> {
    pub fn result<RequestedResult>(&self) -> Result<RequestedResult, TxResponseStatus>
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>,
    {
        let mut raw_result = self.response().out.clone();
        Ok(
            RequestedResult::multi_decode_or_handle_err(&mut raw_result, PanicErrorHandler)
                .unwrap(),
        )
    }

    pub fn from<A>(mut self, address: A) -> Self
    where
        AddressValue: From<A>,
    {
        self.sc_call_step = self.sc_call_step.from(address);
        self
    }

    pub fn to<A>(mut self, address: A) -> Self
    where
        AddressValue: From<A>,
    {
        self.sc_call_step = self.sc_call_step.to(address);
        self
    }

    pub fn egld_value<A>(mut self, amount: A) -> Self
    where
        BigUintValue: From<A>,
    {
        self.sc_call_step = self.sc_call_step.egld_value(amount);
        self
    }

    pub fn esdt_transfer<T, N, A>(mut self, token_id: T, token_nonce: N, amount: A) -> Self
    where
        BytesValue: From<T>,
        U64Value: From<N>,
        BigUintValue: From<A>,
    {
        self.sc_call_step = self
            .sc_call_step
            .esdt_transfer(token_id, token_nonce, amount);

        self
    }

    pub fn function(mut self, expr: &str) -> Self {
        self.sc_call_step = self.sc_call_step.function(expr);
        self
    }

    pub fn argument<A>(mut self, expr: A) -> Self
    where
        BytesValue: From<A>,
    {
        self.sc_call_step = self.sc_call_step.argument(expr);
        self
    }

    pub fn gas_limit<V>(mut self, value: V) -> Self
    where
        U64Value: From<V>,
    {
        self.sc_call_step = self.sc_call_step.gas_limit(value);
        self
    }

    /// Adds a custom expect section to the tx.
    pub fn expect(mut self, expect: TxExpect) -> Self {
        self.sc_call_step = self.sc_call_step.expect(expect);
        self
    }

    /// Explicitly states that no tx expect section should be added and no checks should be performed.
    ///
    /// Note: by default a basic `TxExpect::ok()` is added, which checks that status is 0 and nothing else.
    pub fn no_expect(mut self) -> Self {
        self.sc_call_step = self.sc_call_step.no_expect();
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
        self.sc_call_step.response()
    }
}

impl<OriginalResult> AsMut<ScCallStep> for TypedScCall<OriginalResult> {
    fn as_mut(&mut self) -> &mut ScCallStep {
        &mut self.sc_call_step
    }
}

impl<OriginalResult> From<TypedScCall<OriginalResult>> for ScCallStep {
    fn from(typed: TypedScCall<OriginalResult>) -> Self {
        typed.sc_call_step
    }
}

impl<OriginalResult> From<ScCallStep> for TypedScCall<OriginalResult> {
    fn from(untyped: ScCallStep) -> Self {
        Self {
            sc_call_step: untyped,
            _phantom: PhantomData,
        }
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
