use anyhow::Error;
use multiversx_sc_scenario::imports::Bech32Address;
use multiversx_sdk::gateway::{
    ChainSimulatorAddKeysRequest, ChainSimulatorGenerateBlocksRequest,
    ChainSimulatorSendFundsRequest, ChainSimulatorSetStateOverwriteRequest,
    ChainSimulatorSetStateRequest, GatewayAsyncService, SetStateAccount,
};

use crate::InteractorBase;

impl<GatewayProxy> InteractorBase<GatewayProxy>
where
    GatewayProxy: GatewayAsyncService,
{
    pub async fn send_user_funds(&self, receiver: &Bech32Address) -> Result<String, Error> {
        if !self.use_chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        self.proxy
            .request(ChainSimulatorSendFundsRequest::to_address(receiver))
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

    pub async fn add_key(&self, key: Vec<u8>) -> Result<String, Error> {
        if !self.use_chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        self.proxy
            .request(ChainSimulatorAddKeysRequest::with_keys(vec![key]))
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

    pub async fn set_state(&self, accounts: Vec<SetStateAccount>) -> Result<String, Error> {
        if !self.use_chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        self.proxy
            .request(ChainSimulatorSetStateRequest::for_accounts(accounts))
            .await
    }

    pub async fn set_state_overwrite(
        &self,
        accounts: Vec<SetStateAccount>,
    ) -> Result<String, Error> {
        if !self.use_chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        self.proxy
            .request(ChainSimulatorSetStateOverwriteRequest::for_accounts(
                accounts,
            ))
            .await
    }

    pub async fn set_state_for_saved_accounts(&self) -> Result<String, Error> {
        if !self.use_chain_simulator {
            return Ok(String::from("no-simulator"));
        }

        let accounts = self.get_accounts_from_file();
        self.proxy
            .request(ChainSimulatorSetStateRequest::for_accounts(accounts))
            .await
    }
}
