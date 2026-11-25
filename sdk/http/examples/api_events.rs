//! Retrieves the contract deploy events of the last 7 days from the MultiversX API.

use std::time::{SystemTime, UNIX_EPOCH};

use multiversx_sdk::gateway::{ApiGetEventsRequest, MAINNET_API};
use multiversx_sdk_http::GatewayHttpProxy;

const PAGE_SIZE: usize = 5;
const NUM_PAGES: usize = 2;
const ONE_WEEK_SECONDS: u64 = 7 * 24 * 60 * 60;

#[tokio::main]
async fn main() {
    let api = GatewayHttpProxy::new(MAINNET_API.to_string());

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let one_week_ago = now.as_secs() - ONE_WEEK_SECONDS;

    // Events come newest first. Pass the cursor of the last one to get the next page.
    let mut search_after = None;
    for page in 1..=NUM_PAGES {
        let request =
            ApiGetEventsRequest::new("SCDeploy", PAGE_SIZE, search_after).after(one_week_ago);
        let events = api.http_request(request).await.unwrap();

        println!("page {page}:");
        for event in &events {
            println!(
                "  {} deployed at {}, tx {}",
                event.address, event.timestamp, event.tx_hash
            );
        }

        search_after = events.last().and_then(|event| event.search_after.clone());
        if search_after.is_none() {
            break;
        }
    }
}
