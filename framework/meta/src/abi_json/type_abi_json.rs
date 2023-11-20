use multiversx_sc::abi::*;
use serde::{Deserialize, Serialize};

pub const TYPE_DESCRIPTION_JSON_TYPE_NOT_SPECIFIED: &str = "not-specified";
pub const TYPE_DESCRIPTION_JSON_TYPE_ENUM: &str = "enum";
pub const TYPE_DESCRIPTION_JSON_TYPE_EXPLICIT_ENUM: &str = "explicit-enum";
pub const TYPE_DESCRIPTION_JSON_TYPE_STRUCT: &str = "struct";

#[derive(Serialize, Deserialize)]
pub struct TypeDescriptionJson {
    #[serde(rename = "type")]
    pub content_type: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub docs: Vec<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub variants: Vec<EnumVariantDescriptionJson>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<StructFieldDescriptionJson>,
}

impl From<&TypeDescription> for TypeDescriptionJson {
    fn from(abi: &TypeDescription) -> Self {
        let content_type = match &abi.contents {
            TypeContents::NotSpecified => TYPE_DESCRIPTION_JSON_TYPE_NOT_SPECIFIED,
            TypeContents::Enum(_) => TYPE_DESCRIPTION_JSON_TYPE_ENUM,
            TypeContents::ExplicitEnum(_) => TYPE_DESCRIPTION_JSON_TYPE_EXPLICIT_ENUM,
            TypeContents::Struct(_) => TYPE_DESCRIPTION_JSON_TYPE_STRUCT,
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
            TypeContents::ExplicitEnum(variants) => {
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

impl TypeDescriptionJson {
    pub fn to_type_description(&self, name: &str) -> TypeDescription {
        TypeDescription {
            docs: self.docs.clone(),
            name: name.to_string(),
            contents: match self.content_type.as_str() {
                TYPE_DESCRIPTION_JSON_TYPE_STRUCT => TypeContents::Struct(
                    self.fields
                        .iter()
                        .map(StructFieldDescriptionJson::to_struct_field_description)
                        .collect(),
                ),
                TYPE_DESCRIPTION_JSON_TYPE_ENUM => TypeContents::Enum(
                    self.variants
                        .iter()
                        .map(EnumVariantDescriptionJson::to_enum_variant_description)
                        .collect(),
                ),
                _ => TypeContents::NotSpecified,
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct StructFieldDescriptionJson {
    #[serde(default)]
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

impl StructFieldDescriptionJson {
    pub fn to_struct_field_description(&self) -> StructFieldDescription {
        StructFieldDescription {
            docs: self.docs.clone(),
            name: self.name.clone(),
            field_type: self.field_type.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct EnumVariantDescriptionJson {
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub docs: Vec<String>,

    pub name: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discriminant: Option<usize>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<StructFieldDescriptionJson>,
}

impl From<&EnumVariantDescription> for EnumVariantDescriptionJson {
    fn from(abi: &EnumVariantDescription) -> Self {
        EnumVariantDescriptionJson {
            docs: abi.docs.iter().map(|d| d.to_string()).collect(),
            name: abi.name.to_string(),
            discriminant: Some(abi.discriminant),
            fields: abi
                .fields
                .iter()
                .map(StructFieldDescriptionJson::from)
                .collect(),
        }
    }
}

impl From<&ExplicitEnumVariantDescription> for EnumVariantDescriptionJson {
    fn from(abi: &ExplicitEnumVariantDescription) -> Self {
        EnumVariantDescriptionJson {
            docs: abi.docs.iter().map(|d| d.to_string()).collect(),
            name: abi.name.to_string(),
            discriminant: None,
            fields: Vec::new(),
        }
    }
}

impl EnumVariantDescriptionJson {
    pub fn to_enum_variant_description(&self) -> EnumVariantDescription {
        EnumVariantDescription {
            docs: self.docs.clone(),
            discriminant: self.discriminant.unwrap_or_default(),
            name: self.name.clone(),
            fields: self
                .fields
                .iter()
                .map(StructFieldDescriptionJson::to_struct_field_description)
                .collect(),
        }
    }
}
