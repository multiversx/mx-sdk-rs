use crate::serde_raw::ValueSubTree;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxQueryRaw {
    pub to: ValueSubTree,
    pub function: String,

    #[serde(default)]
    pub arguments: Vec<ValueSubTree>,
}
