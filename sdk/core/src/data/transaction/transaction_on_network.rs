use multiversx_chain_core::std::Bech32Address;
use serde::{Deserialize, Serialize};

use super::api_logs::ApiLogs;
use super::api_smart_contract_result::ApiSmartContractResult;

// TransactionOnNetwork holds a transaction's info entry in a hyper block
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionOnNetwork {
    #[serde(rename = "type")]
    pub kind: String,
    pub hash: Option<String>,
    pub nonce: u64,
    pub round: u64,
    pub epoch: u64,
    pub value: String,
    pub receiver: Bech32Address,
    pub sender: Bech32Address,
    pub gas_price: u64,
    pub gas_limit: u64,
    #[serde(default)]
    pub gas_used: u64,
    #[serde(default)]
    pub signature: String,
    pub source_shard: u32,
    pub destination_shard: u32,
    #[serde(default)]
    pub block_nonce: u64,
    #[serde(default)]
    pub block_hash: String,
    pub notarized_at_source_in_meta_nonce: Option<u64>,
    #[serde(rename = "NotarizedAtSourceInMetaHash")]
    pub notarized_at_source_in_meta_hash: Option<String>,
    pub notarized_at_destination_in_meta_nonce: Option<u64>,
    pub notarized_at_destination_in_meta_hash: Option<String>,
    pub processing_type_on_destination: String,
    #[serde(default)]
    pub miniblock_type: String,
    #[serde(default)]
    pub miniblock_hash: String,
    #[serde(default)]
    pub timestamp: u64,
    pub data: Option<String>,
    pub status: String,
    pub hyperblock_nonce: Option<u64>,
    pub hyperblock_hash: Option<String>,
    #[serde(default)]
    pub smart_contract_results: Vec<ApiSmartContractResult>,
    pub logs: Option<ApiLogs>,
}
