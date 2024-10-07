use crate::interactor::ContractInteract;
use multiversx_sc_snippets_dapp::sdk::{
    core::gateway::GatewayAsyncService, core::gateway::NetworkStatusRequest,
    data::network_status::NetworkStatus,
};

pub async fn get_network_status() -> Result<NetworkStatus, String> {
    let contract_interact = ContractInteract::new().await;
    let shard = 1u32;

    let response = contract_interact
        .interactor
        .proxy
        .request(NetworkStatusRequest::new(shard))
        .await;

    match response {
        Ok(value) => Ok(value),
        Err(err) => Err(err.to_string()),
    }
}
