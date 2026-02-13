use super::*;
use alloc::{
    borrow::ToOwned,
    string::{String, ToString},
    vec::Vec,
};

#[derive(Clone, Debug)]
pub struct InputAbi {
    pub arg_name: String,
    pub type_names: TypeNames,
    pub multi_arg: bool,
}

#[derive(Clone, Debug)]
pub struct OutputAbi {
    pub output_name: String,
    pub type_names: TypeNames,
    pub multi_result: bool,
}

pub type OutputAbis = Vec<OutputAbi>;

#[derive(Clone, Default, Debug)]
pub enum EndpointMutabilityAbi {
    #[default]
    Mutable,
    Readonly,
    Pure,
}

#[derive(Clone, Default, Debug)]
pub enum EndpointTypeAbi {
    #[default]
    Init,
    Upgrade,
    Endpoint,
    PromisesCallback,
}

#[derive(Clone, Default, Debug)]
pub struct EndpointAbi {
    pub docs: Vec<String>,
    pub name: String,
    pub rust_method_name: String,
    pub title: Option<String>,
    pub only_owner: bool,
    pub only_admin: bool,
    pub labels: Vec<String>,
    pub endpoint_type: EndpointTypeAbi,
    pub mutability: EndpointMutabilityAbi,
    pub payable_in_tokens: Vec<String>,
    pub inputs: Vec<InputAbi>,
    pub outputs: OutputAbis,
    pub allow_multiple_var_args: bool,
}

impl EndpointAbi {
    /// Used in code generation.
    pub fn new(
        name: &str,
        rust_method_name: &str,
        mutability: EndpointMutabilityAbi,
        endpoint_type: EndpointTypeAbi,
    ) -> Self {
        EndpointAbi {
            docs: Vec::new(),
            name: name.to_string(),
            rust_method_name: rust_method_name.to_string(),
            only_owner: false,
            only_admin: false,
            labels: Vec::new(),
            endpoint_type,
            mutability,
            payable_in_tokens: Vec::new(),
            title: None,
            inputs: Vec::new(),
            outputs: Vec::new(),
            allow_multiple_var_args: false,
        }
    }

    pub fn with_docs(mut self, doc_line: &str) -> Self {
        self.docs.push(doc_line.to_owned());
        self
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_owned());
        self
    }

    pub fn with_only_owner(mut self) -> Self {
        self.only_owner = true;
        self
    }

    pub fn with_only_admin(mut self) -> Self {
        self.only_admin = true;
        self
    }

    pub fn with_allow_multiple_var_args(mut self) -> Self {
        self.allow_multiple_var_args = true;
        self
    }

    pub fn with_label(mut self, label: &str) -> Self {
        self.labels.push(label.to_owned());
        self
    }

    pub fn with_payable_token(mut self, token: &str) -> Self {
        self.payable_in_tokens.push(token.to_owned());
        self
    }

    pub fn add_input<T: TypeAbi>(&mut self, arg_name: &str) {
        self.inputs.push(InputAbi {
            arg_name: arg_name.to_string(),
            type_names: T::type_names(),
            multi_arg: T::is_variadic(),
        });
    }

    pub fn add_output<T: TypeAbi>(&mut self, output_names: &[&'static str]) {
        self.outputs
            .extend_from_slice(T::output_abis(output_names).as_slice());
    }

    pub fn endpoint_with_name_and_labels(
        name: &'static str,
        labels: &'static [&'static str],
    ) -> Self {
        EndpointAbi {
            name: name.to_string(),
            labels: labels.iter().map(|s| s.to_string()).collect(),
            endpoint_type: EndpointTypeAbi::Endpoint,
            ..Default::default()
        }
    }
}
