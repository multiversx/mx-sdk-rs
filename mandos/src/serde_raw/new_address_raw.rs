use serde::{Deserialize, Serialize};

use crate::serde_raw::ValueSubTree;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewAddressRaw {
    pub creator_address: ValueSubTree,
    pub creator_nonce: ValueSubTree,
    pub new_address: ValueSubTree,
}
