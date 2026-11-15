use crate::data::api::ApiEvent;

use super::{API_EVENTS_ENDPOINT, GatewayRequest, GatewayRequestType, encode_base64_query_value};

/// Retrieves a page of events with a given identifier (e.g. `SCDeploy`, `SCUpgrade`) from the
/// MultiversX API, across the whole network, newest first.
///
/// Uses cursor pagination: pass the `search_after` of the last event of a page to get the next page.
pub struct ApiGetEventsRequest<'a> {
    pub identifier: &'a str,
    pub size: usize,

    /// Cursor (base64) from the last event of the previous page; `None` for the first page.
    pub search_after: Option<String>,

    /// Only events after this timestamp (seconds), e.g. to fetch what is new since an earlier run.
    pub after: Option<u64>,
}

impl<'a> ApiGetEventsRequest<'a> {
    pub fn new(identifier: &'a str, size: usize, search_after: Option<String>) -> Self {
        Self {
            identifier,
            size,
            search_after,
            after: None,
        }
    }

    /// Only events after this timestamp (seconds).
    pub fn after(mut self, timestamp: u64) -> Self {
        self.after = Some(timestamp);
        self
    }
}

impl GatewayRequest for ApiGetEventsRequest<'_> {
    type Payload = ();
    type DecodedJson = Vec<ApiEvent>;
    type Result = Vec<ApiEvent>;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Get
    }

    fn get_endpoint(&self) -> String {
        let mut endpoint = format!(
            "{API_EVENTS_ENDPOINT}?identifier={}&size={}",
            self.identifier, self.size
        );
        if let Some(after) = self.after {
            endpoint += &format!("&after={after}");
        }
        if let Some(search_after) = &self.search_after {
            endpoint += "&searchAfter=";
            endpoint += &encode_base64_query_value(search_after);
        }
        endpoint
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        Ok(decoded)
    }
}
