//! Counts the smart contracts and pages through the oldest ones, using the MultiversX API.

use multiversx_sdk::gateway::{
    ApiCountContractsRequest, ApiListContractsRequest, ApiSortOrder, MAINNET_API,
};
use multiversx_sdk_http::GatewayHttpProxy;

const PAGE_SIZE: usize = 5;
const NUM_PAGES: usize = 3;

#[tokio::main]
async fn main() {
    let api = GatewayHttpProxy::new(MAINNET_API.to_string());

    let count = api.http_request(ApiCountContractsRequest).await.unwrap();
    println!("smart contracts: {count}");

    // Cursor pagination: each page starts after the last item of the previous one.
    let mut search_after = None;
    for page in 1..=NUM_PAGES {
        let contracts = api
            .http_request(ApiListContractsRequest::new(
                PAGE_SIZE,
                ApiSortOrder::Asc,
                search_after,
            ))
            .await
            .unwrap();

        println!("page {page}:");
        for contract in &contracts {
            println!("  {}", contract.address);
        }

        search_after = contracts.last().and_then(|item| item.search_after.clone());
        if search_after.is_none() {
            break;
        }
    }
}
