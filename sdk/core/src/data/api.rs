use multiversx_chain_core::std::Bech32Address;
use serde::{Deserialize, Serialize};

/// Item of the accounts list of the MultiversX API (`/accounts`), with only the address and cursor requested.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiAccountListItem {
    pub address: Bech32Address,

    /// Pagination cursor (base64): pass it as `searchAfter` to get the items after this one.
    pub search_after: Option<String>,
}

/// Account as returned by the MultiversX API (`/accounts/{address}`).
///
/// Unlike the gateway, the API returns the response unwrapped, without a `data` envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiAccount {
    pub address: Bech32Address,

    /// Contract code, hex-encoded.
    pub code: Option<String>,

    /// Hash of the contract code (blake2b-256), base64-encoded.
    pub code_hash: Option<String>,
    pub deploy_tx_hash: Option<String>,

    /// Deploy timestamp, in seconds.
    pub deployed_at: Option<u64>,
    pub owner_address: Option<Bech32Address>,
}

/// Entry of the upgrade history of a contract (`/accounts/{address}/upgrades`), newest first.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiUpgrade {
    /// The account that performed the upgrade.
    pub address: Option<Bech32Address>,
    pub tx_hash: String,

    /// Hash of the new contract code (blake2b-256), base64-encoded.
    pub code_hash: Option<String>,
    pub timestamp: u64,
}

/// Transaction as returned by the MultiversX API (`/transactions/{hash}`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiTransaction {
    pub tx_hash: Option<String>,
    pub status: String,
    pub timestamp: u64,

    /// Transaction data field, base64-encoded.
    pub data: Option<String>,
}

/// Item of the transactions list of the MultiversX API (`/transactions`), with the fields
/// requested by [`ApiListTransactionsRequest`](crate::gateway::ApiListTransactionsRequest).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiTransactionListItem {
    pub tx_hash: String,
    pub sender: Bech32Address,
    pub receiver: Bech32Address,
    pub nonce: u64,

    /// Timestamp, in seconds.
    pub timestamp: u64,

    /// `success`, `fail`, `invalid` or `pending`, as computed by the API (from the transaction's events).
    pub status: String,

    /// The function called, e.g. `upgradeContract`, or `scDeploy` for deploys.
    pub function: Option<String>,

    /// The transaction data, base64-encoded, when requested with
    /// [`ApiListTransactionsRequest::with_data`](crate::gateway::ApiListTransactionsRequest::with_data).
    #[serde(default)]
    pub data: Option<String>,

    /// The transaction's events (e.g. `signalError`, with the failure message), when requested
    /// with [`ApiListTransactionsRequest::with_logs`](crate::gateway::ApiListTransactionsRequest::with_logs).
    #[serde(default)]
    pub logs: Option<crate::data::transaction::ApiLogs>,

    /// Pagination cursor (base64): pass it as `searchAfter` to get the items after this one.
    pub search_after: Option<String>,
}

/// Event as returned by the MultiversX API (`/events`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiEvent {
    /// Hash of the transaction or smart contract result that emitted the event.
    /// For smart contract results, it has the form `<tx hash>-<shard>-<index>`.
    pub tx_hash: String,

    /// Address of the contract the event is about.
    pub address: Bech32Address,

    /// Address in whose logs the event appears (e.g. the calling contract).
    pub log_address: Option<Bech32Address>,
    pub identifier: String,

    /// Event topics, hex-encoded.
    #[serde(default)]
    pub topics: Vec<String>,

    /// Timestamp, in seconds.
    pub timestamp: u64,

    /// Pagination cursor (base64): pass it as `searchAfter` to get the events after this one.
    pub search_after: Option<String>,
}
