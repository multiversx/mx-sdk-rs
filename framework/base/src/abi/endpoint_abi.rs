use super::*;
use alloc::{
    string::{String, ToString},
    vec::Vec,
};

#[derive(Clone, Debug)]
pub struct InputAbi {
    pub arg_name: String,
    pub type_name: TypeName,
    // pub original_type_name: TypeName,
    pub multi_arg: bool,
}

#[derive(Clone, Debug)]
pub struct OutputAbi {
    pub output_name: String,
    pub type_name: TypeName,
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
    Endpoint,
    PromisesCallback,
}

#[derive(Clone, Default, Debug)]
pub struct EndpointAbi {
    pub docs: Vec<String>,
    pub name: String,
    pub rust_method_name: String,
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
    ///
    /// TODO: replace with builder pattern to gt rid of the too many arguments.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        docs: &[&str],
        name: &str,
        rust_method_name: &str,
        only_owner: bool,
        only_admin: bool,
        mutability: EndpointMutabilityAbi,
        endpoint_type: EndpointTypeAbi,
        payable_in_tokens: &[&str],
        labels: &[&str],
        allow_multiple_var_args: bool,
    ) -> Self {
        EndpointAbi {
            docs: docs.iter().map(|s| s.to_string()).collect(),
            name: name.to_string(),
            rust_method_name: rust_method_name.to_string(),
            only_owner,
            only_admin,
            labels: labels.iter().map(|s| s.to_string()).collect(),
            endpoint_type,
            mutability,
            payable_in_tokens: payable_in_tokens.iter().map(|s| s.to_string()).collect(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            allow_multiple_var_args,
        }
    }

    pub fn add_input<T: TypeAbi>(&mut self, arg_name: &str) {
        self.inputs.push(InputAbi {
            arg_name: arg_name.to_string(),
            type_name: T::type_name(),
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
