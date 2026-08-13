use anyhow::{Context, Result};
use multiversx_sdk::{
    chain_core::std::base64_decode,
    data::transaction::{ApiTransactionResult, Transaction},
};
use serde::{Deserialize, Serialize};
use std::path::Path;

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

    /// Populated for deploy transactions: the deterministically-computed contract address.
    #[serde(rename = "contractAddress", skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<String>,

    /// Populated after waiting for the transaction result on-network.
    #[serde(
        rename = "transactionOnNetwork",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_on_network: Option<ApiTransactionResult>,
}

impl TxOutputFile {
    /// Creates an mxpy-compatible output value from a transaction.
    pub fn from_transaction(
        emitted_transaction: Transaction,
        contract_address: Option<String>,
    ) -> Result<Self> {
        let emitted_transaction_data = match &emitted_transaction.data {
            None => String::new(),
            Some(data) => String::from_utf8_lossy(&base64_decode(data)?).into_owned(),
        };

        Ok(Self {
            emitted_transaction,
            emitted_transaction_data,
            emitted_transaction_hash: String::new(),
            contract_address,
            transaction_on_network: None,
        })
    }

    /// Write this output to `outfile`, or print to stdout when no outfile is given.
    pub fn save_output(&self, outfile: Option<&Path>) -> Result<()> {
        let json = to_json_pretty(self)?;
        if let Some(path) = outfile {
            std::fs::write(path, &json)
                .with_context(|| format!("failed to write to {}", path.display()))?;
            println!("Transaction saved to {}", path.display());
        } else {
            println!("{json}");
        }
        Ok(())
    }
}

fn to_json_pretty<T: Serialize>(value: &T) -> Result<String> {
    let mut buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut serializer = serde_json::Serializer::with_formatter(&mut buf, formatter);
    value
        .serialize(&mut serializer)
        .context("failed to serialize transaction")?;
    String::from_utf8(buf).context("non-UTF8 in serialized JSON")
}
