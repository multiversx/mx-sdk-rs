use std::marker::PhantomData;

use crate::scenario_format::interpret_trait::{InterpretableFrom, InterpreterContext};
use multiversx_sc::{
    codec::{CodecFrom, TopEncodeMulti},
    types::{Address, CodeMetadata},
};

use crate::scenario::model::{
    AddressValue, BigUintValue, BytesValue, TxDeploy, TxExpect, U64Value,
};

use super::ScDeployStep;

/// `SCCallStep` with explicit return type.
#[derive(Debug)]
pub struct TypedScDeploy<OriginalResult> {
    pub id: String,
    pub tx_id: Option<String>,
    pub comment: Option<String>,
    pub tx: Box<TxDeploy>,
    pub expect: Option<TxExpect>,
    _return_type: PhantomData<OriginalResult>,
}

impl<OriginalResult> Default for TypedScDeploy<OriginalResult> {
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

impl<OriginalResult> From<TypedScDeploy<OriginalResult>> for ScDeployStep {
    fn from(typed: TypedScDeploy<OriginalResult>) -> Self {
        Self {
            id: typed.id,
            tx_id: typed.tx_id,
            comment: typed.comment,
            tx: typed.tx,
            expect: typed.expect,
        }
    }
}

impl<OriginalResult> From<ScDeployStep> for TypedScDeploy<OriginalResult> {
    fn from(untyped: ScDeployStep) -> Self {
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

impl<OriginalResult> TypedScDeploy<OriginalResult> {
    pub fn from<A>(mut self, address: A) -> Self
    where
        AddressValue: From<A>,
    {
        self.tx.from = AddressValue::from(address);
        self
    }

    pub fn egld_value<A>(mut self, amount: A) -> Self
    where
        BigUintValue: From<A>,
    {
        self.tx.egld_value = BigUintValue::from(amount);
        self
    }

    pub fn code_metadata(mut self, code_metadata: CodeMetadata) -> Self {
        self.tx.code_metadata = code_metadata;
        self
    }

    pub fn contract_code(mut self, expr: &str, context: &InterpreterContext) -> Self {
        self.tx.contract_code = BytesValue::interpret_from(expr, context);
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

/// Helps with syntax. Allows the `TypedScDeploy` to call the `execute` operation directly.
///
/// The trait defines the connection to the executor.
pub trait TypedScDeployExecutor {
    fn execute_typed_sc_deploy<OriginalResult, RequestedResult>(
        &mut self,
        typed_sc_call: TypedScDeploy<OriginalResult>,
    ) -> (Address, RequestedResult)
    where
        OriginalResult: TopEncodeMulti,
        RequestedResult: CodecFrom<OriginalResult>;
}

impl<OriginalResult> TypedScDeploy<OriginalResult>
where
    OriginalResult: TopEncodeMulti,
{
    /// Executes the operation, on the given executor.
    pub fn execute<E: TypedScDeployExecutor, RequestedResult>(
        self,
        executor: &mut E,
    ) -> (Address, RequestedResult)
    where
        RequestedResult: CodecFrom<OriginalResult>,
    {
        executor.execute_typed_sc_deploy(self)
    }
}
