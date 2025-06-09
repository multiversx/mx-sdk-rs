pub const GAS_SCHEDULE_V1_TOML: &str = include_str!("gas_schedules/gasScheduleV1.toml");
pub const GAS_SCHEDULE_V2_TOML: &str = include_str!("gas_schedules/gasScheduleV2.toml");
pub const GAS_SCHEDULE_V3_TOML: &str = include_str!("gas_schedules/gasScheduleV3.toml");
pub const GAS_SCHEDULE_V4_TOML: &str = include_str!("gas_schedules/gasScheduleV4.toml");
pub const GAS_SCHEDULE_V5_TOML: &str = include_str!("gas_schedules/gasScheduleV5.toml");
pub const GAS_SCHEDULE_V6_TOML: &str = include_str!("gas_schedules/gasScheduleV6.toml");
pub const GAS_SCHEDULE_V7_TOML: &str = include_str!("gas_schedules/gasScheduleV7.toml");
pub const GAS_SCHEDULE_V8_TOML: &str = include_str!("gas_schedules/gasScheduleV8.toml");

pub fn gas_schedule_toml_by_version(version: u16) -> &'static str {
    match version {
        1 => GAS_SCHEDULE_V1_TOML,
        2 => GAS_SCHEDULE_V2_TOML,
        3 => GAS_SCHEDULE_V3_TOML,
        4 => GAS_SCHEDULE_V4_TOML,
        5 => GAS_SCHEDULE_V5_TOML,
        6 => GAS_SCHEDULE_V6_TOML,
        7 => GAS_SCHEDULE_V7_TOML,
        8 => GAS_SCHEDULE_V8_TOML,
        _ => panic!("Invalid gas schedule TOML version {version}"),
    }
}
