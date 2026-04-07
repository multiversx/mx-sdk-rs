use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Holds the data which is returned when sending a bulk of transactions.
///
/// Corresponds to [`MultipleTransactionsResponseData`](https://github.com/multiversx/mx-chain-proxy-go/blob/master/data/transaction.go) in mx-chain-proxy-go.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipleTransactionsResponseData {
    #[serde(rename = "txsSent")]
    pub num_of_sent_txs: u64,
    #[serde(rename = "txsHashes")]
    pub txs_hashes: HashMap<u64, String>,
}

/// Defines a response from the node holding the number of transactions sent to the chain.
///
/// Corresponds to [`ResponseMultipleTransactions`](https://github.com/multiversx/mx-chain-proxy-go/blob/master/data/transaction.go) in mx-chain-proxy-go.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMultipleTransactions {
    pub error: String,
    pub code: String,
    pub data: Option<MultipleTransactionsResponseData>,
}
