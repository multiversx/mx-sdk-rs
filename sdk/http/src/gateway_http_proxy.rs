mod http_account;
mod http_block;
mod http_chain_simulator;
mod http_network;
mod http_tx;

use multiversx_sdk::gateway::{GatewayAsyncService, GatewayRequest, GatewayRequestType};
use reqwest::Method;
use std::time::Duration;

/// Allows communication with the MultiversX gateway API.
#[derive(Clone, Debug)]
pub struct GatewayHttpProxy {
    pub(crate) proxy_uri: String,
    pub(crate) client: reqwest::Client,
}

/// Converts from common sdk type to reqwest type.
fn reqwest_method(request_type: GatewayRequestType) -> Method {
    match request_type {
        GatewayRequestType::Get => Method::GET,
        GatewayRequestType::Post => Method::POST,
    }
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
        let method = request.request_type();
        log::info!("{method} request: {url}");

        let mut request_builder = self.client.request(reqwest_method(method), url);
        if let Some(payload) = request.get_payload() {
            log::trace!("{method} payload: {}", serde_json::to_string(payload)?);
            request_builder = request_builder.json(&payload);
        }

        let http_request = request_builder.build()?;
        let http_response = self.client.execute(http_request).await?;
        let http_response_text = http_response.text().await?;
        log::trace!("{method} response: {http_response_text}");

        let decoded = serde_json::from_str::<G::DecodedJson>(&http_response_text)?;

        request.process_json(decoded)
    }
}

impl GatewayAsyncService for GatewayHttpProxy {
    type Instant = std::time::Instant;

    fn from_uri(uri: &str) -> Self {
        Self::new(uri.to_owned())
    }

    fn request<G>(&self, request: G) -> impl std::future::Future<Output = anyhow::Result<G::Result>>
    where
        G: multiversx_sdk::gateway::GatewayRequest,
    {
        self.http_request(request)
    }

    fn sleep(&self, millis: u64) -> impl std::future::Future<Output = ()> {
        tokio::time::sleep(Duration::from_millis(millis))
    }

    fn now(&self) -> Self::Instant {
        std::time::Instant::now()
    }

    fn elapsed_seconds(&self, instant: &Self::Instant) -> f32 {
        instant.elapsed().as_secs_f32()
    }
}
