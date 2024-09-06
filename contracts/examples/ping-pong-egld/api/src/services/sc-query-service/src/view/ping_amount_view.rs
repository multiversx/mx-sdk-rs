use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub struct PingAmountResponse {
    response: Option<Value>,
}

impl PingAmountResponse {
    pub fn new(response: String) -> Self {
        let response = serde_json::from_str::<Value>(&response).ok();
        Self { response }
    }

    pub fn response(&self) -> Value {
        self.response.clone().unwrap_or_else(|| json!({ "error": "no response" }))
    }
}
