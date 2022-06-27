use std::marker::PhantomData;

use elrond_wasm::types::ContractDeploy;
use mandos::interpret_trait::{InterpretableFrom, InterpreterContext};

use crate::{
    mandos_system::model::{AddressValue, BigUintValue, BytesValue, TxDeploy, TxExpect, U64Value},
    DebugApi,
};

use super::ScDeployStep;

/// `SCCallStep` with explicit return type.
#[derive(Debug)]
pub struct TypedScDeploy<OriginalResult> {
    pub tx_id: String,
    pub comment: Option<String>,
    pub tx: Box<TxDeploy>,
    pub expect: Option<TxExpect>,
    _return_type: PhantomData<OriginalResult>,
}

pub trait IntoBlockchainDeploy<OriginalResult> {
    fn into_blockchain_deploy(self) -> TypedScDeploy<OriginalResult>;
}

impl<OriginalResult> IntoBlockchainDeploy<OriginalResult>
    for ContractDeploy<DebugApi, OriginalResult>
{
    fn into_blockchain_deploy(self) -> TypedScDeploy<OriginalResult> {
        ScDeployStep::new().call(self).into()
    }
}

impl<OriginalResult> Default for TypedScDeploy<OriginalResult> {
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

impl<OriginalResult> From<TypedScDeploy<OriginalResult>> for ScDeployStep {
    fn from(typed: TypedScDeploy<OriginalResult>) -> Self {
        Self {
            tx_id: typed.tx_id,
            comment: typed.comment,
            tx: typed.tx,
            expect: typed.expect,
        }
    }
}

impl<OriginalResult> From<ScDeployStep> for TypedScDeploy<OriginalResult> {
    fn from(typed: ScDeployStep) -> Self {
        Self {
            tx_id: typed.tx_id,
            comment: typed.comment,
            tx: typed.tx,
            expect: typed.expect,
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
