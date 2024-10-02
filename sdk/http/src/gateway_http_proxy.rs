mod http_account;
mod http_block;
mod http_chain_simulator;
mod http_network;
mod http_tx;

use std::time::Duration;

use multiversx_sdk::gateway::{GatewayAsyncService, GatewayRequest};

/// Allows communication with the MultiversX gateway API.
#[derive(Clone, Debug)]
pub struct GatewayHttpProxy {
    pub(crate) proxy_uri: String,
    pub(crate) client: reqwest::Client,
}

impl GatewayHttpProxy {
    pub fn new(proxy_uri: String) -> Self {
        Self {
            proxy_uri,
            client: reqwest::Client::new(),
        }
    }

    /// Performs a request to the gateway.
    /// Can be either GET or POST, depending on the argument.
    pub async fn http_request<G>(&self, request: G) -> anyhow::Result<G::Result>
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

impl GatewayAsyncService for GatewayHttpProxy {
    fn request<G>(
        &self,
        request: G,
    ) -> impl std::future::Future<Output = anyhow::Result<G::Result>> + Send
    where
        G: multiversx_sdk::gateway::GatewayRequest,
    {
        self.http_request(request)
    }

    fn sleep(&self, millis: u64) -> impl std::future::Future<Output = ()> + Send {
        tokio::time::sleep(Duration::from_millis(millis))
    }
}
