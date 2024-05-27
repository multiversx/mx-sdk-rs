use multiversx_sc::abi::{EsdtAttributeAbi, TypeName};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EsdtAttributeJson {
    pub ticker: String,
    #[serde(rename = "type")]
    pub ty: TypeName,
}

impl From<&EsdtAttributeAbi> for EsdtAttributeJson {
    fn from(attr: &EsdtAttributeAbi) -> Self {
        EsdtAttributeJson {
            ticker: attr.ticker.to_owned(),
            ty: attr.ty.clone(),
        }
    }
}
