use reqwest::Client;

pub const CHAIN_SIMULATOR_GATEWAY: &str = "http://localhost:8085";

/// Allows communication with the MultiversX gateway API.
#[derive(Clone, Debug)]
pub struct GatewayProxy {
    pub(crate) proxy_url: String,
    pub(crate) client: Client,
    pub chain_simulator: bool,
}

impl GatewayProxy {
    pub fn new(proxy_url: String) -> Self {
        let chain_simulator = proxy_url == CHAIN_SIMULATOR_GATEWAY;

        Self {
            proxy_url,
            client: Client::new(),
            chain_simulator,
        }
    }

    pub(crate) fn get_endpoint(&self, endpoint: &str) -> String {
        format!("{}/{}", self.proxy_url, endpoint)
    }
}
