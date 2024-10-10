use anyhow::Result;
use multiversx_sdk::{
    data::{network_config::NetworkConfig, network_economics::NetworkEconomics},
    gateway::{NetworkConfigRequest, NetworkEconimicsRequest},
};

use super::GatewayHttpProxy;

impl GatewayHttpProxy {
    // get_network_config retrieves the network configuration from the proxy
    pub async fn get_network_config(&self) -> Result<NetworkConfig> {
        self.http_request(NetworkConfigRequest).await
    }

    // get_network_economics retrieves the network economics from the proxy
    pub async fn get_network_economics(&self) -> Result<NetworkEconomics> {
        self.http_request(NetworkEconimicsRequest).await
    }
}
