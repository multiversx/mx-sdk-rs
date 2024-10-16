use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use super::{
    GatewayRequest, GatewayRequestType, GENERATE_BLOCKS_ENDPOINT,
    GENERATE_BLOCKS_UNTIL_EPOCH_REACHED_ENDPOINT, GENERATE_BLOCKS_UNTIL_TX_PROCESSED_ENDPOINT,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateBlocksResponse {
    pub data: serde_json::Value,
    pub error: String,
    pub code: String,
}

/// Generates blocks using the chain simulator API.
pub struct ChainSimulatorGenerateBlocksRequest {
    pub query: String,
}

impl ChainSimulatorGenerateBlocksRequest {
    pub fn num_blocks(num_blocks: u64) -> Self {
        Self {
            query: format!("{}/{}", GENERATE_BLOCKS_ENDPOINT, num_blocks),
        }
    }

    pub fn until_epoch(epoch_number: u64) -> Self {
        Self {
            query: format!(
                "{}/{}",
                GENERATE_BLOCKS_UNTIL_EPOCH_REACHED_ENDPOINT, epoch_number
            ),
        }
    }

    /// TODO: convert arg to H256
    pub fn until_tx_processed(tx_hash: &str) -> Self {
        Self {
            query: format!(
                "{}/{}",
                GENERATE_BLOCKS_UNTIL_TX_PROCESSED_ENDPOINT, tx_hash
            ),
        }
    }
}

impl GatewayRequest for ChainSimulatorGenerateBlocksRequest {
    type Payload = ();
    type DecodedJson = GenerateBlocksResponse;
    type Result = String;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Post
    }

    fn get_endpoint(&self) -> String {
        self.query.clone()
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        match decoded.code.as_str() {
            "successful" => Ok(decoded.code),
            _ => Err(anyhow!("{}", decoded.error)),
        }
    }
}
