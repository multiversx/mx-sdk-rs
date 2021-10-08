use crate::serde_raw::ValueSubTree;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxValidatorRewardRaw {
    pub to: ValueSubTree,
    pub value: ValueSubTree,
}
