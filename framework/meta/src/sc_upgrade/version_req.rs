/// Crate version requirements, as expressed in Cargo.toml. A very crude version.
///
/// TODO: replace with semver::VersionReq at some point.
#[derive(Debug, Clone)]
pub struct VersionReq {
    pub semver: String,
    pub is_strict: bool,
}

impl VersionReq {
    pub fn from_string(raw: String) -> Self {
        if let Some(stripped_version) = raw.strip_prefix('=') {
            VersionReq {
                semver: stripped_version.to_string(),
                is_strict: true,
            }
        } else {
            VersionReq {
                semver: raw,
                is_strict: false,
            }
        }
    }

    pub fn into_string(self) -> String {
        if self.is_strict {
            format!("={}", self.semver)
        } else {
            self.semver
        }
    }
}
