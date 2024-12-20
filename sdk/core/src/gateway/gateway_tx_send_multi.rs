use crate::data::transaction::{SendTransactionsResponse, Transaction};
use anyhow::anyhow;
use itertools::Itertools;

use super::{GatewayRequest, GatewayRequestType, SEND_MULTIPLE_TRANSACTIONS_ENDPOINT};

/// Sends multiple transactions at once.
pub struct SendMultiTxRequest<'a>(pub &'a [Transaction]);

impl GatewayRequest for SendMultiTxRequest<'_> {
    type Payload = [Transaction];
    type DecodedJson = SendTransactionsResponse;
    type Result = Vec<String>;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Post
    }

    fn get_payload(&self) -> Option<&Self::Payload> {
        Some(self.0)
    }

    fn get_endpoint(&self) -> String {
        SEND_MULTIPLE_TRANSACTIONS_ENDPOINT.to_owned()
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        match decoded.data {
            None => Err(anyhow!("{}", decoded.error)),
            Some(b) => {
                let mut tx_hashs: Vec<String> = vec![];
                for key in b.txs_hashes.keys().sorted() {
                    tx_hashs.push(b.txs_hashes[key].clone());
                }

                Ok(tx_hashs)
            },
        }
    }
}
