use crate::data::esdt::EsdtRolesResponse;
use anyhow::anyhow;
use multiversx_chain_core::types::Address;
use std::collections::HashMap;

use super::{GatewayRequest, GatewayRequestType, ACCOUNT_ENDPOINT};

/// Retrieves an all esdt roles of an account from the network.
pub struct GetAccountEsdtRolesRequest<'a> {
    pub address: &'a Address,
}

impl<'a> GetAccountEsdtRolesRequest<'a> {
    pub fn new(address: &'a Address) -> Self {
        Self { address }
    }
}

impl<'a> GatewayRequest for GetAccountEsdtRolesRequest<'a> {
    type Payload = ();
    type DecodedJson = EsdtRolesResponse;
    type Result = HashMap<String, Vec<String>>;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Get
    }

    fn get_endpoint(&self) -> String {
        format!(
            "{ACCOUNT_ENDPOINT}/{}/esdts/roles",
            crate::bech32::encode(self.address)
        )
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        match decoded.data {
            None => Err(anyhow!("{}", decoded.error)),
            Some(b) => Ok(b.roles),
        }
    }
}
