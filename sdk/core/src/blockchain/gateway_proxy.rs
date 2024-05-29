use reqwest::Client;

#[derive(Clone, Debug)]
pub struct CommunicationProxy {
    pub(crate) proxy_url: String,
    pub(crate) client: Client,
}

impl CommunicationProxy {
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
