use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum GasScheduleVersion {
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

impl GasScheduleVersion {
    pub fn to_content(&self) -> String {
        match self {
            GasScheduleVersion::V1 => include_str!("gas_schedules/gasScheduleV1.toml").to_string(),
            GasScheduleVersion::V2 => include_str!("gas_schedules/gasScheduleV2.toml").to_string(),
            GasScheduleVersion::V3 => include_str!("gas_schedules/gasScheduleV3.toml").to_string(),
            GasScheduleVersion::V4 => include_str!("gas_schedules/gasScheduleV4.toml").to_string(),
            GasScheduleVersion::V5 => include_str!("gas_schedules/gasScheduleV5.toml").to_string(),
            GasScheduleVersion::V6 => include_str!("gas_schedules/gasScheduleV6.toml").to_string(),
            GasScheduleVersion::V7 => include_str!("gas_schedules/gasScheduleV7.toml").to_string(),
            GasScheduleVersion::V8 => include_str!("gas_schedules/gasScheduleV8.toml").to_string(),
        }
    }
}
