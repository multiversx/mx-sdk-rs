use multiversx_sc::abi::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TypeDescriptionJson {
    #[serde(rename = "type")]
    pub content_type: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub docs: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub variants: Vec<EnumVariantDescriptionJson>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<StructFieldDescriptionJson>,
}

impl From<&TypeDescription> for TypeDescriptionJson {
    fn from(abi: &TypeDescription) -> Self {
        let content_type = match &abi.contents {
            TypeContents::NotSpecified => "not_specified",
            TypeContents::Enum(_) => "enum",
            TypeContents::Struct(_) => "struct",
        };
        let mut type_desc_json = TypeDescriptionJson {
            content_type: content_type.to_string(),
            docs: abi.docs.iter().map(|line| line.to_string()).collect(),
            variants: Vec::new(),
            fields: Vec::new(),
        };
        match &abi.contents {
            TypeContents::Struct(fields) => {
                for field in fields {
                    type_desc_json
                        .fields
                        .push(StructFieldDescriptionJson::from(field));
                }
            },
            TypeContents::Enum(variants) => {
                for variant in variants {
                    type_desc_json
                        .variants
                        .push(EnumVariantDescriptionJson::from(variant));
                }
            },
            _ => {},
        }

        type_desc_json
    }
}

#[derive(Serialize, Deserialize)]
pub struct StructFieldDescriptionJson {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub docs: Vec<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
}

impl From<&StructFieldDescription> for StructFieldDescriptionJson {
    fn from(abi: &StructFieldDescription) -> Self {
        StructFieldDescriptionJson {
            docs: abi.docs.iter().map(|d| d.to_string()).collect(),
            name: abi.name.to_string(),
            field_type: abi.field_type.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct EnumVariantDescriptionJson {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub docs: Vec<String>,
    pub name: String,
    pub discriminant: usize,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<StructFieldDescriptionJson>,
}

impl From<&EnumVariantDescription> for EnumVariantDescriptionJson {
    fn from(abi: &EnumVariantDescription) -> Self {
        EnumVariantDescriptionJson {
            docs: abi.docs.iter().map(|d| d.to_string()).collect(),
            name: abi.name.to_string(),
            discriminant: abi.discriminant,
            fields: abi
                .fields
                .iter()
                .map(StructFieldDescriptionJson::from)
                .collect(),
        }
    }
}
