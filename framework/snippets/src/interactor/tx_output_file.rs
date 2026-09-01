use anyhow::{Context, Result};
use multiversx_sdk::{
    chain_core::{
        std::{base64_decode, new_address::compute_new_address_bech32},
        types::Address,
    },
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
    pub fn from_transaction(emitted_transaction: Transaction) -> Result<Self> {
        let emitted_transaction_data = match &emitted_transaction.data {
            None => String::new(),
            Some(data) => String::from_utf8_lossy(&base64_decode(data)?).into_owned(),
        };
        let contract_address = tx_opt_deploy_new_address(&emitted_transaction);

        Ok(Self {
            emitted_transaction,
            emitted_transaction_data,
            emitted_transaction_hash: String::new(),
            contract_address: contract_address,
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

/// Returns the new contract address for a deploy transaction, otherwise returns `None`.
fn tx_opt_deploy_new_address(tx: &Transaction) -> Option<String> {
    if tx.receiver.address == Address::zero() {
        Some(compute_new_address_bech32(&tx.sender, tx.nonce).to_bech32_string())
    } else {
        None
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

#[cfg(test)]
mod tests {
    use super::*;
    use multiversx_sdk::{chain_core::std::Bech32Address, data::transaction::TransactionVersion};

    fn transaction(receiver: Bech32Address) -> Transaction {
        Transaction {
            nonce: 42,
            value: "0".to_owned(),
            receiver,
            sender: Address::from([7; 32]).to_bech32_default(),
            gas_price: 1_000_000_000,
            gas_limit: 100_000,
            data: None,
            signature: None,
            chain_id: "D".to_owned(),
            version: TransactionVersion::V2,
            options: None,
            relayer: None,
            relayer_signature: None,
        }
    }

    #[test]
    fn derives_contract_address_for_zero_recipient() {
        let transaction = transaction(Bech32Address::zero_default_hrp());
        let expected =
            compute_new_address_bech32(&transaction.sender, transaction.nonce).to_bech32_string();

        let output = TxOutputFile::from_transaction(transaction).unwrap();

        assert_eq!(output.contract_address, Some(expected));
    }

    #[test]
    fn does_not_set_contract_address_for_nonzero_recipient() {
        let output =
            TxOutputFile::from_transaction(transaction(Address::from([8; 32]).to_bech32_default()))
                .unwrap();

        assert_eq!(output.contract_address, None);
    }
}
