use crate::data::{
    hyperblock::{HyperBlock, HyperBlockResponse},
    network_status::NetworkStatusResponse,
};
use anyhow::{anyhow, Result};

use super::GatewayProxy;
use super::METACHAIN_SHARD_ID;

const GET_HYPER_BLOCK_BY_NONCE_ENDPOINT: &str = "hyperblock/by-nonce/";
const GET_HYPER_BLOCK_BY_HASH_ENDPOINT: &str = "hyperblock/by-hash/";
const GET_NETWORK_STATUS_ENDPOINT: &str = "network/status";

impl GatewayProxy {
    async fn get_hyper_block(&self, endpoint: &str) -> Result<HyperBlock> {
        let endpoint = self.get_endpoint(endpoint);
        let resp = self
            .client
            .get(endpoint)
            .send()
            .await?
            .json::<HyperBlockResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.hyperblock),
        }
    }

    // get_hyper_block_by_hash retrieves a hyper block's info by hash from the network
    pub async fn get_hyper_block_by_hash(&self, hash: &str) -> Result<HyperBlock> {
        let endpoint = GET_HYPER_BLOCK_BY_HASH_ENDPOINT.to_string() + hash;
        self.get_hyper_block(endpoint.as_str()).await
    }

    // get_hyper_block_by_nonce retrieves a hyper block's info by nonce from the network
    pub async fn get_hyper_block_by_nonce(&self, nonce: u64) -> Result<HyperBlock> {
        let endpoint = GET_HYPER_BLOCK_BY_NONCE_ENDPOINT.to_string() + nonce.to_string().as_str();
        self.get_hyper_block(endpoint.as_str()).await
    }

    // get_latest_hyper_block_nonce retrieves the latest hyper block (metachain) nonce from the network
    pub async fn get_latest_hyper_block_nonce(&self, with_metachain: bool) -> Result<u64> {
        let mut endpoint = GET_NETWORK_STATUS_ENDPOINT.to_string();

        if with_metachain {
            endpoint = format!("{GET_NETWORK_STATUS_ENDPOINT}/{METACHAIN_SHARD_ID}");
        }

        let endpoint = self.get_endpoint(endpoint.as_str());

        let resp = self
            .client
            .get(endpoint)
            .send()
            .await?
            .json::<NetworkStatusResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.status.nonce),
        }
    }
}
