use anyhow::Result;
use multiversx_sdk::{
    chain_core::types::Address,
    data::{account::Account, esdt::EsdtBalance},
    gateway::{
        GetAccountEsdtRolesRequest, GetAccountEsdtTokensRequest, GetAccountRequest,
        GetAccountStorageRequest,
    },
};
use std::collections::HashMap;

use super::GatewayHttpProxy;

impl GatewayHttpProxy {
    // get_account retrieves an account info from the network (nonce, balance)
    pub async fn get_account(&self, hrp: &str, address: &Address) -> Result<Account> {
        self.http_request(GetAccountRequest::new(hrp, address))
            .await
    }

    // get_account_esdt_roles retrieves an all esdt roles of an account from the network
    pub async fn get_account_esdt_roles(
        &self,
        hrp: &str,
        address: &Address,
    ) -> Result<HashMap<String, Vec<String>>> {
        self.http_request(GetAccountEsdtRolesRequest::new(hrp, address))
            .await
    }

    // get_account_esdt_tokens retrieves an all esdt token of an account from the network
    pub async fn get_account_esdt_tokens(
        &self,
        hrp: &str,
        address: &Address,
    ) -> Result<HashMap<String, EsdtBalance>> {
        self.http_request(GetAccountEsdtTokensRequest::new(hrp, address))
            .await
    }

    // get_account_esdt_tokens retrieves an all esdt token of an account from the network
    pub async fn get_account_storage_keys(
        &self,
        hrp: &str,
        address: &Address,
    ) -> Result<HashMap<String, String>> {
        self.http_request(GetAccountStorageRequest::new(hrp, address))
            .await
    }
}
