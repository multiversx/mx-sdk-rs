use anyhow::{anyhow, Result};
use gloo_net::http::Request;
use multiversx_sdk::data::{
    network_config::{NetworkConfig, NetworkConfigResponse},
    network_economics::{NetworkEconomics, NetworkEconomicsResponse},
    network_status::{NetworkStatus, NetworkStatusResponse},
};

use super::GatewayDappProxy;

const NETWORK_CONFIG_ENDPOINT: &str = "network/config";
const NETWORK_ECONOMICS_ENDPOINT: &str = "network/economics";
const NETWORK_STATUS_ENDPOINT: &str = "network/status";

impl GatewayDappProxy {
    // get_network_config retrieves the network configuration from the proxy
    pub async fn get_network_config(&self) -> Result<NetworkConfig> {
        let endpoint = self.get_endpoint(NETWORK_CONFIG_ENDPOINT);
        let resp = Request::get(&endpoint)
            .send()
            .await?
            .json::<NetworkConfigResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.config),
        }
    }

    // get_network_economics retrieves the network economics from the proxy
    pub async fn get_network_economics(&self) -> Result<NetworkEconomics> {
        let endpoint = self.get_endpoint(NETWORK_ECONOMICS_ENDPOINT);
        let resp = Request::get(&endpoint)
            .send()
            .await?
            .json::<NetworkEconomicsResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.metrics),
        }
    }

    // get_network_status retrieves the network status from the proxy
    pub async fn get_network_status(&self, shard: u8) -> Result<NetworkStatus> {
        let endpoint = self.get_endpoint(NETWORK_STATUS_ENDPOINT);
        let full_endpoint = format!("{}/{}", endpoint, shard);
        let resp = Request::get(&full_endpoint)
            .send()
            .await?
            .json::<NetworkStatusResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.status),
        }
    }
}
