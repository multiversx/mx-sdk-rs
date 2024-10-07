use crate::data::network_economics::{NetworkEconomics, NetworkEconomicsResponse};
use anyhow::anyhow;

use super::{GatewayRequest, GatewayRequestType, NETWORK_ECONOMICS_ENDPOINT};

/// Retrieves the network economics from the proxy.
pub struct NetworkEconimicsRequest;

impl GatewayRequest for NetworkEconimicsRequest {
    type Payload = ();
    type DecodedJson = NetworkEconomicsResponse;
    type Result = NetworkEconomics;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Get
    }

    fn get_endpoint(&self) -> String {
        NETWORK_ECONOMICS_ENDPOINT.to_owned()
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        match decoded.data {
            None => Err(anyhow!("{}", decoded.error)),
            Some(b) => Ok(b.metrics),
        }
    }
}
