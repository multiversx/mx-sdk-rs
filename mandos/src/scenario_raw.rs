use super::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Mapped 1-on-1 with the JSON. No complex logic here, just a basic interface with the JSON.
/// The conversion to `Scenario` adds all additional functionality.
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

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_schedule: Option<String>,

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
        #[serde(default)]
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
    ScQuery {
        #[serde(default)]
        tx_id: String,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,

        tx: TxQueryRaw,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        expect: Option<TxExpectRaw>,
    },

    #[serde(rename_all = "camelCase")]
    ScDeploy {
        #[serde(default)]
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
        #[serde(default)]
        tx_id: String,

        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,

        tx: TxTransferRaw,
    },

    #[serde(rename_all = "camelCase")]
    ValidatorReward {
        #[serde(default)]
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

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_random_seed: Option<ValueSubTree>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxESDTRaw {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_identifier: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "ValueSubTree::is_empty_string")]
    pub nonce: ValueSubTree,

    #[serde(default)]
    #[serde(skip_serializing_if = "ValueSubTree::is_empty_string")]
    pub value: ValueSubTree,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxCallRaw {
    pub from: ValueSubTree,
    pub to: ValueSubTree,
    pub value: ValueSubTree,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub esdt: Option<TxESDTRaw>,

    pub function: String,

    #[serde(default)]
    pub arguments: Vec<ValueSubTree>,

    pub gas_limit: ValueSubTree,
    pub gas_price: ValueSubTree,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxQueryRaw {
    pub to: ValueSubTree,
    pub function: String,

    #[serde(default)]
    pub arguments: Vec<ValueSubTree>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxDeployRaw {
    pub from: ValueSubTree,
    pub value: ValueSubTree,

    #[serde(default)]
    #[serde(skip_serializing_if = "ValueSubTree::is_empty_string")]
    pub esdt_value: ValueSubTree,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub esdt_token_identifier: Option<ValueSubTree>,
    pub nonce: ValueSubTree,
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

    #[serde(default)]
    #[serde(skip_serializing_if = "ValueSubTree::is_empty_string")]
    pub esdt_value: ValueSubTree,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub esdt_token_identifier: Option<ValueSubTree>,

    #[serde(default)]
    #[serde(skip_serializing_if = "ValueSubTree::is_empty_string")]
    pub nonce: ValueSubTree,
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
    pub out: Vec<CheckBytesValueRaw>,

    #[serde(default)]
    pub status: CheckBytesValueRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckLogsRaw::is_default")]
    pub logs: CheckLogsRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub message: CheckBytesValueRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub gas: CheckBytesValueRaw,

    #[serde(default)]
    #[serde(skip_serializing_if = "CheckBytesValueRaw::is_unspecified")]
    pub refund: CheckBytesValueRaw,
}
