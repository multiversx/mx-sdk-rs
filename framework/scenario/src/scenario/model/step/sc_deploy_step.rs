use multiversx_sc::types::H256;

use crate::{
    api::StaticApi,
    scenario_format::interpret_trait::{InterpretableFrom, InterpreterContext},
};

use crate::{
    scenario::model::{AddressValue, BigUintValue, BytesValue, TxDeploy, TxExpect, U64Value},
    scenario_model::TxResponse,
};

use crate::multiversx_sc::types::{CodeMetadata, ContractDeploy};

use super::{convert_call_args, TypedScDeploy};

#[derive(Debug, Clone)]
pub struct ScDeployStep {
    pub id: String,
    pub tx_id: Option<String>,
    pub explicit_tx_hash: Option<H256>,
    pub comment: Option<String>,
    pub tx: Box<TxDeploy>,
    pub expect: Option<TxExpect>,
    pub response: Option<TxResponse>,
}

impl Default for ScDeployStep {
    fn default() -> Self {
        Self {
            id: Default::default(),
            tx_id: Default::default(),
            explicit_tx_hash: Default::default(),
            comment: Default::default(),
            tx: Default::default(),
            expect: Some(TxExpect::ok()),
            response: Default::default(),
        }
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

    pub fn code_metadata(mut self, code_metadata: CodeMetadata) -> Self {
        self.tx.code_metadata = code_metadata;
        self
    }

    pub fn code<V>(mut self, expr: V) -> Self
    where
        BytesValue: From<V>,
    {
        self.tx.contract_code = BytesValue::from(expr);
        self
    }

    #[deprecated(
        since = "0.42.0",
        note = "Please use method `code` instead. To ease transition, it is also possible to call it with a tuple like so: `.code((expr, context))`"
    )]
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

    /// Sets following fields based on the smart contract proxy:
    /// - "function"
    /// - "arguments"
    pub fn call<OriginalResult>(
        mut self,
        contract_deploy: ContractDeploy<StaticApi, OriginalResult>,
    ) -> TypedScDeploy<OriginalResult> {
        let (_, mandos_args) = process_contract_deploy(contract_deploy);
        for arg in mandos_args {
            self = self.argument(arg.as_str());
        }
        self.into()
    }

    /// Adds a custom expect section to the tx.
    pub fn expect(mut self, expect: TxExpect) -> Self {
        self.expect = Some(expect);
        self
    }

    /// Explicitly states that no tx expect section should be added and no checks should be performed.
    ///
    /// Note: by default a basic `TxExpect::ok()` is added, which checks that status is 0 and nothing else.
    pub fn no_expect(mut self) -> Self {
        self.expect = None;
        self
    }

    /// Unwraps the response, if available.
    pub fn response(&self) -> &TxResponse {
        self.response
            .as_ref()
            .expect("SC deploy response not yet available")
    }

    pub fn save_response(&mut self, response: TxResponse) {
        if let Some(expect) = &mut self.expect {
            if expect.build_from_response {
                expect.update_from_response(&response)
            }
        }
        self.response = Some(response);
    }
}

impl AsMut<ScDeployStep> for ScDeployStep {
    fn as_mut(&mut self) -> &mut ScDeployStep {
        self
    }
}

/// Extracts
/// - (optional) recipient (needed for contract upgrade, not yet used);
/// - the arguments.
pub(crate) fn process_contract_deploy<OriginalResult>(
    contract_deploy: ContractDeploy<StaticApi, OriginalResult>,
) -> (Option<String>, Vec<String>) {
    let to_str = contract_deploy
        .to
        .as_option()
        .map(|to| format!("0x{}", hex::encode(to.to_address().as_bytes())));
    let mandos_args = convert_call_args(&contract_deploy.arg_buffer);
    (to_str, mandos_args)
}
