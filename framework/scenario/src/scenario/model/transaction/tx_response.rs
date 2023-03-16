use multiversx_sdk::data::transaction::{ApiLogs, ApiSmartContractResult, TransactionOnNetwork};

use crate::scenario_model::{BytesValue, U64Value};

use super::Log;

#[derive(Debug, Default, Clone)]
pub struct TxResponse {
    pub out: Vec<BytesValue>,
    pub status: U64Value,
    pub message: BytesValue,
    pub logs: Vec<Log>,
    pub gas: U64Value,
    pub refund: U64Value,
    api_scrs: Vec<ApiSmartContractResult>,
    api_logs: Option<ApiLogs>,
}

impl TxResponse {
    pub fn new(tx: TransactionOnNetwork) -> Self {
        Self {
            api_scrs: tx.smart_contract_results.unwrap_or_default(),
            api_logs: tx.logs,
            ..Default::default()
        }
    }
}
