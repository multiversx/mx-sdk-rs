use multiversx_sc::abi::{BuildInfoAbi, ContractCrateBuildAbi, FrameworkBuildAbi, RustcAbi};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildInfoAbiJson {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rustc: Option<RustcAbiJson>,
    pub contract_crate: ContractCrateBuildAbiJson,
    pub framework: FrameworkBuildAbiJson,
}

impl From<&BuildInfoAbi> for BuildInfoAbiJson {
    fn from(abi: &BuildInfoAbi) -> Self {
        BuildInfoAbiJson {
            rustc: abi.rustc.as_ref().map(RustcAbiJson::from),
            contract_crate: ContractCrateBuildAbiJson::from(&abi.contract_crate),
            framework: FrameworkBuildAbiJson::from(&abi.framework),
        }
    }
}

impl From<&BuildInfoAbiJson> for BuildInfoAbi {
    fn from(abi_json: &BuildInfoAbiJson) -> Self {
        BuildInfoAbi {
            rustc: abi_json.rustc.as_ref().map(RustcAbi::from),
            contract_crate: ContractCrateBuildAbi::from(&abi_json.contract_crate),
            framework: FrameworkBuildAbi::from(&abi_json.framework),
        }
    }
}

impl From<BuildInfoAbiJson> for BuildInfoAbi {
    fn from(abi_json: BuildInfoAbiJson) -> Self {
        BuildInfoAbi::from(&abi_json)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustcAbiJson {
    pub version: String,
    pub commit_hash: String,
    pub commit_date: String,
    pub channel: String,
    pub host: String,
    pub short: String,
}

impl From<&RustcAbi> for RustcAbiJson {
    fn from(abi: &RustcAbi) -> Self {
        RustcAbiJson {
            version: abi.version.clone(),
            commit_hash: abi.commit_hash.clone(),
            commit_date: abi.commit_date.clone(),
            channel: abi.channel.clone(),
            host: abi.host.clone(),
            short: abi.short.clone(),
        }
    }
}

impl From<&RustcAbiJson> for RustcAbi {
    fn from(abi_json: &RustcAbiJson) -> Self {
        RustcAbi {
            version: abi_json.version.clone(),
            commit_hash: abi_json.commit_hash.clone(),
            commit_date: abi_json.commit_date.clone(),
            channel: abi_json.channel.clone(),
            host: abi_json.host.clone(),
            short: abi_json.short.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractCrateBuildAbiJson {
    pub name: String,
    pub version: String,
    #[serde(default)]
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

impl From<&ContractCrateBuildAbiJson> for ContractCrateBuildAbi {
    fn from(abi_json: &ContractCrateBuildAbiJson) -> Self {
        ContractCrateBuildAbi {
            name: abi_json.name.clone(),
            version: abi_json.version.clone(),
            git_version: abi_json.git_version.clone(),
        }
    }
}

impl From<ContractCrateBuildAbiJson> for ContractCrateBuildAbi {
    fn from(abi: ContractCrateBuildAbiJson) -> Self {
        ContractCrateBuildAbi::from(&abi)
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

impl From<&FrameworkBuildAbiJson> for FrameworkBuildAbi {
    fn from(abi_json: &FrameworkBuildAbiJson) -> Self {
        FrameworkBuildAbi {
            name: abi_json.name.clone(),
            version: abi_json.version.clone(),
        }
    }
}

impl From<FrameworkBuildAbiJson> for FrameworkBuildAbi {
    fn from(abi: FrameworkBuildAbiJson) -> Self {
        FrameworkBuildAbi::from(&abi)
    }
}
