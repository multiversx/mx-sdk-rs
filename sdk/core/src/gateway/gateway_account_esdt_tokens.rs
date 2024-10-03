use crate::data::{
    esdt::{EsdtBalance, EsdtBalanceResponse},
    sdk_address::SdkAddress,
};
use anyhow::anyhow;
use std::collections::HashMap;

use super::{GatewayRequest, GatewayRequestType, ACCOUNT_ENDPOINT};

/// Retrieves an all esdt tokens of an account from the network.
pub struct GetAccountEsdtTokensRequest<'a> {
    pub address: &'a SdkAddress,
}

impl<'a> GetAccountEsdtTokensRequest<'a> {
    pub fn new(address: &'a SdkAddress) -> Self {
        Self { address }
    }
}

impl<'a> GatewayRequest for GetAccountEsdtTokensRequest<'a> {
    type Payload = ();
    type DecodedJson = EsdtBalanceResponse;
    type Result = HashMap<String, EsdtBalance>;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Get
    }

    fn get_endpoint(&self) -> String {
        format!("{ACCOUNT_ENDPOINT}{}/esdt", self.address)
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        match decoded.data {
            None => Err(anyhow!("{}", decoded.error)),
            Some(b) => Ok(b.esdts),
        }
    }
}
