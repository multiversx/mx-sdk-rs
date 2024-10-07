use anyhow::Error;
use multiversx_sdk::{
    data::sdk_address::SdkAddress,
    gateway::{
        ChainSimulatorGenerateBlocksRequest, ChainSimulatorSendFundsRequest, GatewayAsyncService,
    },
};

use crate::InteractorBase;

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    pub async fn send_user_funds(&self, receiver: &SdkAddress) -> Result<String, Error> {
        if !self.use_chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        self.proxy
            .request(ChainSimulatorSendFundsRequest::to_address(
                receiver.to_bech32_string().unwrap(),
            ))
            .await
    }

    pub async fn generate_blocks(&self, num_blocks: u64) -> Result<String, Error> {
        if !self.use_chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        self.proxy
            .request(ChainSimulatorGenerateBlocksRequest::num_blocks(num_blocks))
            .await
    }

    pub async fn generate_blocks_until_epoch(&self, epoch_number: u64) -> Result<String, Error> {
        if !self.use_chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        self.proxy
            .request(ChainSimulatorGenerateBlocksRequest::until_epoch(
                epoch_number,
            ))
            .await
    }

    pub async fn generate_blocks_until_tx_processed(&self, tx_hash: &str) -> Result<String, Error> {
        if !self.use_chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        self.proxy
            .request(ChainSimulatorGenerateBlocksRequest::until_tx_processed(
                tx_hash,
            ))
            .await
    }
}
