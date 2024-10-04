use gloo_net::http::Request;
use multiversx_sdk::gateway::{GatewayAsyncService, GatewayRequest, GatewayRequestType};

/// Allows communication with the MultiversX gateway API.
#[derive(Clone, Debug)]
pub struct GatewayDappProxy {
    pub(crate) proxy_url: String,
}

impl GatewayDappProxy {
    pub fn new(proxy_url: String) -> Self {
        Self { proxy_url }
    }

    /// Performs a request to the gateway.
    /// Can be either GET or POST, depending on the argument.
    pub async fn http_request<G>(&self, request: G) -> anyhow::Result<G::Result>
    where
        G: GatewayRequest,
    {
        let url = format!("{}/{}", self.proxy_url, request.get_endpoint());
        let request_builder = match request.request_type() {
            GatewayRequestType::Get => Request::get(&url),
            GatewayRequestType::Post => Request::post(&url),
        };

        let response = if let Some(payload) = request.get_payload() {
            request_builder.json(&payload)?.send().await?
        } else {
            request_builder.send().await?
        };

        let decoded = response.json::<G::DecodedJson>().await?;

        request.process_json(decoded)
    }
}

impl GatewayAsyncService for GatewayDappProxy {
    type Instant = f64;

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
        sleep(millis as f32 * 1000f32)
    }

    fn now(&self) -> Self::Instant {
        js_sys::Date::now()
    }

    fn elapsed_seconds(&self, instant: &Self::Instant) -> f32 {
        ((js_sys::Date::now() - instant) / 1000.0) as f32
    }
}

async fn sleep(seconds: f32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        if let Some(win) = web_sys::window() {
            let _ = win
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    &resolve,
                    (seconds * 1000.0) as i32,
                )
                .expect("Failed to set timeout");
        } else {
            panic!("No global window object available");
        }
    });
    wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
}
