use multiversx_chain_core::std::Bech32Address;
use serde::{Deserialize, Serialize};

/// Represents the structure that maps and validates user input for publishing a new transaction.
///
/// Corresponds to [`Transaction`](https://github.com/multiversx/mx-chain-proxy-go/blob/master/data/transaction.go) in mx-chain-proxy-go.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub nonce: u64,
    pub value: String,
    pub receiver: Bech32Address,
    pub sender: Bech32Address,
    pub gas_price: u64,
    pub gas_limit: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(rename = "chainID")]
    pub chain_id: String,
    pub version: u32,
    #[serde(skip_serializing_if = "is_zero")]
    pub options: u32,
}

/// This is only used for serialize
#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_zero(num: &u32) -> bool {
    *num == 0
}
