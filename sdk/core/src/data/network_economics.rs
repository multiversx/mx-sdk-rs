use serde::{Deserialize, Serialize};

// NetworkEconomics holds the network economics details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkEconomics {
    #[serde(rename = "erd_dev_rewards")]
    pub dev_rewards: String,
    #[serde(rename = "erd_epoch_for_economics_data")]
    pub epoch_for_economics_data: u32,
    #[serde(rename = "erd_inflation")]
    pub inflation: String,
    #[serde(rename = "erd_total_fees")]
    pub total_fees: String,
    #[serde(rename = "erd_total_base_staked_value")]
    pub total_base_staked_value: String,
    #[serde(rename = "erd_total_supply")]
    pub total_supply: String,
    #[serde(rename = "erd_total_top_up_value")]
    pub total_top_up_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkEconomicsData {
    pub metrics: NetworkEconomics,
}

// NetworkEconomicsResponse holds the network economics endpoint response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkEconomicsResponse {
    pub error: String,
    pub code: String,
    pub data: Option<NetworkEconomicsData>,
}
