use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BuildInfoAbiJson {
    pub rustc: RustcAbiJson,
}

impl BuildInfoAbiJson {
    pub fn create() -> Self {
        BuildInfoAbiJson {
            rustc: RustcAbiJson::create(),
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
