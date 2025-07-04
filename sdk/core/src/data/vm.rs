use multiversx_chain_core::std::Bech32Address;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum CallType {
    // DirectCall means that the call is an explicit SC invocation originating from a user Transaction
    DirectCall = 0,

    // AsynchronousCall means that the invocation was performed from within
    // another SmartContract from another Shard, using asyncCall
    AsynchronousCall = 1,

    // AsynchronousCallBack means that an AsynchronousCall was performed
    // previously, and now the control returns to the caller SmartContract's callBack method
    AsynchronousCallBack = 2,

    // ESDTTransferAndExecute means that there is a smart contract execution after the ESDT transfer
    // this is needed in order to skip the check whether a contract is payable or not
    ESDTTransferAndExecute = 3,
}

// VmValueRequest defines the request struct for values available in a VM
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VMQueryInput {
    pub sc_address: Bech32Address,
    pub func_name: String,
    pub args: Vec<String>,
}

// LogEntryApi is a wrapper over vmcommon's LogEntry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEntryApi {
    pub identifier: String,
    pub address: Bech32Address,
    pub topics: Vec<String>,
    pub data: String,
}

// OutputTransferApi is a wrapper over vmcommon's OutputTransfer
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputTransferApi {
    pub value: String,
    pub gas_limit: u64,
    pub data: String,
    pub call_type: CallType,
    pub sender_address: Bech32Address,
}

// OutputAccountApi is a wrapper over vmcommon's OutputAccount
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputAccountApi {
    address: Bech32Address,
    nonce: u64,

    // TODO: unknown type of data
    // balance: Option<String>,
    balance_delta: u64,
    storage_updates: Option<HashMap<String, StorageUpdateApi>>,
    code: Option<String>,
    code_metadata: Option<String>,

    #[serde(default)]
    output_transfers: Vec<OutputTransferApi>,
    call_type: CallType,
}

// StorageUpdateApi is a wrapper over vmcommon's StorageUpdate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageUpdateApi {
    offset: String,
    data: String,
}

// VMOutputApi is a wrapper over the vmcommon's VMOutput
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VMOutputApi {
    #[serde(default)]
    pub return_data: Vec<String>,
    pub return_code: String,
    pub return_message: String,
    pub gas_remaining: u64,
    pub gas_refund: u64,
    pub output_accounts: HashMap<String, OutputAccountApi>,
    pub deleted_accounts: Option<Vec<String>>,
    pub touched_accounts: Option<Vec<String>>,
    pub logs: Option<Vec<LogEntryApi>>,
}

// VmValuesResponseData follows the format of the data field in an API response for a VM values query
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VmValuesResponseData {
    pub data: VMOutputApi,
}

// ResponseVmValue defines a wrapper over string containing returned data in hex format
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseVmValue {
    pub data: Option<VmValuesResponseData>,
    pub error: String,
    pub code: String,
}
