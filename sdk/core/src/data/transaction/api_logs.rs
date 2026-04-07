use multiversx_chain_core::std::Bech32Address;
use serde::{Deserialize, Serialize};

use super::events::Events;

// ApiLogs represents logs with changed fields' types in order to make it friendly for API's json
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiLogs {
    pub address: Bech32Address,
    pub events: Vec<Events>,
}
