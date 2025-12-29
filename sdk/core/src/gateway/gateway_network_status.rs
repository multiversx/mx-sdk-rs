use crate::data::network_status::{NetworkStatus, NetworkStatusResponse};
use anyhow::anyhow;

use super::{GET_NETWORK_STATUS_ENDPOINT, GatewayRequest, GatewayRequestType, METACHAIN_SHARD_ID};

/// Retrieves the network status from the proxy.
pub struct NetworkStatusRequest {
    shard: u32,
}

impl NetworkStatusRequest {
    pub fn new(shard: u32) -> Self {
        NetworkStatusRequest { shard }
    }
}

impl Default for NetworkStatusRequest {
    fn default() -> Self {
        Self {
            shard: METACHAIN_SHARD_ID,
        }
    }
}

impl GatewayRequest for NetworkStatusRequest {
    type Payload = ();
    type DecodedJson = NetworkStatusResponse;
    type Result = NetworkStatus;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Get
    }

    fn get_endpoint(&self) -> String {
        format!("{GET_NETWORK_STATUS_ENDPOINT}/{}", self.shard)
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        match decoded.data {
            None => Err(anyhow!("{}", decoded.error)),
            Some(b) => Ok(b.status),
        }
    }
}
