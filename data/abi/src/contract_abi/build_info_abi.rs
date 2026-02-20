use alloc::{borrow::ToOwned, string::String};

/// Designed to hold metadata of the contract crate.
/// Must be instanced inside the smart contract crate to work,
/// that is why a `create` associated method would not make sense here.
#[derive(Clone, Default, Debug)]
pub struct BuildInfoAbi {
    pub rustc: Option<RustcAbi>,
    pub contract_crate: ContractCrateBuildAbi,
    pub abi: FrameworkBuildAbi,
    pub framework: FrameworkBuildAbi,
}

impl BuildInfoAbi {
    pub fn dummy() -> Self {
        BuildInfoAbi {
            rustc: None,
            contract_crate: ContractCrateBuildAbi {
                name: "contract-crate".to_owned(),
                version: "0.0.0".to_owned(),
                git_version: "0.0.0".to_owned(),
            },
            abi: FrameworkBuildAbi::new("abi-crate", "0.0.0"),
            framework: FrameworkBuildAbi::new("framework-crate", "0.0.0"),
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct RustcAbi {
    pub version: String,
    pub commit_hash: String,
    pub commit_date: String,
    pub build_date: Option<String>,
    pub channel: String,
    pub host: String,
    pub short: String,
    pub llvm_version: Option<String>,
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
    pub fn new(name: &str, version: &str) -> Self {
        FrameworkBuildAbi {
            name: name.to_owned(),
            version: version.to_owned(),
        }
    }

    /// Called from the ABI generator in every contract.
    ///
    /// Contains the ABI crate name and current version.
    pub fn abi_crate() -> Self {
        FrameworkBuildAbi::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
    }

    pub fn dummy() -> Self {
        FrameworkBuildAbi::new("", "")
    }
}
