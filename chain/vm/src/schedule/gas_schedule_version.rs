use std::fmt;

use super::GasSchedule;

pub const GAS_SCHEDULE_V1_TOML: &str = include_str!("versions/gasScheduleV1.toml");
pub const GAS_SCHEDULE_V2_TOML: &str = include_str!("versions/gasScheduleV2.toml");
pub const GAS_SCHEDULE_V3_TOML: &str = include_str!("versions/gasScheduleV3.toml");
pub const GAS_SCHEDULE_V4_TOML: &str = include_str!("versions/gasScheduleV4.toml");
pub const GAS_SCHEDULE_V5_TOML: &str = include_str!("versions/gasScheduleV5.toml");
pub const GAS_SCHEDULE_V6_TOML: &str = include_str!("versions/gasScheduleV6.toml");
pub const GAS_SCHEDULE_V7_TOML: &str = include_str!("versions/gasScheduleV7.toml");
pub const GAS_SCHEDULE_V8_TOML: &str = include_str!("versions/gasScheduleV8.toml");
pub const GAS_SCHEDULE_V9_TOML: &str = include_str!("versions/gasScheduleV9.toml");

#[derive(Clone, Copy, Default, Debug)]
pub enum GasScheduleVersion {
    V1 = 1,
    V2 = 2,
    V3 = 3,
    V4 = 4,
    V5 = 5,
    V6 = 6,
    V7 = 7,
    V8 = 8,
    #[default]
    V9 = 9,
}

impl fmt::Display for GasScheduleVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "gasScheduleV{}.toml", *self as u8)
    }
}

impl GasScheduleVersion {
    pub fn from_version_num(version: usize) -> Self {
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
            GasScheduleVersion::V1 => GAS_SCHEDULE_V1_TOML,
            GasScheduleVersion::V2 => GAS_SCHEDULE_V2_TOML,
            GasScheduleVersion::V3 => GAS_SCHEDULE_V3_TOML,
            GasScheduleVersion::V4 => GAS_SCHEDULE_V4_TOML,
            GasScheduleVersion::V5 => GAS_SCHEDULE_V5_TOML,
            GasScheduleVersion::V6 => GAS_SCHEDULE_V6_TOML,
            GasScheduleVersion::V7 => GAS_SCHEDULE_V7_TOML,
            GasScheduleVersion::V8 => GAS_SCHEDULE_V8_TOML,
            GasScheduleVersion::V9 => GAS_SCHEDULE_V9_TOML,
        }
    }

    pub fn load_gas_schedule(&self) -> GasSchedule {
        GasSchedule::from_toml_str(self.toml_str()).expect("error parsing gas schedule toml")
    }
}
