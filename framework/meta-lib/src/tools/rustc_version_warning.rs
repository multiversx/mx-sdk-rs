use rustc_version::Channel;
use semver::Version;

const MX_STABLE_VERSION: Version = Version::new(0, 50, 0);
const MX_BULK_MEMORY_FIX_VERSION: Version = Version::new(0, 58, 0);
const RUSTC_BULK_MEMORY_VERSION: Version = Version::new(1, 87, 0);

/// Writes a warning if there is an incompatiblilty between the framework version and the Rustc version.
pub fn rustc_version_warning(
    contract_name: &str,
    framework_version: Version,
    rustc_version: &super::RustcVersion,
) -> Option<String> {
    let rustc_semver = &rustc_version.version_meta.semver;

    if framework_version < MX_STABLE_VERSION
        && rustc_version.version_meta.channel != Channel::Nightly
    {
        return Some(format!("
WARNING! Contract {contract_name} is using multiversx-sc v{framework_version} with Rust {rustc_semver}.
Compiling contracts using multiversx-sc before {MX_STABLE_VERSION} requires nightly Rust!
Recommended Rust versions: nightly-2023-12-11 or nightly-2024-05-22."));
    }

    if framework_version < MX_BULK_MEMORY_FIX_VERSION && *rustc_semver >= RUSTC_BULK_MEMORY_VERSION
    {
        return Some(format!("
WARNING! Contract {contract_name} is using multiversx-sc v{framework_version} with Rust {rustc_semver}.
Compiling contracts using multiversx-sc before {MX_BULK_MEMORY_FIX_VERSION} with Rust v{RUSTC_BULK_MEMORY_VERSION} or newer is not supported!
Either upgrade the framework (to {MX_BULK_MEMORY_FIX_VERSION} or newer), or downgrade the compiler to v1.86!"));
    }

    // This condition will rarely be hit.
    //
    // This is because in order to reach this point, the local meta crate needs to be built,
    // which will fail if the rustc version is below minimum.
    //
    // It can be reached if rustc-version was configured explicitly in sc-config.toml.
    if let Some(minimum_version) = minimum_rustc_version(&framework_version)
        && *rustc_semver < minimum_version
    {
        return Some(format!("
WARNING! Contract {contract_name} is using multiversx-sc v{framework_version} with Rust {rustc_semver}.
This is below the minimum rustc version for this release, which is v{minimum_version}."));
    }

    None
}

fn minimum_rustc_version(framework_version: &Version) -> Option<Version> {
    if *framework_version < Version::new(0, 50, 0) {
        return None;
    }

    if *framework_version < Version::new(0, 57, 0) {
        return Some(Version::new(1, 78, 0));
    }

    Some(Version::new(1, 83, 0))
}
