use multiversx_sc::abi::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InputAbiJson {
    #[serde(rename = "name")]
    pub arg_name: String,

    #[serde(rename = "type")]
    pub type_name: String,

    /// Bool that is only serialized when true
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_arg: Option<bool>,
}

impl From<&InputAbi> for InputAbiJson {
    fn from(abi: &InputAbi) -> Self {
        InputAbiJson {
            arg_name: abi.arg_name.to_string(),
            type_name: abi.type_names.abi.clone(),
            multi_arg: if abi.multi_arg { Some(true) } else { None },
        }
    }
}

impl From<&InputAbiJson> for InputAbi {
    fn from(abi: &InputAbiJson) -> Self {
        InputAbi {
            arg_name: abi.arg_name.to_string(),
            type_names: TypeNames::from_abi(abi.type_name.clone()),
            multi_arg: abi.multi_arg.unwrap_or(false),
        }
    }
}

impl From<InputAbiJson> for InputAbi {
    fn from(abi: InputAbiJson) -> Self {
        InputAbi::from(&abi)
    }
}

#[derive(Serialize, Deserialize)]
pub struct OutputAbiJson {
    #[serde(rename = "name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub output_name: String,
    #[serde(rename = "type")]
    pub type_name: String,
    /// Bool that is only serialized when true
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_result: Option<bool>,
}

impl From<&OutputAbi> for OutputAbiJson {
    fn from(abi: &OutputAbi) -> Self {
        OutputAbiJson {
            output_name: abi.output_name.clone(),
            type_name: abi.type_names.abi.clone(),
            multi_result: if abi.multi_result { Some(true) } else { None },
        }
    }
}

impl From<&OutputAbiJson> for OutputAbi {
    fn from(abi: &OutputAbiJson) -> Self {
        OutputAbi {
            output_name: abi.output_name.clone(),
            type_names: TypeNames::from_abi(abi.type_name.clone()),
            multi_result: abi.multi_result.unwrap_or(false),
        }
    }
}

impl From<OutputAbiJson> for OutputAbi {
    fn from(abi: OutputAbiJson) -> Self {
        OutputAbi::from(&abi)
    }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum EndpointMutabilityAbiJson {
    #[default]
    Mutable,
    Readonly,
    Pure,
}

/// Same as EndpointAbiJson but ignores the name
#[derive(Serialize, Deserialize)]
pub struct ConstructorAbiJson {
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub docs: Vec<String>,
    #[serde(rename = "payableInTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub payable_in_tokens: Vec<String>,
    pub inputs: Vec<InputAbiJson>,
    pub outputs: Vec<OutputAbiJson>,
}

impl From<&EndpointAbi> for ConstructorAbiJson {
    fn from(abi: &EndpointAbi) -> Self {
        ConstructorAbiJson {
            docs: abi.docs.iter().map(|d| d.to_string()).collect(),
            payable_in_tokens: abi
                .payable_in_tokens
                .iter()
                .map(|d| d.to_string())
                .collect(),
            inputs: abi.inputs.iter().map(InputAbiJson::from).collect(),
            outputs: abi.outputs.iter().map(OutputAbiJson::from).collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct EndpointAbiJson {
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub docs: Vec<String>,
    pub name: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "onlyOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_owner: Option<bool>,

    #[serde(rename = "onlyAdmin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_admin: Option<bool>,

    #[serde(default)]
    pub mutability: EndpointMutabilityAbiJson,

    #[serde(rename = "payableInTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub payable_in_tokens: Vec<String>,

    pub inputs: Vec<InputAbiJson>,

    pub outputs: Vec<OutputAbiJson>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_multiple_var_args: Option<bool>,
}

impl From<&EndpointAbi> for EndpointAbiJson {
    fn from(abi: &EndpointAbi) -> Self {
        EndpointAbiJson {
            docs: abi.docs.iter().map(|d| d.to_string()).collect(),
            name: abi.name.to_string(),
            title: abi.title.clone(),
            only_owner: if abi.only_owner { Some(true) } else { None },
            only_admin: if abi.only_admin { Some(true) } else { None },
            mutability: match abi.mutability {
                EndpointMutabilityAbi::Mutable => EndpointMutabilityAbiJson::Mutable,
                EndpointMutabilityAbi::Readonly => EndpointMutabilityAbiJson::Readonly,
                EndpointMutabilityAbi::Pure => EndpointMutabilityAbiJson::Pure,
            },
            payable_in_tokens: abi
                .payable_in_tokens
                .iter()
                .map(|d| d.to_string())
                .collect(),
            inputs: abi.inputs.iter().map(InputAbiJson::from).collect(),
            outputs: abi.outputs.iter().map(OutputAbiJson::from).collect(),
            labels: abi.labels.clone(),
            allow_multiple_var_args: if abi.allow_multiple_var_args {
                Some(true)
            } else {
                None
            },
        }
    }
}

impl From<&EndpointAbiJson> for EndpointAbi {
    fn from(abi: &EndpointAbiJson) -> Self {
        EndpointAbi {
            docs: abi.docs.iter().map(|d| d.to_string()).collect(),
            name: abi.name.to_string(),
            only_owner: abi.only_owner.unwrap_or(false),
            only_admin: abi.only_admin.unwrap_or(false),
            mutability: match abi.mutability {
                EndpointMutabilityAbiJson::Mutable => EndpointMutabilityAbi::Mutable,
                EndpointMutabilityAbiJson::Readonly => EndpointMutabilityAbi::Readonly,
                EndpointMutabilityAbiJson::Pure => EndpointMutabilityAbi::Pure,
            },
            payable_in_tokens: abi
                .payable_in_tokens
                .iter()
                .map(|d| d.to_string())
                .collect(),
            inputs: abi.inputs.iter().map(InputAbi::from).collect(),
            outputs: abi.outputs.iter().map(OutputAbi::from).collect(),
            labels: abi.labels.clone(),
            allow_multiple_var_args: abi.allow_multiple_var_args.unwrap_or(false),
            rust_method_name: abi.name.clone(),
            title: None,
            endpoint_type: EndpointTypeAbi::Endpoint,
        }
    }
}

impl From<EndpointAbiJson> for EndpointAbi {
    fn from(abi: EndpointAbiJson) -> Self {
        EndpointAbi::from(&abi)
    }
}

impl From<&ConstructorAbiJson> for EndpointAbi {
    fn from(abi: &ConstructorAbiJson) -> Self {
        EndpointAbi {
            docs: abi.docs.iter().map(|d| d.to_string()).collect(),
            name: "".to_string(),
            only_owner: false,
            only_admin: false,
            mutability: EndpointMutabilityAbi::Mutable,
            payable_in_tokens: abi
                .payable_in_tokens
                .iter()
                .map(|d| d.to_string())
                .collect(),
            inputs: abi.inputs.iter().map(InputAbi::from).collect(),
            outputs: abi.outputs.iter().map(OutputAbi::from).collect(),
            labels: vec![],
            allow_multiple_var_args: false,
            rust_method_name: "".to_string(),
            title: None,
            endpoint_type: EndpointTypeAbi::Init,
        }
    }
}

impl From<ConstructorAbiJson> for EndpointAbi {
    fn from(abi: ConstructorAbiJson) -> Self {
        EndpointAbi::from(&abi)
    }
}
