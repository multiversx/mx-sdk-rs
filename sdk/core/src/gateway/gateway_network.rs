use crate::data::{
    network_config::{NetworkConfig, NetworkConfigResponse},
    network_economics::{NetworkEconomics, NetworkEconomicsResponse},
};
use anyhow::{anyhow, Result};

use super::GatewayProxy;

const NETWORK_CONFIG_ENDPOINT: &str = "network/config";
const NETWORK_ECONOMICS_ENDPOINT: &str = "network/economics";

impl GatewayProxy {
    // get_network_config retrieves the network configuration from the proxy
    pub async fn get_network_config(&self) -> Result<NetworkConfig> {
        let endpoint = self.get_endpoint(NETWORK_CONFIG_ENDPOINT);
        let resp = self
            .client
            .get(endpoint)
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
        let resp = self
            .client
            .get(endpoint)
            .send()
            .await?
            .json::<NetworkEconomicsResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.metrics),
        }
    }
}
