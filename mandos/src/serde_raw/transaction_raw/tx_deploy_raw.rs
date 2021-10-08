use crate::serde_raw::ValueSubTree;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxDeployRaw {
    pub from: ValueSubTree,
    pub value: ValueSubTree,

    pub contract_code: ValueSubTree,

    #[serde(default)]
    pub arguments: Vec<ValueSubTree>,

    pub gas_limit: ValueSubTree,
    pub gas_price: ValueSubTree,
}
