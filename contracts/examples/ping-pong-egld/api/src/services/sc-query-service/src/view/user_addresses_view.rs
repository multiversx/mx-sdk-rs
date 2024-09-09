use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub struct UserAddressesResponse {
    response: Option<Value>,
}

impl UserAddressesResponse {
    pub fn new(response: Vec<String>) -> Self {
        let response_json = json!({
            "response": response
        });
        Self { response: Some(response_json) }
    }

    pub fn response(&self) -> Value {
        self.response.clone().unwrap_or_else(|| json!({ "error": "no response" }))
    }
}
