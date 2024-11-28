use crate::data::transaction::TransactionStatus;
use anyhow::anyhow;

use super::{GatewayRequest, GatewayRequestType};

/// Retrieves a transaction's status from the network.
pub struct GetTxStatus<'a> {
    pub hash: &'a str,
}

impl<'a> GetTxStatus<'a> {
    pub fn new(hash: &'a str) -> Self {
        Self { hash }
    }
}

impl GatewayRequest for GetTxStatus<'_> {
    type Payload = ();
    type DecodedJson = TransactionStatus;
    type Result = String;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Get
    }

    fn get_endpoint(&self) -> String {
        format!("transaction/{}/status", self.hash)
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        match decoded.data {
            None => Err(anyhow!("{}", decoded.error)),
            Some(b) => Ok(b.status),
        }
    }
}
