use crate::data::network_config::{NetworkConfig, NetworkConfigResponse};
use anyhow::anyhow;

use super::{GatewayRequest, GatewayRequestType, NETWORK_CONFIG_ENDPOINT};

/// Retrieves the network configuration from the proxy.
pub struct NetworkConfigRequest;

impl GatewayRequest for NetworkConfigRequest {
    type Payload = ();
    type DecodedJson = NetworkConfigResponse;
    type Result = NetworkConfig;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Get
    }

    fn get_endpoint(&self) -> String {
        NETWORK_CONFIG_ENDPOINT.to_owned()
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        match decoded.data {
            None => Err(anyhow!("{}", decoded.error)),
            Some(b) => Ok(b.config),
        }
    }
}
