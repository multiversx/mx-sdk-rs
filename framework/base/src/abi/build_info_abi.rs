use alloc::{borrow::ToOwned, string::String};

/// Designed to hold metadata of the contract crate.
/// Must be instanced inside the smart contract crate to work,
/// that is why a `create` associated method would not make sense here.
#[derive(Clone, Default, Debug)]
pub struct BuildInfoAbi {
    pub rustc: Option<RustcAbi>,
    pub contract_crate: ContractCrateBuildAbi,
    pub framework: FrameworkBuildAbi,
}

#[derive(Clone, Default, Debug)]
pub struct RustcAbi {
    pub version: String,
    pub commit_hash: String,
    pub commit_date: String,
    pub channel: String,
    pub host: String,
    pub short: String,
}

#[derive(Clone, Default, Debug)]
pub struct ContractCrateBuildAbi {
    pub name: String,
    pub version: String,
    pub git_version: String,
}

impl ContractCrateBuildAbi {
    /// Called from the ABI generator in every contract.
    ///
    /// Note: the values come from env! macros in the caller, to capture the crate info of the contract, not of the framework.
    pub fn new(name: &str, version: &str) -> Self {
        ContractCrateBuildAbi {
            name: name.to_owned(),
            version: version.to_owned(),
            git_version: String::new(),
        }
    }
}

/// Gives the multiversx-sc metadata.
/// Should be instanced via the `create` associated function.
#[derive(Clone, Default, Debug)]
pub struct FrameworkBuildAbi {
    pub name: String,
    pub version: String,
}

impl FrameworkBuildAbi {
    /// Called from the ABI generator in every contract.
    ///
    /// Note: the values are extracted here, this makes them capture the framework crate info.
    pub fn create() -> Self {
        FrameworkBuildAbi {
            name: env!("CARGO_PKG_NAME").to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
        }
    }
}
