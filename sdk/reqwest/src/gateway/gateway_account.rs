use anyhow::Result;
use multiversx_sdk::{
    data::{account::Account, address::Address, esdt::EsdtBalance},
    gateway::{
        GetAccountEsdtRolesRequest, GetAccountEsdtTokensRequest, GetAccountRequest,
        GetAccountStorageRequest,
    },
};
use std::collections::HashMap;

use super::GatewayProxy;

impl GatewayProxy {
    // get_account retrieves an account info from the network (nonce, balance)
    pub async fn get_account(&self, address: &Address) -> Result<Account> {
        self.request(GetAccountRequest::new(address)).await
    }

    // get_account_esdt_roles retrieves an all esdt roles of an account from the network
    pub async fn get_account_esdt_roles(
        &self,
        address: &Address,
    ) -> Result<HashMap<String, Vec<String>>> {
        self.request(GetAccountEsdtRolesRequest::new(address)).await
    }

    // get_account_esdt_tokens retrieves an all esdt token of an account from the network
    pub async fn get_account_esdt_tokens(
        &self,
        address: &Address,
    ) -> Result<HashMap<String, EsdtBalance>> {
        self.request(GetAccountEsdtTokensRequest::new(address))
            .await
    }

    // get_account_esdt_tokens retrieves an all esdt token of an account from the network
    pub async fn get_account_storage_keys(
        &self,
        address: &Address,
    ) -> Result<HashMap<String, String>> {
        self.request(GetAccountStorageRequest::new(address)).await
    }
}
