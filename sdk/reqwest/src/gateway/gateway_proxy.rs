use multiversx_sdk::gateway::GatewayRequest;
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

    /// Performs a request to the gateway.
    /// Can be either GET or POST, depending on the argument.
    ///
    ///
    pub async fn request<G>(&self, request: G) -> anyhow::Result<G::Result>
    where
        G: GatewayRequest,
    {
        let url = format!("{}/{}", self.proxy_uri, request.get_endpoint());
        let mut request_builder;
        match request.request_type() {
            multiversx_sdk::gateway::GatewayRequestType::Get => {
                request_builder = self.client.get(url);
            },
            multiversx_sdk::gateway::GatewayRequestType::Post => {
                request_builder = self.client.post(url);
            },
        }

        if let Some(payload) = request.get_payload() {
            request_builder = request_builder.json(&payload);
        }

        let decoded = request_builder
            .send()
            .await?
            .json::<G::DecodedJson>()
            .await?;

        request.process_json(decoded)
    }
}
