/// Allows communication with the MultiversX gateway API.
#[derive(Clone, Debug)]
pub struct GatewayProxy {
    pub(crate) proxy_url: String,
}

impl GatewayProxy {
    pub fn new(proxy_url: String) -> Self {
        Self { proxy_url }
    }

    pub(crate) fn get_endpoint(&self, endpoint: &str) -> String {
        format!("{}/{}", self.proxy_url, endpoint)
    }
}
