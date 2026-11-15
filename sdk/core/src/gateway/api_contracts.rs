use crate::data::api::ApiAccountListItem;

use super::{API_ACCOUNTS_ENDPOINT, GatewayRequest, GatewayRequestType, encode_base64_query_value};

/// Sort order for API listings, by timestamp.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ApiSortOrder {
    Asc,
    Desc,
}

impl ApiSortOrder {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            ApiSortOrder::Asc => "asc",
            ApiSortOrder::Desc => "desc",
        }
    }
}

/// Retrieves the number of smart contracts from the MultiversX API.
pub struct ApiCountContractsRequest;

impl GatewayRequest for ApiCountContractsRequest {
    type Payload = ();
    type DecodedJson = usize;
    type Result = usize;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Get
    }

    fn get_endpoint(&self) -> String {
        format!("{API_ACCOUNTS_ENDPOINT}/count?isSmartContract=true")
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        Ok(decoded)
    }
}

/// Retrieves a page of smart contract addresses from the MultiversX API, sorted by timestamp.
///
/// Uses cursor pagination: pass the `search_after` of the last item of a page to get the next page.
/// Unlike offset pagination (`from`), which the API caps at 10000 results, this covers all contracts.
/// A cursor is only valid with the same sort order it was obtained with.
pub struct ApiListContractsRequest {
    pub size: usize,
    pub order: ApiSortOrder,

    /// Cursor (base64) from the last item of the previous page; `None` for the first page.
    pub search_after: Option<String>,
}

impl ApiListContractsRequest {
    pub fn new(size: usize, order: ApiSortOrder, search_after: Option<String>) -> Self {
        Self {
            size,
            order,
            search_after,
        }
    }
}

impl GatewayRequest for ApiListContractsRequest {
    type Payload = ();
    type DecodedJson = Vec<ApiAccountListItem>;
    type Result = Vec<ApiAccountListItem>;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Get
    }

    fn get_endpoint(&self) -> String {
        let mut endpoint = format!(
            "{API_ACCOUNTS_ENDPOINT}?isSmartContract=true&fields=address,searchAfter&sort=timestamp&order={}&size={}",
            self.order.as_str(),
            self.size
        );
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
