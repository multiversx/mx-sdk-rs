use multiversx_chain_core::std::Bech32Address;
use serde::{Deserialize, Serialize};

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
