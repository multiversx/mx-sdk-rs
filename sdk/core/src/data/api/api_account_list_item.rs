use multiversx_chain_core::std::Bech32Address;
use serde::{Deserialize, Serialize};

/// Item of the accounts list of the MultiversX API (`/accounts`), with only the address and cursor requested.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiAccountListItem {
    pub address: Bech32Address,

    /// Pagination cursor (base64): pass it as `searchAfter` to get the items after this one.
    pub search_after: Option<String>,
}
