use multiversx_sc::types::H256;

use crate::{
    scenario::model::{AddressValue, BytesValue, TxExpect, TxQuery},
    scenario_model::TxResponse,
};

#[derive(Debug, Clone)]
pub struct ScQueryStep {
    pub id: String,
    pub tx_id: Option<String>,
    pub explicit_tx_hash: Option<H256>,
    pub comment: Option<String>,
    pub tx: Box<TxQuery>,
    pub expect: Option<TxExpect>,
    pub response: Option<TxResponse>,
}

impl Default for ScQueryStep {
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

impl ScQueryStep {
    pub fn new() -> Self {
        Self::default()
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

    pub fn argument(mut self, expr: &str) -> Self {
        self.tx.arguments.push(BytesValue::from(expr));
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
            .expect("SC query response not yet available")
    }

    pub fn save_response(&mut self, tx_response: TxResponse) {
        if let Some(expect) = &mut self.expect {
            if expect.build_from_response {
                expect.update_from_response(&tx_response)
            }
        }
        self.response = Some(tx_response);
    }
}
