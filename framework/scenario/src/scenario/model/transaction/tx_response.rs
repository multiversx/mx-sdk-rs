use multiversx_chain_vm::tx_mock::TxResult;
use multiversx_sc::types::Address;

use super::{Log, TxExpect, TxResponseStatus};

#[derive(Debug, Default, Clone)]
/// The response of a transaction.
pub struct TxResponse {
    /// The output of the transaction.
    pub out: Vec<Vec<u8>>,
    /// The address of the newly deployed smart contract.
    pub new_deployed_address: Option<Address>,
    /// The identifier of the newly issued token.
    pub new_issued_token_identifier: Option<String>,
    /// The status of the transaction.
    pub tx_error: TxResponseStatus,
    /// The logs of the transaction.
    pub logs: Vec<Log>,
    /// The gas used by the transaction.
    pub gas: u64,
    /// The refund of the transaction.
    pub refund: u64,
}

impl TxResponse {
    /// Creates a [`TxResponse`] from a [`TxResult`].
    pub fn from_tx_result(tx_result: TxResult) -> Self {
        TxResponse {
            out: tx_result.result_values,
            tx_error: TxResponseStatus {
                status: tx_result.result_status,
                message: tx_result.result_message,
            },
            ..Default::default()
        }
    }

    /// Creates a [`TxResponse`] from raw results.
    pub fn from_raw_results(raw_results: Vec<Vec<u8>>) -> Self {
        TxResponse {
            out: raw_results,
            ..Default::default()
        }
    }

    /// Creates a scenario "expect" field based on the real response.
    ///
    /// Useful for creating traces that also check the results come out always the same.
    pub fn to_expect(&self) -> TxExpect {
        if self.tx_error.is_success() {
            let mut tx_expect = TxExpect::ok();
            if self.out.is_empty() {
                tx_expect = tx_expect.no_result();
            } else {
                for raw_result in &self.out {
                    let result_hex_string = format!("0x{}", hex::encode(raw_result));
                    tx_expect = tx_expect.result(result_hex_string.as_str());
                }
            }
            tx_expect
        } else {
            TxExpect::err(
                self.tx_error.status,
                format!("str:{}", self.tx_error.message),
            )
        }
    }

    /// Checks if the transaction was successful.
    pub fn is_success(&self) -> bool {
        self.tx_error.is_success()
    }
}
