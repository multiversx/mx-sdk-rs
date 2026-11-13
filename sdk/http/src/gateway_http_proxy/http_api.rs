use anyhow::Result;
use multiversx_sdk::{
    chain_core::std::Bech32Address,
    data::api::{
        ApiAccount, ApiAccountListItem, ApiEvent, ApiTransaction, ApiTransactionListItem,
        ApiUpgrade,
    },
    gateway::{
        ApiCountContractsRequest, ApiGetAccountRequest, ApiGetEventsRequest, ApiGetTxRequest,
        ApiGetUpgradesRequest, ApiListContractsRequest, ApiListTransactionsRequest, ApiSortOrder,
    },
};

use super::GatewayHttpProxy;

/// Queries for the MultiversX API (as opposed to the gateway).
/// The proxy must be created with the API URL, e.g. `MAINNET_API`.
impl GatewayHttpProxy {
    /// Retrieves the total number of smart contracts.
    pub async fn api_count_contracts(&self) -> Result<usize> {
        self.http_request(ApiCountContractsRequest).await
    }

    /// Retrieves a page of smart contract addresses, sorted by timestamp.
    /// `search_after` is the cursor of the last item of the previous page (`None` for the first page).
    pub async fn api_list_contracts(
        &self,
        size: usize,
        order: ApiSortOrder,
        search_after: Option<String>,
    ) -> Result<Vec<ApiAccountListItem>> {
        self.http_request(ApiListContractsRequest::new(size, order, search_after))
            .await
    }

    /// Retrieves an account, including its code and deploy transaction.
    pub async fn api_get_account(&self, address: &Bech32Address) -> Result<ApiAccount> {
        self.http_request(ApiGetAccountRequest::new(address)).await
    }

    /// Retrieves an account without its code: code hash, owner and deploy transaction.
    pub async fn api_get_account_without_code(
        &self,
        address: &Bech32Address,
    ) -> Result<ApiAccount> {
        self.http_request(ApiGetAccountRequest::new(address).without_code())
            .await
    }

    /// Retrieves the upgrade history of a contract.
    pub async fn api_get_upgrades(&self, address: &Bech32Address) -> Result<Vec<ApiUpgrade>> {
        self.http_request(ApiGetUpgradesRequest::new(address)).await
    }

    /// Retrieves a transaction, with status, timestamp and data.
    pub async fn api_get_transaction(&self, hash: &str) -> Result<ApiTransaction> {
        self.http_request(ApiGetTxRequest::new(hash)).await
    }

    /// Retrieves a page of events with the given identifier (e.g. `SCUpgrade`), newest first.
    /// `search_after` is the cursor of the last event of the previous page (`None` for the first page).
    pub async fn api_get_events(
        &self,
        identifier: &str,
        size: usize,
        search_after: Option<String>,
    ) -> Result<Vec<ApiEvent>> {
        self.http_request(ApiGetEventsRequest::new(identifier, size, search_after))
            .await
    }

    /// Retrieves a page of transactions matching the request's filters (see [`ApiListTransactionsRequest`]).
    pub async fn api_list_transactions(
        &self,
        request: ApiListTransactionsRequest,
    ) -> Result<Vec<ApiTransactionListItem>> {
        self.http_request(request).await
    }
}
