//! Retrieves a contract's metadata, code and upgrade history from the MultiversX API.

use multiversx_sdk::{
    chain_core::std::Bech32Address,
    gateway::{ApiGetAccountRequest, ApiGetUpgradesRequest, MAINNET_API},
};
use multiversx_sdk_http::GatewayHttpProxy;

#[tokio::main]
async fn main() {
    let address = Bech32Address::from_bech32_string(
        "erd1qqqqqqqqqqqqqpgqq66xk9gfr4esuhem3jru86wg5hvp33a62jps2fy57p".to_owned(),
    );

    // The `Api*` requests need a proxy pointing to the API, not to the gateway.
    let api = GatewayHttpProxy::new(MAINNET_API.to_string());

    // The code can be large, leave it out if only the metadata is needed.
    let account = api
        .http_request(ApiGetAccountRequest::new(&address).without_code())
        .await
        .unwrap();
    println!("account: {account:#?}");

    let account_with_code = api
        .http_request(ApiGetAccountRequest::new(&address))
        .await
        .unwrap();
    if let Some(code) = account_with_code.code {
        // hex-encoded, 2 characters per byte
        println!("code size: {} bytes", code.len() / 2);
    }

    let upgrades = api
        .http_request(ApiGetUpgradesRequest::new(&address))
        .await
        .unwrap();
    println!("{} upgrades, newest first:", upgrades.len());
    for upgrade in upgrades {
        println!("  {} at {}", upgrade.tx_hash, upgrade.timestamp);
    }
}
