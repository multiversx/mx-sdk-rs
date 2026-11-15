use multiversx_chain_core::std::Bech32Address;
use serde::{Deserialize, Serialize};

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
