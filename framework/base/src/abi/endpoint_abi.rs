use super::*;
use alloc::vec::Vec;

#[derive(Clone, Debug)]
pub struct InputAbi {
    pub arg_name: &'static str,
    pub type_name: TypeName,
    // pub original_type_name: TypeName,
    pub multi_arg: bool,
}

#[derive(Clone, Debug)]
pub struct OutputAbi {
    pub output_name: &'static str,
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
    pub docs: &'static [&'static str],
    pub name: &'static str,
    pub rust_method_name: &'static str,
    pub only_owner: bool,
    pub only_admin: bool,
    pub labels: &'static [&'static str],
    pub endpoint_type: EndpointTypeAbi,
    pub mutability: EndpointMutabilityAbi,
    pub payable_in_tokens: &'static [&'static str],
    pub inputs: Vec<InputAbi>,
    pub outputs: OutputAbis,
}

impl EndpointAbi {
    pub fn add_input<T: TypeAbi>(&mut self, arg_name: &'static str) {
        self.inputs.push(InputAbi {
            arg_name,
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
            name,
            labels,
            endpoint_type: EndpointTypeAbi::Endpoint,
            ..Default::default()
        }
    }
}
