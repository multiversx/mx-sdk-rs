use std::collections::HashMap;

use anyhow::anyhow;

use crate::{gateway::ADD_KEYS, utils::base64_encode};

use super::{
    GatewayRequest, GatewayRequestType, gateway_chain_simulator_blocks::GenerateBlocksResponse,
};

/// Allows to add new validator private keys in the multi key handler.
pub struct ChainSimulatorAddKeysRequest {
    payload: HashMap<&'static str, Vec<String>>,
}

impl ChainSimulatorAddKeysRequest {
    pub fn with_keys(keys: Vec<Vec<u8>>) -> Self {
        let mut payload = HashMap::new();
        let keys_str_vec: Vec<String> = keys
            .into_iter()
            .map(|key| base64_encode(key).to_string())
            .collect();
        payload.insert("privateKeysBase64", keys_str_vec);
        Self { payload }
    }
}

impl GatewayRequest for ChainSimulatorAddKeysRequest {
    type Payload = HashMap<&'static str, Vec<String>>;
    type DecodedJson = GenerateBlocksResponse;
    type Result = String;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Post
    }

    fn get_endpoint(&self) -> String {
        ADD_KEYS.to_owned()
    }

    fn get_payload(&self) -> Option<&Self::Payload> {
        Some(&self.payload)
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        match decoded.code.as_str() {
            "successful" => Ok(decoded.code),
            _ => Err(anyhow!("{}", decoded.error)),
        }
    }
}
