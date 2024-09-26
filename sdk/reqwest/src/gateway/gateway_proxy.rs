use reqwest::Client;

/// Allows communication with the MultiversX gateway API.
#[derive(Clone, Debug)]
pub struct GatewayProxy {
    pub(crate) proxy_url: String,
    pub(crate) client: Client,
}

impl GatewayProxy {
    pub fn new(proxy_url: String) -> Self {
        Self {
            proxy_url,
            client: Client::new(),
        }
    }

    pub(crate) fn get_endpoint(&self, endpoint: &str) -> String {
        format!("{}/{}", self.proxy_url, endpoint)
    }
}
