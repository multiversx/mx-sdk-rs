use multiversx_chain_core::std::Bech32Address;
use serde::{Deserialize, Serialize};

use super::events::Events;

/// Logs with changed fields' types in order to make it friendly for the API's JSON.
///
/// Corresponds to [`ApiLogs`](https://github.com/multiversx/mx-chain-core-go/blob/main/data/transaction/apiTransactionResult.go) in mx-chain-core-go.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiLogs {
    pub address: Bech32Address,
    pub events: Vec<Events>,
}
