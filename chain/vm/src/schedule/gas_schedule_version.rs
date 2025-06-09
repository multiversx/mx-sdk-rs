use std::fmt;

use super::GasSchedule;

#[derive(Clone, Copy, Default, Debug)]
pub enum GasScheduleVersion {
    #[default]
    Zero,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
}

impl fmt::Display for GasScheduleVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "gasScheduleV{}.toml", *self as u8 + 1)
    }
}

pub fn parse_gas_schedule(content: &str) -> GasSchedule {
    GasSchedule::from_toml_str(content).expect("error parsing gas schedule toml")
}

impl GasScheduleVersion {
    pub fn load_gas_schedule(&self) -> GasSchedule {
        match self {
            GasScheduleVersion::Zero => GasSchedule::default(),
            GasScheduleVersion::V1 => parse_gas_schedule(super::GAS_SCHEDULE_V1_TOML),
            GasScheduleVersion::V2 => parse_gas_schedule(super::GAS_SCHEDULE_V2_TOML),
            GasScheduleVersion::V3 => parse_gas_schedule(super::GAS_SCHEDULE_V3_TOML),
            GasScheduleVersion::V4 => parse_gas_schedule(super::GAS_SCHEDULE_V4_TOML),
            GasScheduleVersion::V5 => parse_gas_schedule(super::GAS_SCHEDULE_V5_TOML),
            GasScheduleVersion::V6 => parse_gas_schedule(super::GAS_SCHEDULE_V6_TOML),
            GasScheduleVersion::V7 => parse_gas_schedule(super::GAS_SCHEDULE_V7_TOML),
            GasScheduleVersion::V8 => parse_gas_schedule(super::GAS_SCHEDULE_V8_TOML),
        }
    }
}
