use crate::data::transaction::{ResponseTxCost, Transaction, TxCostResponseData};
use anyhow::anyhow;

use super::{GatewayRequest, GatewayRequestType, COST_TRANSACTION_ENDPOINT};

/// Verifies the cost of a transaction.
///
/// Note: it is a POST request.
pub struct GetTxCost<'a>(pub &'a Transaction);

impl<'a> GatewayRequest for GetTxCost<'a> {
    type Payload = Transaction;
    type DecodedJson = ResponseTxCost;
    type Result = TxCostResponseData;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Post
    }

    fn get_payload(&self) -> Option<&Self::Payload> {
        Some(self.0)
    }

    fn get_endpoint(&self) -> String {
        COST_TRANSACTION_ENDPOINT.to_owned()
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        match decoded.data {
            None => Err(anyhow!("{}", decoded.error)),
            Some(b) => Ok(b),
        }
    }
}
