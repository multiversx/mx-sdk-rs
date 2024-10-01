use anyhow::Result;
use multiversx_sdk::{
    data::{
        address::Address,
        network_config::NetworkConfig,
        transaction::{
            ArgCreateTransaction, Transaction, TransactionOnNetwork, TxCostResponseData,
        },
        vm::{VmValueRequest, VmValuesResponseData},
    },
    gateway::{
        GetTxCost, GetTxInfo, GetTxProcessStatus, GetTxStatus, SendMultiTxRequest, SendTxRequest,
        VMQueryRequest,
    },
};

use super::GatewayProxy;

impl GatewayProxy {
    // request_transaction_cost retrieves how many gas a transaction will consume
    pub async fn request_transaction_cost(&self, tx: &Transaction) -> Result<TxCostResponseData> {
        self.request(GetTxCost(tx)).await
    }

    // get_transaction_info retrieves a transaction's details from the network
    pub async fn get_transaction_info(&self, hash: &str) -> Result<TransactionOnNetwork> {
        self.request(GetTxInfo::new(hash)).await
    }

    // get_transaction_info_with_results retrieves a transaction's details from the network with events
    pub async fn get_transaction_info_with_results(
        &self,
        hash: &str,
    ) -> Result<TransactionOnNetwork> {
        self.request(GetTxInfo::new(hash).with_results()).await
    }

    // get_transaction_status retrieves a transaction's status from the network
    pub async fn get_transaction_status(&self, hash: &str) -> Result<String> {
        self.request(GetTxStatus::new(hash)).await
    }

    // get_transaction_process_status retrieves a transaction's status from the network using process-status API
    pub async fn get_transaction_process_status(&self, hash: &str) -> Result<(String, String)> {
        self.request(GetTxProcessStatus::new(hash)).await
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
        self.request(SendTxRequest(tx)).await
    }

    #[allow(clippy::ptr_arg)]
    pub async fn send_transactions(&self, txs: &Vec<Transaction>) -> Result<Vec<String>> {
        self.request(SendMultiTxRequest(txs)).await
    }

    // execute_vmquery retrieves data from existing SC trie through the use of a VM
    pub async fn execute_vmquery(
        &self,
        vm_request: &VmValueRequest,
    ) -> Result<VmValuesResponseData> {
        self.request(VMQueryRequest(vm_request)).await
    }
}
