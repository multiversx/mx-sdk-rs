use multiversx_chain_core::std::Bech32Address;
use serde::{Deserialize, Serialize};

use crate::data::transaction::{TransactionOptions, TransactionVersion};

// ArgCreateTransaction will hold the transaction fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArgCreateTransaction {
    pub nonce: u64,
    pub value: String,
    pub rcv_addr: Bech32Address,
    pub snd_addr: Bech32Address,
    pub gas_price: u64,
    pub gas_limit: u64,
    pub data: Option<String>,
    pub signature: String,
    pub chain_id: String,
    pub version: TransactionVersion,
    pub options: Option<TransactionOptions>,
    pub available_balance: String,
}
