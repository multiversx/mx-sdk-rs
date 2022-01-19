use crate::serde_raw::ValueSubTree;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxDeployRaw {
    pub from: ValueSubTree,

    /// Backwards compatibility only.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egld_value: Option<ValueSubTree>,

    pub contract_code: ValueSubTree,

    #[serde(default)]
    pub arguments: Vec<ValueSubTree>,

    pub gas_limit: ValueSubTree,
    pub gas_price: ValueSubTree,
}
