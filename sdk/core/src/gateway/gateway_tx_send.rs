use crate::data::transaction::{SendTransactionResponse, Transaction};
use anyhow::anyhow;

use super::{GatewayRequest, GatewayRequestType, SEND_TRANSACTION_ENDPOINT};

/// Sends a single transaction.
pub struct SendTxRequest<'a>(pub &'a Transaction);

impl<'a> GatewayRequest for SendTxRequest<'a> {
    type Payload = Transaction;
    type DecodedJson = SendTransactionResponse;
    type Result = String;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Post
    }

    fn get_payload(&self) -> Option<&Self::Payload> {
        Some(self.0)
    }

    fn get_endpoint(&self) -> String {
        SEND_TRANSACTION_ENDPOINT.to_owned()
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        match decoded.data {
            None => Err(anyhow!("{}", decoded.error)),
            Some(b) => Ok(b.tx_hash),
        }
    }
}
