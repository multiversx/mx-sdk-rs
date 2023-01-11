use multiversx_sc::abi::{BuildInfoAbi, ContractCrateBuildAbi, FrameworkBuildAbi};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildInfoAbiJson {
    pub rustc: RustcAbiJson,
    pub contract_crate: ContractCrateBuildAbiJson,
    pub framework: FrameworkBuildAbiJson,
}

impl From<&BuildInfoAbi> for BuildInfoAbiJson {
    fn from(abi: &BuildInfoAbi) -> Self {
        BuildInfoAbiJson {
            rustc: RustcAbiJson::create(),
            contract_crate: ContractCrateBuildAbiJson::from(&abi.contract_crate),
            framework: FrameworkBuildAbiJson::from(&abi.framework),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustcAbiJson {
    pub version: String,
    pub commit_hash: String,
    pub commit_date: String,
    pub channel: String,
    pub short: String,
}

impl RustcAbiJson {
    pub fn create() -> Self {
        let meta = rustc_version::version_meta().unwrap();
        RustcAbiJson {
            version: meta.semver.to_string(),
            commit_hash: meta.commit_hash.unwrap_or_default(),
            commit_date: meta.commit_date.unwrap_or_default(),
            channel: format!("{:?}", meta.channel),
            short: meta.short_version_string,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractCrateBuildAbiJson {
    pub name: String,
    pub version: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub git_version: String,
}

impl From<&ContractCrateBuildAbi> for ContractCrateBuildAbiJson {
    fn from(abi: &ContractCrateBuildAbi) -> Self {
        ContractCrateBuildAbiJson {
            name: abi.name.to_string(),
            version: abi.version.to_string(),
            git_version: abi.git_version.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FrameworkBuildAbiJson {
    pub name: String,
    pub version: String,
}

impl From<&FrameworkBuildAbi> for FrameworkBuildAbiJson {
    fn from(abi: &FrameworkBuildAbi) -> Self {
        FrameworkBuildAbiJson {
            name: abi.name.to_string(),
            version: abi.version.to_string(),
        }
    }
}
