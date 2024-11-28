use crate::data::transaction::{TransactionInfo, TransactionOnNetwork};
use anyhow::anyhow;

use super::{
    GatewayRequest, GatewayRequestType, GET_TRANSACTION_INFO_ENDPOINT, WITH_RESULTS_QUERY_PARAM,
};

/// Retrieves transaction data from the network.
pub struct GetTxInfo<'a> {
    pub hash: &'a str,
    pub with_results: bool,
}

impl<'a> GetTxInfo<'a> {
    pub fn new(hash: &'a str) -> Self {
        Self {
            hash,
            with_results: true,
        }
    }

    pub fn with_results(self) -> Self {
        Self {
            hash: self.hash,
            with_results: true,
        }
    }
}

impl GatewayRequest for GetTxInfo<'_> {
    type Payload = ();
    type DecodedJson = TransactionInfo;
    type Result = TransactionOnNetwork;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Get
    }

    fn get_endpoint(&self) -> String {
        let mut endpoint = format!("{GET_TRANSACTION_INFO_ENDPOINT}/{}", self.hash);

        if self.with_results {
            endpoint += WITH_RESULTS_QUERY_PARAM;
        }

        endpoint
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        match decoded.data {
            None => Err(anyhow!("{}", decoded.error)),
            Some(b) => Ok(b.transaction),
        }
    }
}
