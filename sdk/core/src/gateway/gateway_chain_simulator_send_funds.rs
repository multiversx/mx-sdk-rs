use std::collections::HashMap;

use anyhow::anyhow;

use super::{
    gateway_chain_simulator_blocks::GenerateBlocksResponse, GatewayRequest, GatewayRequestType,
    SEND_USER_FUNDS_ENDPOINT,
};

/// Generates blocks using the chain simulator API.
pub struct ChainSimulatorSendFundsRequest {
    payload: HashMap<&'static str, String>,
}

impl ChainSimulatorSendFundsRequest {
    /// TODO: convert to argument to Address
    pub fn to_address(receiver: String) -> Self {
        let mut payload = HashMap::new();
        payload.insert("receiver", receiver);
        Self { payload }
    }
}

impl GatewayRequest for ChainSimulatorSendFundsRequest {
    type Payload = HashMap<&'static str, String>;
    type DecodedJson = GenerateBlocksResponse;
    type Result = String;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Get
    }

    fn get_endpoint(&self) -> String {
        SEND_USER_FUNDS_ENDPOINT.to_owned()
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
