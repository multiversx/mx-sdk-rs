use multiversx_sc_snippets::sdk::data::transaction::{ApiTransactionResult, Transaction};
use serde::{Deserialize, Serialize};

/// mxpy-compatible output format for a signed transaction.
///
/// When saved to a file this can be re-loaded by `sc-meta tx send --infile <path>`
/// or by `mxpy tx send --infile <path>`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxOutputFile {
    #[serde(rename = "emittedTransaction")]
    pub emitted_transaction: Transaction,

    /// Decoded (plain-text / UTF-8) representation of the transaction data field.
    #[serde(rename = "emittedTransactionData")]
    pub emitted_transaction_data: String,

    /// Populated after a successful broadcast; empty string when just serializing.
    #[serde(rename = "emittedTransactionHash")]
    pub emitted_transaction_hash: String,

    /// Populated after waiting for the transaction result on-network.
    #[serde(
        rename = "transactionOnNetwork",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_on_network: Option<ApiTransactionResult>,
}
