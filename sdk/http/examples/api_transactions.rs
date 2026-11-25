//! Lists the latest failed contract deploys with their error messages, using the MultiversX API,
//! then retrieves one of them by hash.

use multiversx_sdk::{
    chain_core::std::{Bech32Address, base64_decode},
    data::transaction::ApiLogs,
    gateway::{ApiGetTxRequest, ApiListTransactionsRequest, ApiSortOrder, MAINNET_API},
};
use multiversx_sdk_http::GatewayHttpProxy;

#[tokio::main]
async fn main() {
    let api = GatewayHttpProxy::new(MAINNET_API.to_string());

    // Direct deploys are sent to the zero address.
    let request = ApiListTransactionsRequest::new(5, ApiSortOrder::Desc, None)
        .receiver(Bech32Address::zero_default_hrp().bech32)
        .status("fail")
        .with_logs();
    let transactions = api.http_request(request).await.unwrap();

    for tx in &transactions {
        let error = tx.logs.as_ref().and_then(error_message);
        println!(
            "{} from {}: {}",
            tx.tx_hash,
            tx.sender,
            error.as_deref().unwrap_or("<no error message>")
        );
    }

    let Some(first) = transactions.first() else {
        println!("no failed deploys found");
        return;
    };

    let tx = api
        .http_request(ApiGetTxRequest::new(&first.tx_hash))
        .await
        .unwrap();
    println!("status: {}, timestamp: {}", tx.status, tx.timestamp);

    if let Some(data) = tx.data {
        let data = base64_decode(data).unwrap();
        println!("data size: {} bytes", data.len());
    }
}

/// The failure message is the second topic of the `signalError` event, base64-encoded.
fn error_message(logs: &ApiLogs) -> Option<String> {
    let event = logs
        .events
        .iter()
        .find(|event| event.identifier == "signalError")?;
    let message = base64_decode(event.topics.get(1)?).ok()?;
    String::from_utf8(message).ok()
}
