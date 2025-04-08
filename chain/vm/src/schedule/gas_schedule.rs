use multiversx_chain_vm_executor::OpcodeCost;
use serde::{Deserialize, Serialize};

use super::gas_schedule_version::GasScheduleVersion;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct GasSchedule {
    #[serde(rename = "WASMOpcodeCost")]
    pub wasm_opcode_cost: OpcodeCost,
}

impl GasSchedule {
    pub fn new(gas_schedule: GasScheduleVersion) -> Self {
        let toml_content = gas_schedule.to_content();
        Self::from_toml_str(&toml_content).unwrap()
    }

    pub fn from_toml_str(content: &str) -> Result<Self, toml::de::Error> {
        let full_schedule: GasSchedule = toml::from_str(content)?;
        Ok(full_schedule)
    }
}
