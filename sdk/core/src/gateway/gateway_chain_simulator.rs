use std::collections::HashMap;

use super::GatewayProxy;
use anyhow::{anyhow, Error};
use serde::{Deserialize, Serialize};

const SEND_USER_FUNDS_ENDPOINT: &str = "transaction/send-user-funds";
const GENERATE_BLOCKS_ENDPOINT: &str = "simulator/generate-blocks";
const GENERATE_BLOCKS_UNTIL_TX_PROCESSED_ENDPOINT: &str =
    "simulator/generate-blocks-until-transaction-processed";
const GENERATE_BLOCKS_UNTIL_EPOCH_REACHED_ENDPOINT: &str =
    "simulator/generate-blocks-until-epoch-reached";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateBlocksResponse {
    pub data: serde_json::Value,
    pub error: String,
    pub code: String,
}

impl GatewayProxy {
    pub async fn send_user_funds(&self, receiver: &String) -> Result<String, Error> {
        if !self.chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        let mut r = HashMap::new();
        r.insert("receiver", receiver);
        let endpoint_funds = self.get_endpoint(SEND_USER_FUNDS_ENDPOINT);
        let resp = self
            .client
            .post(endpoint_funds)
            .json(&r)
            .send()
            .await?
            .json::<GenerateBlocksResponse>()
            .await?;

        match resp.code.as_str() {
            "successful" => Ok(resp.code),
            _ => Err(anyhow!("{}", resp.error)),
        }
    }

    pub async fn generate_blocks(&self, number_blocks: u64) -> Result<String, Error> {
        if !self.chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        let uri_gen_blocks: String = format!("{}/{}", GENERATE_BLOCKS_ENDPOINT, number_blocks);
        let endpoint_blocks = self.get_endpoint(&uri_gen_blocks);
        let resp = self
            .client
            .post(endpoint_blocks)
            .send()
            .await?
            .json::<GenerateBlocksResponse>()
            .await?;

        match resp.code.as_str() {
            "successful" => Ok(resp.code),
            _ => Err(anyhow!("{}", resp.error)),
        }
    }

    pub async fn generate_blocks_until_epoch(&self, epoch_number: u64) -> Result<String, Error> {
        if !self.chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        let uri_gen_blocks_until_reached_epoch: String = format!(
            "{}/{}",
            GENERATE_BLOCKS_UNTIL_EPOCH_REACHED_ENDPOINT, epoch_number
        );
        let endpoint_blocks = self.get_endpoint(&uri_gen_blocks_until_reached_epoch);
        let resp = self
            .client
            .post(endpoint_blocks)
            .send()
            .await?
            .json::<GenerateBlocksResponse>()
            .await?;

        match resp.code.as_str() {
            "successful" => Ok(resp.code),
            _ => Err(anyhow!("{}", resp.error)),
        }
    }

    pub async fn generate_blocks_until_tx_processed(&self, tx: &String) -> Result<String, Error> {
        if !self.chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        let url_gen_blocks_until_tx_processed: String =
            format!("{}/{}", GENERATE_BLOCKS_UNTIL_TX_PROCESSED_ENDPOINT, tx);
        let endpoint_blocks = self.get_endpoint(&url_gen_blocks_until_tx_processed);
        let resp = self
            .client
            .post(endpoint_blocks)
            .send()
            .await?
            .json::<GenerateBlocksResponse>()
            .await?;

        match resp.code.as_str() {
            "successful" => Ok(resp.code),
            _ => Err(anyhow!("{}", resp.error)),
        }
    }
}
