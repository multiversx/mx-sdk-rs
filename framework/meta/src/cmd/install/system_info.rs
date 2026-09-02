#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemInfo {
    Linux,
    MacOs,
    Windows,
}

pub fn get_system_info() -> SystemInfo {
    let os = std::env::consts::OS;
    match os {
        "linux" => SystemInfo::Linux,
        "macos" => SystemInfo::MacOs,
        "windows" => SystemInfo::Windows,
        _ => panic!("unknown configuration: {os}"),
    }
}
