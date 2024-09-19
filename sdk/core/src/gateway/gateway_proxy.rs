use reqwest::Client;

/// Allows communication with the MultiversX gateway API.
#[derive(Clone, Debug)]
pub struct GatewayProxy {
    pub(crate) proxy_uri: String,
    pub(crate) client: Client,
    pub chain_simulator: bool,
}

impl GatewayProxy {
    pub fn new(proxy_uri: String, chain_simulator: bool) -> Self {
        Self {
            proxy_uri,
            client: Client::new(),
            chain_simulator,
        }
    }

    pub(crate) fn get_endpoint(&self, endpoint: &str) -> String {
        format!("{}/{}", self.proxy_uri, endpoint)
    }
}
