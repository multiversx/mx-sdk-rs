use crate::data::transaction::{SimulateGasTransactionResponse, Transaction};
use anyhow::anyhow;

use super::{GatewayRequest, GatewayRequestType, TRANSACTION_COST_ENDPOINT};

/// Sends a single transaction.
pub struct SimulateTxRequest<'a>(pub &'a Transaction);

/// Returned by the `transaction/cost` endpoint when the simulated transaction runs out of gas.
const NOT_ENOUGH_GAS_MESSAGE: &str = "not enough gas";

impl GatewayRequest for SimulateTxRequest<'_> {
    type Payload = Transaction;
    type DecodedJson = SimulateGasTransactionResponse;
    type Result = u64;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Post
    }

    fn get_payload(&self) -> Option<&Self::Payload> {
        Some(self.0)
    }

    fn get_endpoint(&self) -> String {
        TRANSACTION_COST_ENDPOINT.to_owned()
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        match decoded.data {
            None => Err(anyhow!("{}", decoded.error)),
            // the endpoint reports 0 gas units when running out of gas,
            // `u64::MAX` signals that the transaction needs more gas than allowed
            Some(b) if b.return_message.contains(NOT_ENOUGH_GAS_MESSAGE) => Ok(u64::MAX),
            Some(b) => Ok(b.tx_gas_units),
        }
    }
}
