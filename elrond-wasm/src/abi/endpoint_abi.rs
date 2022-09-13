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

#[derive(Clone, Debug)]
pub enum EndpointMutabilityAbi {
    Mutable,
    Readonly,
    Pure,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum EndpointLocationAbi {
    MainContract,
    ViewContract,
}

#[derive(Clone, Debug)]
pub struct EndpointAbi {
    pub docs: &'static [&'static str],
    pub name: &'static str,
    pub rust_method_name: &'static str,
    pub only_owner: bool,
    pub only_admin: bool,
    pub mutability: EndpointMutabilityAbi,
    pub location: EndpointLocationAbi,
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
}
