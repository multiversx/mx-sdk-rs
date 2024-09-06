use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub struct UserAddressesResponse {
    response: Option<Value>,
}

impl UserAddressesResponse {
    pub fn new(response: String) -> Self {
        let response = serde_json::from_str::<Value>(&response).ok();
        Self { response }
    }

    pub fn response(&self) -> Value {
        self.response.clone().unwrap_or_else(|| json!({ "error": "no response" }))
    }

    pub fn is_empty(&self) -> bool {
        if let Some(ref response) = self.response {
            if let Some(user_addresses) = response.get("userAddresses") {
                if let Some(user_addresses_str) = user_addresses.as_str() {
                    return user_addresses_str.is_empty();
                } else if let Some(user_addresses_array) = user_addresses.as_array() {
                    return user_addresses_array.is_empty();
                }
            }
        }
        true
    }
}
