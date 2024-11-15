use crate::data::transaction::TransactionProcessStatus;
use anyhow::anyhow;

use super::{GatewayRequest, GatewayRequestType};

/// Retrieves a transaction's status from the network using process-status API.
pub struct GetTxProcessStatus<'a> {
    pub hash: &'a str,
}

impl<'a> GetTxProcessStatus<'a> {
    pub fn new(hash: &'a str) -> Self {
        Self { hash }
    }
}

impl<'a> GatewayRequest for GetTxProcessStatus<'a> {
    type Payload = ();
    type DecodedJson = TransactionProcessStatus;
    type Result = (String, String);

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Get
    }

    fn get_endpoint(&self) -> String {
        format!("transaction/{}/process-status", self.hash)
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        match decoded.data {
            None => Err(anyhow!("{}", decoded.error)),
            Some(b) => Ok((b.status, b.reason)),
        }
    }
}
