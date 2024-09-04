use serde_json::{json, Value};

pub struct TimestampView {
    data: Option<Value>,
}

impl TimestampView {
    // not used anymore I guess
    pub fn new() -> Self {
        TimestampView { data: Some(json!({"value": 1})) }
    }

    pub fn from_response(response: String) -> Self {
        let data = json!({ "data": { "timestamp": response } });
        TimestampView { data: Some(data) }
    }

    pub fn data(&self) -> Value {
        self.data.clone().unwrap_or_else(|| json!({ "error": "no data" }))
    }
}
