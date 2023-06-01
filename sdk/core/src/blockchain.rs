use std::collections::HashMap;

use crate::data::{
    account::{Account, AccountResponse},
    account_storage::AccountStorageResponse,
    address::Address,
    esdt::{EsdtBalance, EsdtBalanceResponse, EsdtRolesResponse},
    hyperblock::{HyperBlock, HyperBlockResponse},
    network_config::{NetworkConfig, NetworkConfigResponse},
    network_economics::{NetworkEconomics, NetworkEconomicsResponse},
    network_status::NetworkStatusResponse,
    transaction::{
        ArgCreateTransaction, ResponseTxCost, SendTransactionResponse, SendTransactionsResponse,
        Transaction, TransactionInfo, TransactionOnNetwork, TransactionStatus, TxCostResponseData,
    },
    vm::{ResponseVmValue, VmValueRequest, VmValuesResponseData},
};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use reqwest::Client;

pub const MAINNET_GATEWAY: &str = "https://gateway.multiversx.com";
pub const TESTNET_GATEWAY: &str = "https://testnet-gateway.multiversx.com";
pub const DEVNET_GATEWAY: &str = "https://devnet-gateway.multiversx.com";

// MetachainShardId will be used to identify a shard ID as metachain
pub const METACHAIN_SHARD_ID: u32 = 0xFFFFFFFF;

const NETWORK_CONFIG_ENDPOINT: &str = "network/config";
const NETWORK_ECONOMICS_ENDPOINT: &str = "network/economics";
const ACCOUNT_ENDPOINT: &str = "address/";
const KEYS_ENDPOINT: &str = "/keys/";
const COST_TRANSACTION_ENDPOINT: &str = "transaction/cost";
const SEND_TRANSACTION_ENDPOINT: &str = "transaction/send";
const SEND_MULTIPLE_TRANSACTIONS_ENDPOINT: &str = "transaction/send-multiple";
const GET_TRANSACTION_INFO_ENDPOINT: &str = "transaction/";
const GET_HYPER_BLOCK_BY_NONCE_ENDPOINT: &str = "hyperblock/by-nonce/";
const GET_HYPER_BLOCK_BY_HASH_ENDPOINT: &str = "hyperblock/by-hash/";
const GET_NETWORK_STATUS_ENDPOINT: &str = "network/status";
const WITH_RESULTS_QUERY_PARAM: &str = "?withResults=true";
const VM_VALUES_ENDPOINT: &str = "vm-values/query";

#[derive(Clone, Debug)]
pub struct CommunicationProxy {
    proxy_url: String,
    client: Client,
}

impl CommunicationProxy {
    pub fn new(proxy_url: String) -> Self {
        Self {
            proxy_url,
            client: Client::new(),
        }
    }

    fn get_endpoint(&self, endpoint: &str) -> String {
        format!("{}/{}", self.proxy_url, endpoint)
    }

    // get_network_config retrieves the network configuration from the proxy
    pub async fn get_network_config(&self) -> Result<NetworkConfig> {
        let endpoint = self.get_endpoint(NETWORK_CONFIG_ENDPOINT);
        let resp = self
            .client
            .get(endpoint)
            .send()
            .await?
            .json::<NetworkConfigResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.config),
        }
    }

    // get_network_economics retrieves the network economics from the proxy
    pub async fn get_network_economics(&self) -> Result<NetworkEconomics> {
        let endpoint = self.get_endpoint(NETWORK_ECONOMICS_ENDPOINT);
        let resp = self
            .client
            .get(endpoint)
            .send()
            .await?
            .json::<NetworkEconomicsResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.metrics),
        }
    }

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

    // request_transaction_cost retrieves how many gas a transaction will consume
    pub async fn request_transaction_cost(&self, tx: &Transaction) -> Result<TxCostResponseData> {
        let endpoint = self.get_endpoint(COST_TRANSACTION_ENDPOINT);
        let resp = self
            .client
            .post(endpoint)
            .json(tx)
            .send()
            .await?
            .json::<ResponseTxCost>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b),
        }
    }

    // get_account retrieves an account info from the network (nonce, balance)
    pub async fn get_account(&self, address: &Address) -> Result<Account> {
        if !address.is_valid() {
            return Err(anyhow!("invalid address"));
        }

        let endpoint = ACCOUNT_ENDPOINT.to_string() + address.to_string().as_str();
        let endpoint = self.get_endpoint(endpoint.as_str());
        let resp = self
            .client
            .get(endpoint)
            .send()
            .await?
            .json::<AccountResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.account),
        }
    }

    // get_account_esdt_roles retrieves an all esdt roles of an account from the network
    pub async fn get_account_esdt_roles(
        &self,
        address: &Address,
    ) -> Result<HashMap<String, Vec<String>>> {
        if !address.is_valid() {
            return Err(anyhow!("invalid address"));
        }

        let endpoint = ACCOUNT_ENDPOINT.to_string() + address.to_string().as_str() + "/esdts/roles";
        let endpoint = self.get_endpoint(endpoint.as_str());
        let resp = self
            .client
            .get(endpoint)
            .send()
            .await?
            .json::<EsdtRolesResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.roles),
        }
    }

    // get_account_esdt_tokens retrieves an all esdt token of an account from the network
    pub async fn get_account_esdt_tokens(
        &self,
        address: &Address,
    ) -> Result<HashMap<String, EsdtBalance>> {
        if !address.is_valid() {
            return Err(anyhow!("invalid address"));
        }

        let endpoint = ACCOUNT_ENDPOINT.to_string() + address.to_string().as_str() + "/esdt";
        let endpoint = self.get_endpoint(endpoint.as_str());
        let resp = self
            .client
            .get(endpoint)
            .send()
            .await?
            .json::<EsdtBalanceResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.esdts),
        }
    }

    // get_account_esdt_tokens retrieves an all esdt token of an account from the network
    pub async fn get_account_storage_keys(
        &self,
        address: &Address,
    ) -> Result<HashMap<String, String>> {
        if !address.is_valid() {
            return Err(anyhow!("invalid address"));
        }

        let endpoint = ACCOUNT_ENDPOINT.to_string() + address.to_string().as_str() + KEYS_ENDPOINT;
        let endpoint = self.get_endpoint(endpoint.as_str());
        let resp = self
            .client
            .get(endpoint)
            .send()
            .await?
            .json::<AccountStorageResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.pairs),
        }
    }

    async fn get_transaction_info_internal(
        &self,
        hash: &str,
        with_results: bool,
    ) -> Result<TransactionOnNetwork> {
        let mut endpoint = GET_TRANSACTION_INFO_ENDPOINT.to_string() + hash;

        if with_results {
            endpoint += WITH_RESULTS_QUERY_PARAM
        }

        let endpoint = self.get_endpoint(endpoint.as_str());
        let resp = self
            .client
            .get(endpoint)
            .send()
            .await?
            .json::<TransactionInfo>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.transaction),
        }
    }

    // get_transaction_info retrieves a transaction's details from the network
    pub async fn get_transaction_info(&self, hash: &str) -> Result<TransactionOnNetwork> {
        self.get_transaction_info_internal(hash, false).await
    }

    // get_transaction_info_with_results retrieves a transaction's details from the network with events
    pub async fn get_transaction_info_with_results(
        &self,
        hash: &str,
    ) -> Result<TransactionOnNetwork> {
        self.get_transaction_info_internal(hash, true).await
    }

    // get_transaction_status retrieves a transaction's status from the network
    pub async fn get_transaction_status(&self, hash: &str) -> Result<String> {
        let endpoint = format!("transaction/{hash}/status");
        let endpoint = self.get_endpoint(endpoint.as_str());

        let resp = self
            .client
            .get(endpoint)
            .send()
            .await?
            .json::<TransactionStatus>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.status),
        }
    }

    // get_default_transaction_arguments will prepare the transaction creation argument by querying the account's info
    pub async fn get_default_transaction_arguments(
        &self,
        address: &Address,
        network_configs: &NetworkConfig,
    ) -> Result<ArgCreateTransaction> {
        let account = self.get_account(address).await?;

        Ok(ArgCreateTransaction {
            nonce: account.nonce,
            value: "".to_string(),
            rcv_addr: address.clone(),
            snd_addr: address.clone(),
            gas_price: network_configs.min_gas_price,
            gas_limit: network_configs.min_gas_limit,
            data: None,
            signature: "".to_string(),
            chain_id: network_configs.chain_id.clone(),
            version: network_configs.min_transaction_version,
            options: 0,
            available_balance: account.balance,
        })
    }

    pub async fn send_transaction(&self, tx: &Transaction) -> Result<String> {
        let endpoint = self.get_endpoint(SEND_TRANSACTION_ENDPOINT);
        let resp = self
            .client
            .post(endpoint)
            .json(tx)
            .send()
            .await?
            .json::<SendTransactionResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.tx_hash),
        }
    }

    pub async fn send_transactions(&self, txs: &Vec<Transaction>) -> Result<Vec<String>> {
        let endpoint = self.get_endpoint(SEND_MULTIPLE_TRANSACTIONS_ENDPOINT);
        let resp = self
            .client
            .post(endpoint)
            .json(txs)
            .send()
            .await?
            .json::<SendTransactionsResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => {
                let mut tx_hashs: Vec<String> = vec![];
                for key in b.txs_hashes.keys().sorted() {
                    tx_hashs.push(b.txs_hashes[key].clone());
                }

                Ok(tx_hashs)
            },
        }
    }

    // execute_vmquery retrieves data from existing SC trie through the use of a VM
    pub async fn execute_vmquery(
        &self,
        vm_request: &VmValueRequest,
    ) -> Result<VmValuesResponseData> {
        let endpoint = self.get_endpoint(VM_VALUES_ENDPOINT);
        let resp = self
            .client
            .post(endpoint)
            .json(vm_request)
            .send()
            .await?
            .json::<ResponseVmValue>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b),
        }
    }
}
