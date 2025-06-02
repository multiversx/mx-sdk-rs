use crate::data::account_storage::AccountStorageResponse;
use anyhow::anyhow;
use multiversx_chain_core::types::Address;
use std::collections::HashMap;

use super::{GatewayRequest, GatewayRequestType, ACCOUNT_ENDPOINT, KEYS_ENDPOINT};

/// Retrieves an account storage from the network.
pub struct GetAccountStorageRequest<'a> {
    pub hrp: &'a str,
    pub address: &'a Address,
}

impl<'a> GetAccountStorageRequest<'a> {
    pub fn new(hrp: &'a str, address: &'a Address) -> Self {
        Self { hrp, address }
    }
}

impl GatewayRequest for GetAccountStorageRequest<'_> {
    type Payload = ();
    type DecodedJson = AccountStorageResponse;
    type Result = HashMap<String, String>;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Get
    }

    fn get_endpoint(&self) -> String {
        format!(
            "{ACCOUNT_ENDPOINT}/{}/{KEYS_ENDPOINT}",
            crate::bech32::encode(self.hrp, self.address)
        )
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        match decoded.data {
            None => Err(anyhow!("{}", decoded.error)),
            Some(b) => Ok(b.pairs),
        }
    }
}
