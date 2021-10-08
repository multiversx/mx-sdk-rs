use serde::{Deserialize, Serialize};

use crate::serde_raw::StepRaw;

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
