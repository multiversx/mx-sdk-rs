use crate::data::api::{ApiTransaction, ApiTransactionListItem};

use super::{
    API_TRANSACTIONS_ENDPOINT, ApiSortOrder, GatewayRequest, GatewayRequestType,
    encode_base64_query_value,
};

/// Retrieves a transaction from the MultiversX API, with status, timestamp and data.
pub struct ApiGetTxRequest<'a> {
    pub hash: &'a str,
}

impl<'a> ApiGetTxRequest<'a> {
    pub fn new(hash: &'a str) -> Self {
        Self { hash }
    }
}

impl GatewayRequest for ApiGetTxRequest<'_> {
    type Payload = ();
    type DecodedJson = ApiTransaction;
    type Result = ApiTransaction;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Get
    }

    fn get_endpoint(&self) -> String {
        format!(
            "{API_TRANSACTIONS_ENDPOINT}/{}?fields=txHash,status,timestamp,data",
            self.hash
        )
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        Ok(decoded)
    }
}

/// Retrieves a page of transactions from the MultiversX API, filtered, sorted by timestamp.
///
/// Uses cursor pagination: pass the `search_after` of the last item of a page to get the next page.
/// A cursor is only valid with the same filters and sort order it was obtained with.
///
/// Example, the failed direct deploys (sent to the zero address), oldest first:
/// `ApiListTransactionsRequest::new(100, ApiSortOrder::Asc, None).receiver(ZERO).status("fail")`.
#[derive(Clone, Debug)]
pub struct ApiListTransactionsRequest {
    pub size: usize,
    pub order: ApiSortOrder,

    /// Cursor (base64) from the last item of the previous page; `None` for the first page.
    pub search_after: Option<String>,

    pub receiver: Option<String>,
    pub function: Option<String>,
    pub status: Option<String>,

    /// Also retrieve the transactions' data field.
    pub with_data: bool,

    /// Also retrieve the transactions' events. The API then allows at most 50 items per page.
    pub with_logs: bool,
}

impl ApiListTransactionsRequest {
    pub fn new(size: usize, order: ApiSortOrder, search_after: Option<String>) -> Self {
        Self {
            size,
            order,
            search_after,
            receiver: None,
            function: None,
            status: None,
            with_data: false,
            with_logs: false,
        }
    }

    /// Only transactions sent to this address (bech32).
    pub fn receiver(mut self, receiver: impl Into<String>) -> Self {
        self.receiver = Some(receiver.into());
        self
    }

    /// Only transactions calling this function, e.g. `upgradeContract`.
    pub fn function(mut self, function: impl Into<String>) -> Self {
        self.function = Some(function.into());
        self
    }

    /// Also retrieve the transactions' data field (e.g. the code, for deploys and upgrades).
    pub fn with_data(mut self) -> Self {
        self.with_data = true;
        self
    }

    /// Also retrieve the transactions' events, e.g. the `signalError` event of failed transactions,
    /// which carries the failure message. The API then allows at most 50 items per page.
    pub fn with_logs(mut self) -> Self {
        self.with_logs = true;
        self
    }

    /// Only transactions with this status: `success`, `fail`, `invalid` or `pending`.
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }
}

impl GatewayRequest for ApiListTransactionsRequest {
    type Payload = ();
    type DecodedJson = Vec<ApiTransactionListItem>;
    type Result = Vec<ApiTransactionListItem>;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Get
    }

    fn get_endpoint(&self) -> String {
        let mut fields =
            "txHash,sender,receiver,nonce,timestamp,status,function,searchAfter".to_owned();
        if self.with_data {
            fields += ",data";
        }
        if self.with_logs {
            fields += ",logs&withLogs=true";
        }
        let mut endpoint = format!(
            "{API_TRANSACTIONS_ENDPOINT}?fields={fields}&order={}&size={}",
            self.order.as_str(),
            self.size
        );
        for (name, value) in [
            ("receiver", &self.receiver),
            ("function", &self.function),
            ("status", &self.status),
        ] {
            if let Some(value) = value {
                endpoint += &format!("&{name}={value}");
            }
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
