use anyhow::Result;
use multiversx_sdk::{
    data::hyperblock::HyperBlock,
    gateway::{GetHyperBlockRequest, NetworkStatusRequest},
};

use super::GatewayProxy;

impl GatewayProxy {
    // get_hyper_block_by_hash retrieves a hyper block's info by hash from the network
    pub async fn get_hyper_block_by_hash(&self, hash: &str) -> Result<HyperBlock> {
        self.request(GetHyperBlockRequest::by_hash(hash)).await
    }

    // get_hyper_block_by_nonce retrieves a hyper block's info by nonce from the network
    pub async fn get_hyper_block_by_nonce(&self, nonce: u64) -> Result<HyperBlock> {
        self.request(GetHyperBlockRequest::by_nonce(nonce)).await
    }

    // get_latest_hyper_block_nonce retrieves the latest hyper block (metachain) nonce from the network
    pub async fn get_latest_hyper_block_nonce(&self) -> Result<u64> {
        let network_status = self.request(NetworkStatusRequest::default()).await?;
        Ok(network_status.nonce)
    }
}
