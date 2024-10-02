use super::GatewayHttpProxy;
use anyhow::Error;
use multiversx_sdk::gateway::{
    ChainSimulatorGenerateBlocksRequest, ChainSimulatorSendFundsRequest, GatewayAsyncService,
};

impl GatewayHttpProxy {
    pub async fn send_user_funds(&self, receiver: &str) -> Result<String, Error> {
        if !self.chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        self.request(ChainSimulatorSendFundsRequest::to_address(
            receiver.to_owned(),
        ))
        .await
    }

    pub async fn generate_blocks(&self, num_blocks: u64) -> Result<String, Error> {
        if !self.chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        self.request(ChainSimulatorGenerateBlocksRequest::num_blocks(num_blocks))
            .await
    }

    pub async fn generate_blocks_until_epoch(&self, epoch_number: u64) -> Result<String, Error> {
        if !self.chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        self.request(ChainSimulatorGenerateBlocksRequest::until_epoch(
            epoch_number,
        ))
        .await
    }

    pub async fn generate_blocks_until_tx_processed(&self, tx_hash: &str) -> Result<String, Error> {
        if !self.chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        self.request(ChainSimulatorGenerateBlocksRequest::until_tx_processed(
            tx_hash,
        ))
        .await
    }
}