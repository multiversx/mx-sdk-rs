use crate::data::{
    account::{Account, AccountResponse},
    sdk_address::SdkAddress,
};
use anyhow::anyhow;

use super::ACCOUNT_ENDPOINT;
use super::{GatewayRequest, GatewayRequestType};

/// Retrieves an account info from the network (nonce, balance).
pub struct GetAccountRequest<'a> {
    pub address: &'a SdkAddress,
}

impl<'a> GetAccountRequest<'a> {
    pub fn new(address: &'a SdkAddress) -> Self {
        Self { address }
    }
}

impl<'a> GatewayRequest for GetAccountRequest<'a> {
    type Payload = ();
    type DecodedJson = AccountResponse;
    type Result = Account;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Get
    }

    fn get_endpoint(&self) -> String {
        format!("{ACCOUNT_ENDPOINT}{}", self.address)
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        match decoded.data {
            None => Err(anyhow!("{}", decoded.error)),
            Some(b) => Ok(b.account),
        }
    }
}
