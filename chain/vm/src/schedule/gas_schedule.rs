use std::mem::MaybeUninit;

use multiversx_chain_vm_executor::OpcodeCost;
use serde::{Deserialize, Serialize};

use super::sections::{
    BaseOperationCost, BaseOpsAPICost, BigFloatAPICost, BigIntAPICost, BuiltInCost, CryptoAPICost,
    DynamicStorageLoad, EthAPICost, ManagedBufferAPICost, MaxPerTransaction,
    MetaChainSystemSCsCost,
};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(default)]
pub struct GasSchedule {
    #[serde(rename = "BuiltInCost")]
    pub builtin_cost: BuiltInCost,
    #[serde(rename = "MetaChainSystemSCsCost")]
    pub metachain_system_scs_cost: MetaChainSystemSCsCost,
    #[serde(rename = "BaseOperationCost")]
    pub base_operation_cost: BaseOperationCost,
    #[serde(rename = "BaseOpsAPICost")]
    pub base_ops_api_cost: BaseOpsAPICost,
    #[serde(rename = "EthAPICost")]
    pub eth_api_cost: EthAPICost,
    #[serde(rename = "BigIntAPICost")]
    pub big_int_api_cost: BigIntAPICost,
    #[serde(rename = "CryptoAPICost")]
    pub crypto_api_cost: CryptoAPICost,
    #[serde(rename = "ManagedBufferAPICost")]
    pub managed_buffer_api_cost: ManagedBufferAPICost,
    #[serde(rename = "BigFloatAPICost")]
    pub big_float_api_cost: BigFloatAPICost,
    #[serde(rename = "WASMOpcodeCost")]
    pub wasm_opcode_cost: OpcodeCost,
    #[serde(rename = "MaxPerTransaction")]
    pub max_per_transaction: MaxPerTransaction,
    #[serde(rename = "DynamicStorageLoad")]
    pub dynamic_storage_load: DynamicStorageLoad,
}

impl GasSchedule {
    pub fn from_toml_str(content: &str) -> Result<Self, toml::de::Error> {
        let full_schedule: GasSchedule = toml::from_str(content)?;
        Ok(full_schedule)
    }

    /// TODO: safer to replace with auto-generated zero const initializer
    pub const fn zeroed() -> GasSchedule {
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
}
