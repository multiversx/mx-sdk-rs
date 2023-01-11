/// Deisgned to hold metadata of the contract crate.
/// Must be instanced inside the smart contract crate to work,
/// that is why a `create` associated method would not make sense here.
#[derive(Clone, Default, Debug)]
pub struct BuildInfoAbi {
    pub contract_crate: ContractCrateBuildAbi,
    pub framework: FrameworkBuildAbi,
}

#[derive(Clone, Default, Debug)]
pub struct ContractCrateBuildAbi {
    pub name: &'static str,
    pub version: &'static str,
    pub git_version: &'static str,
}

/// Gives the multiversx-sc metadata.
/// Should be instanced via the `create` associated function.
#[derive(Clone, Default, Debug)]
pub struct FrameworkBuildAbi {
    pub name: &'static str,
    pub version: &'static str,
}

impl FrameworkBuildAbi {
    pub fn create() -> Self {
        FrameworkBuildAbi {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
        }
    }
}
