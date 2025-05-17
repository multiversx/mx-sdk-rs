use rustc_version::{version_meta, Version};

pub const WASM32_TARGET: &str = "wasm32-unknown-unknown";
pub const WASM32V1_TARGET: &str = "wasm32v1-none";
const FIRST_RUSTC_VERSION_WITH_WASM32V1_TARGET: Version = Version::new(1, 85, 0);

/// Gets the rustc wasm32 target name.
///
/// It is currently "wasm32v1-none", except for before Rust 1.85, where we use "wasm32-unknown-unknown".
pub fn default_target() -> &'static str {
    if is_wasm32v1_available() {
        WASM32V1_TARGET
    } else {
        WASM32_TARGET
    }
}

pub fn is_wasm32v1_available() -> bool {
    let Ok(version) = version_meta() else {
        return false;
    };

    version.semver >= FIRST_RUSTC_VERSION_WITH_WASM32V1_TARGET
}
