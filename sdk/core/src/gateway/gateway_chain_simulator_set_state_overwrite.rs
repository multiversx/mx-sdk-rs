use anyhow::anyhow;

use super::{
    gateway_chain_simulator_set_state::SetStateResponse, GatewayRequest, GatewayRequestType,
    SetStateAccount, SET_STATE_OVERWRITE_ENDPOINT,
};

/// Sets state for a list of accounts using the chain simulator API.
/// Overwrites previous state.
pub struct ChainSimulatorSetStateOverwriteRequest {
    pub accounts: Vec<SetStateAccount>,
}

impl ChainSimulatorSetStateOverwriteRequest {
    pub fn for_accounts(accounts: Vec<SetStateAccount>) -> Self {
        Self { accounts }
    }
}

impl GatewayRequest for ChainSimulatorSetStateOverwriteRequest {
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
        SET_STATE_OVERWRITE_ENDPOINT.to_owned()
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        match decoded.code.as_str() {
            "successful" => Ok(decoded.code),
            _ => Err(anyhow!("{}", decoded.error)),
        }
    }
}
