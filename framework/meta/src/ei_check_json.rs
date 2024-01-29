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
    pub fn new(check_ei: &Option<EIVersion>, status: bool) -> Self {
        EiCheckJson {
            ei_version: get_ei_version(check_ei),
            ok: status,
        }
    }
}

fn get_ei_version(check_ei: &Option<EIVersion>) -> String {
    match check_ei {
        Some(ei) => ei.name().to_string(),
        None => "ignore".to_string(),
    }
}
