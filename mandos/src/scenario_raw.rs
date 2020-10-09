use super::*;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScenarioRaw {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_gas: Option<bool>,
    pub steps: Vec<StepRaw>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "step")]
pub enum StepRaw {
    ExternalSteps {
        path: String,
    },
    
    #[serde(rename_all = "camelCase")]
    SetState {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,

        #[serde(default)]
        #[serde(skip_serializing_if = "BTreeMap::is_empty")]
        accounts: BTreeMap<String, AccountRaw>,
        
        #[serde(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        new_addresses: Vec<NewAddressRaw>,
        
        #[serde(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        block_hashes: Vec<ValueSubTree>,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        previous_block_info: Option<BlockInfoRaw>,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        current_block_info: Option<BlockInfoRaw>,
    },

    #[serde(rename_all = "camelCase")]
    ScCall {
        tx_id: String,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,

        tx: TxCallRaw,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        expect: Option<TxExpectRaw>,
    },

    #[serde(rename_all = "camelCase")]
    ScDeploy {
        tx_id: String,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,

        tx: TxDeployRaw,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        expect: Option<TxExpectRaw>,
    },

    #[serde(rename_all = "camelCase")]
    Transfer {
        tx_id: String,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,

        tx: TxTransferRaw,
    },

    #[serde(rename_all = "camelCase")]
    ValidatorReward {
        tx_id: String,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,

        tx: TxValidatorRewardRaw,
    },

    CheckState {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,

        accounts: CheckAccountsRaw,
    },

    DumpState {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewAddressRaw {
    pub creator_address: ValueSubTree,
    pub creator_nonce: ValueSubTree,
    pub new_address: ValueSubTree,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockInfoRaw {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_timestamp: Option<ValueSubTree>,
    
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_nonce: Option<ValueSubTree>,
    
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_round: Option<ValueSubTree>,
    
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_epoch: Option<ValueSubTree>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxCallRaw {
    pub from: ValueSubTree,
    pub to: ValueSubTree,
    pub value: ValueSubTree,
    pub function: String,

    #[serde(default)]
    pub arguments: Vec<ValueSubTree>,

    pub gas_limit: ValueSubTree,
    pub gas_price: ValueSubTree,
}

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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxTransferRaw {
    pub from: ValueSubTree,
    pub to: ValueSubTree,
    pub value: ValueSubTree,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxValidatorRewardRaw {
    pub to: ValueSubTree,
    pub value: ValueSubTree,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxExpectRaw {

    #[serde(default)]
    pub out: Vec<ValueSubTree>,

    pub status: ValueSubTree,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckLogsRaw::is_default")]
    pub logs: CheckLogsRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<ValueSubTree>,

}
