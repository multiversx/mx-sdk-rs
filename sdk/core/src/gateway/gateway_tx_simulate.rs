use crate::data::transaction::{SimulateGasTransactionResponse, Transaction};
use anyhow::anyhow;
use multiversx_chain_core::types::GasResult;

use super::{GatewayRequest, GatewayRequestType, TRANSACTION_COST_ENDPOINT};

/// Sends a single transaction.
pub struct SimulateTxRequest<'a>(pub &'a Transaction);

/// Returned by the `transaction/cost` endpoint when the simulated transaction runs out of gas.
const NOT_ENOUGH_GAS_MESSAGE: &str = "not enough gas";

impl GatewayRequest for SimulateTxRequest<'_> {
    type Payload = Transaction;
    type DecodedJson = SimulateGasTransactionResponse;
    type Result = GasResult;

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
            // the endpoint reports 0 gas units when the simulated transaction fails,
            // the reason is in the return message
            Some(b) if b.return_message.is_empty() => Ok(GasResult::GasUsed(b.tx_gas_units)),
            Some(b) if b.return_message.contains(NOT_ENOUGH_GAS_MESSAGE) => Ok(GasResult::OutOfGas),
            Some(b) => Ok(GasResult::Failed(b.return_message)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::transaction::TransactionVersion;
    use multiversx_chain_core::std::Bech32Address;

    const ALICE: &str = "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th";

    fn dummy_tx() -> Transaction {
        Transaction {
            nonce: 0,
            value: "0".to_string(),
            receiver: Bech32Address::from_bech32_str(ALICE),
            sender: Bech32Address::from_bech32_str(ALICE),
            gas_price: 0,
            gas_limit: 0,
            data: None,
            signature: None,
            chain_id: "chain".to_string(),
            version: TransactionVersion::V2,
            options: None,
            relayer: None,
            relayer_signature: None,
        }
    }

    fn process(json: &str) -> GasResult {
        let tx = dummy_tx();
        let decoded: SimulateGasTransactionResponse = serde_json::from_str(json).unwrap();
        SimulateTxRequest(&tx).process_json(decoded).unwrap()
    }

    #[test]
    fn simulate_gas_used() {
        let result = process(
            r#"{"data":{"txGasUnits":1234567,"returnMessage":""},"error":"","code":"successful"}"#,
        );
        assert_eq!(result, GasResult::GasUsed(1234567));
    }

    #[test]
    fn simulate_out_of_gas() {
        let result = process(
            r#"{"data":{"txGasUnits":0,"returnMessage":"not enough gas"},"error":"","code":"successful"}"#,
        );
        assert_eq!(result, GasResult::OutOfGas);
    }

    #[test]
    fn simulate_failed() {
        let result = process(
            r#"{"data":{"txGasUnits":0,"returnMessage":"invalid function (not found)"},"error":"","code":"successful"}"#,
        );
        assert_eq!(
            result,
            GasResult::Failed("invalid function (not found)".to_string())
        );
    }

    #[test]
    fn simulate_gateway_error() {
        let tx = dummy_tx();
        let decoded: SimulateGasTransactionResponse =
            serde_json::from_str(r#"{"data":null,"error":"bad request","code":"bad_request"}"#)
                .unwrap();
        assert!(SimulateTxRequest(&tx).process_json(decoded).is_err());
    }
}
