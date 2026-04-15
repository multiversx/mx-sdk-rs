use std::fmt;

use super::GasSchedule;

#[derive(Clone, Copy, Default, Debug)]
pub enum GasScheduleVersion {
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    #[default]
    V9,
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
    pub fn from_version_num(version: u16) -> Self {
        match version {
            1 => GasScheduleVersion::V1,
            2 => GasScheduleVersion::V2,
            3 => GasScheduleVersion::V3,
            4 => GasScheduleVersion::V4,
            5 => GasScheduleVersion::V5,
            6 => GasScheduleVersion::V6,
            7 => GasScheduleVersion::V7,
            8 => GasScheduleVersion::V8,
            9 => GasScheduleVersion::V9,
            _ => panic!("Invalid gas schedule version {version}"),
        }
    }

    pub fn toml_str(&self) -> &'static str {
        match self {
            GasScheduleVersion::V1 => super::GAS_SCHEDULE_V1_TOML,
            GasScheduleVersion::V2 => super::GAS_SCHEDULE_V2_TOML,
            GasScheduleVersion::V3 => super::GAS_SCHEDULE_V3_TOML,
            GasScheduleVersion::V4 => super::GAS_SCHEDULE_V4_TOML,
            GasScheduleVersion::V5 => super::GAS_SCHEDULE_V5_TOML,
            GasScheduleVersion::V6 => super::GAS_SCHEDULE_V6_TOML,
            GasScheduleVersion::V7 => super::GAS_SCHEDULE_V7_TOML,
            GasScheduleVersion::V8 => super::GAS_SCHEDULE_V8_TOML,
            GasScheduleVersion::V9 => super::GAS_SCHEDULE_V9_TOML,
        }
    }

    pub fn load_gas_schedule(&self) -> GasSchedule {
        parse_gas_schedule(self.toml_str())
    }
}
