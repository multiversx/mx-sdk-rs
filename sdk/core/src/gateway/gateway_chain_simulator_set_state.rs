use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub keys: HashMap<String, String>,
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
            keys: HashMap::new(),
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
    pub fn with_keys(mut self, keys: HashMap<String, String>) -> Self {
        self.keys = keys;

        self
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
