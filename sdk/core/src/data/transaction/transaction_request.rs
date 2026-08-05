use crate::wallet::WalletSignature;
use multiversx_chain_core::std::Bech32Address;
use serde::{Deserialize, Serialize};

use super::transaction_options::TransactionOptions;
use super::transaction_version::TransactionVersion;

/// Represents the structure that maps and validates user input for publishing a new transaction.
///
/// Corresponds to [`Transaction`](https://github.com/multiversx/mx-chain-proxy-go/blob/master/data/transaction.go) in mx-chain-proxy-go.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub nonce: u64,
    pub value: String,
    pub receiver: Bech32Address,
    pub sender: Bech32Address,
    pub gas_price: u64,
    pub gas_limit: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<WalletSignature>,
    #[serde(rename = "chainID")]
    pub chain_id: String,
    pub version: TransactionVersion,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<TransactionOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relayer: Option<Bech32Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relayer_signature: Option<WalletSignature>,
}

impl Transaction {
    pub fn should_sign_with_hash(&self) -> bool {
        if !self.version.supports_options() {
            return false;
        }

        self.options
            .as_ref()
            .map(|o| o.sign_with_hash())
            .unwrap_or(false)
    }
}
