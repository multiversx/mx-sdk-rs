use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter},
    path::Path,
};

use crate::data::account::Account;

use super::{GatewayRequest, GatewayRequestType, SET_STATE_ENDPOINT};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetStateResponse {
    pub data: serde_json::Value,
    pub error: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetStateAccount {
    pub address: String,
    pub nonce: u64,
    pub balance: String,
    pub pairs: HashMap<String, String>,
    pub code: String,
    #[serde(default)]
    pub code_hash: String,
    #[serde(default)]
    pub root_hash: String,
    #[serde(default)]
    pub code_metadata: String,
    #[serde(default)]
    pub owner_address: String,
    #[serde(default)]
    pub developer_reward: String,
}

impl From<Account> for SetStateAccount {
    fn from(value: Account) -> Self {
        Self {
            address: value.address.to_bech32_string().unwrap_or_default(),
            nonce: value.nonce,
            balance: value.balance.to_string(),
            pairs: HashMap::new(),
            code: value.code,
            code_hash: value.code_hash.unwrap_or_default(),
            root_hash: value.root_hash.unwrap_or_default(),
            code_metadata: value.code_metadata.unwrap_or_default(),
            owner_address: value.owner_address.unwrap_or_default(),
            developer_reward: value.developer_reward.unwrap_or_default(),
        }
    }
}

impl SetStateAccount {
    /// Specify the storage key-value pairs to set to the target account.
    pub fn with_storage(mut self, pairs: HashMap<String, String>) -> Self {
        self.pairs = pairs;
        self
    }

    /// Creates a SetStateAccount from an address
    pub fn from_address(address: String) -> Self {
        Self {
            address,
            ..Default::default()
        }
    }

    /// Kept for backwards compatibility.
    #[deprecated(since = "0.56.0", note = "Use `with_storage` instead.")]
    pub fn with_keys(self, keys: HashMap<String, String>) -> Self {
        self.with_storage(keys)
    }

    pub fn add_to_state_file(self, path: &Path) {
        let mut accounts = if path.exists() {
            let file = File::open(path)
                .unwrap_or_else(|_| panic!("Failed to open state file at path {path:#?}"));

            let reader = BufReader::new(file);

            serde_json::from_reader::<_, Vec<SetStateAccount>>(reader).unwrap_or_default()
        } else {
            Vec::new()
        };

        if let Some(existing_account) = accounts
            .iter_mut()
            .find(|account| account.address == self.address)
        {
            *existing_account = self;
        } else {
            accounts.push(self);
        }

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .unwrap_or_else(|_| panic!("Failed to open or create state file at path {path:#?}"));

        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &accounts).unwrap_or_else(|_| {
            panic!("Failed to write updated state accounts to file at path {path:#?}")
        });
    }
}

/// Sets state for a list of accounts using the chain simulator API.
pub struct ChainSimulatorSetStateRequest {
    pub accounts: Vec<SetStateAccount>,
}

impl ChainSimulatorSetStateRequest {
    pub fn for_accounts(accounts: Vec<SetStateAccount>) -> Self {
        Self { accounts }
    }
}

impl GatewayRequest for ChainSimulatorSetStateRequest {
    type Payload = Vec<SetStateAccount>;
    type DecodedJson = SetStateResponse;
    type Result = String;

    fn get_payload(&self) -> Option<&Self::Payload> {
        Some(&self.accounts)
    }

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Post
    }

    fn get_endpoint(&self) -> String {
        SET_STATE_ENDPOINT.to_owned()
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        match decoded.code.as_str() {
            "successful" => Ok(decoded.code),
            _ => Err(anyhow!("{}", decoded.error)),
        }
    }
}
