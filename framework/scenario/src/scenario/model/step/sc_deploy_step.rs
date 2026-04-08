use multiversx_sc::types::H256;

use crate::{
    scenario::model::{AddressValue, BigUintValue, BytesValue, TxDeploy, TxExpect, U64Value},
    scenario_model::TxResponse,
};

use crate::multiversx_sc::types::CodeMetadata;

#[derive(Debug, Clone)]
pub struct ScDeployStep {
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

    pub fn get_tx_id(&self) -> &str {
        if let Some(tx_id) = &self.tx_id {
            tx_id
        } else {
            ""
        }
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

    pub fn save_response(&mut self, mut tx_response: TxResponse) {
        if let Some(expect) = &mut self.expect {
            if expect.build_from_response {
                expect.update_from_response(&tx_response)
            }
        }
        if tx_response.tx_hash.is_none() {
            tx_response.tx_hash = self
                .explicit_tx_hash
                .as_ref()
                .map(|vm_hash| vm_hash.as_array().into());
        }
        self.response = Some(tx_response);
    }
}

impl AsMut<ScDeployStep> for ScDeployStep {
    fn as_mut(&mut self) -> &mut ScDeployStep {
        self
    }
}
