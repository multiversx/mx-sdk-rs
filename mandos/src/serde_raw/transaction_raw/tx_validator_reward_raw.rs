use crate::ValueSubTree;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxValidatorRewardRaw {
    pub to: ValueSubTree,
    pub value: ValueSubTree,
}
