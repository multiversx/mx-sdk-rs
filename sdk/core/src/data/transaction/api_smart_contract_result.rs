use multiversx_chain_core::std::Bech32Address;
use serde::{Deserialize, Serialize};

use super::super::vm::CallType;
use super::api_logs::ApiLogs;

/// Smart contract result with changed fields' types in order to make it friendly for API's json.
///
/// Corresponds to [`ApiSmartContractResult`](https://github.com/multiversx/mx-chain-core-go/blob/main/data/transaction/apiTransactionResult.go) in mx-chain-core-go.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiSmartContractResult {
    #[serde(default)]
    pub hash: String,
    pub nonce: u64,
    pub value: u128, // consider switching to BigUint if this proves insufficient
    pub receiver: Bech32Address,
    pub sender: Bech32Address,
    #[serde(default)]
    pub data: String,
    pub prev_tx_hash: String,
    pub original_tx_hash: String,
    pub gas_limit: u64,
    pub gas_price: u64,
    pub call_type: CallType,
    pub relayer_address: Option<String>,
    pub relayed_value: Option<u128>,
    pub code: Option<String>,
    pub code_metadata: Option<String>,
    pub return_message: Option<String>,
    pub original_sender: Option<String>,
    pub logs: Option<ApiLogs>,
}
