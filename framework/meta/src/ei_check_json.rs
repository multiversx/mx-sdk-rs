use serde::{Deserialize, Serialize};

use crate::ei::EIVersion;

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EiCheckJson {
    #[serde(default)]
    pub ei_version: String,

    #[serde(default)]
    pub ok: bool,
}

impl EiCheckJson {
    pub fn new(check_ei: &Option<EIVersion>, status: bool) -> Option<Self> {
        if let Some(ei) = check_ei {
            return Some(EiCheckJson {
                ei_version: ei.name().to_string(),
                ok: status,
            });
        }

        None
    }
}
