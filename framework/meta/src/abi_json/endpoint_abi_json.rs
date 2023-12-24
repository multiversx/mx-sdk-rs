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
            type_name: abi.type_name.clone(),
            multi_arg: if abi.multi_arg { Some(true) } else { None },
        }
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
            type_name: abi.type_name.clone(),
            multi_result: if abi.multi_result { Some(true) } else { None },
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EndpointMutabilityAbiJson {
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

    #[serde(rename = "onlyOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_owner: Option<bool>,

    #[serde(rename = "onlyAdmin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_admin: Option<bool>,

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
