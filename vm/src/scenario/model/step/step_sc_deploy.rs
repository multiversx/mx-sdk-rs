use crate::scenario_format::interpret_trait::{InterpretableFrom, InterpreterContext};

use crate::{
    scenario::model::{AddressValue, BigUintValue, BytesValue, TxDeploy, TxExpect, U64Value},
    DebugApi,
};

use multiversx_sc::types::{CodeMetadata, ContractDeploy};

use super::convert_call_args;

#[derive(Debug, Default)]
pub struct ScDeployStep {
    pub id: String,
    pub tx_id: Option<String>,
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

    pub fn code_metadata(mut self, code_metadata: CodeMetadata) -> Self {
        self.tx.code_metadata = code_metadata;
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

    /// Sets following fields based on the smart contract proxy:
    /// - "function"
    /// - "arguments"
    pub fn call<OriginalResult>(
        mut self,
        contract_deploy: ContractDeploy<DebugApi, OriginalResult>,
    ) -> Self {
        let (_, mandos_args) = process_contract_deploy(contract_deploy);
        for arg in mandos_args {
            self = self.argument(arg.as_str());
        }
        self
    }
}

/// Extracts
/// - (optional) recipient (needed for contract upgrade, not yet used);
/// - the arguments.
fn process_contract_deploy<OriginalResult>(
    contract_deploy: ContractDeploy<DebugApi, OriginalResult>,
) -> (Option<String>, Vec<String>) {
    let to_str = contract_deploy
        .to
        .as_option()
        .map(|to| format!("0x{}", hex::encode(to.to_address().as_bytes())));
    let mandos_args = convert_call_args(&contract_deploy.arg_buffer);
    (to_str, mandos_args)
}
