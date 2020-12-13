use super::*;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct ContractAbi {
	pub docs: &'static [&'static str],
	pub name: &'static str,
	pub endpoints: Vec<EndpointAbi>,
}

impl ContractAbi {
	pub fn coalesce(&mut self, other: Self) {
		self.endpoints.extend_from_slice(other.endpoints.as_slice());
	}
}

pub enum EndpointMutability {
	Pure,
	Readonly,
	Mutable,
}

#[derive(Clone, Debug)]
pub struct EndpointAbi {
	pub docs: &'static [&'static str],
	pub name: &'static str,
	pub payable: bool,
	pub inputs: Vec<InputAbi>,
	pub outputs: Vec<OutputAbi>,
}

#[derive(Clone, Debug)]
pub struct InputAbi {
	pub arg_name: &'static str,
	pub type_name: String,
	pub variable_num: bool,
}

#[derive(Clone, Debug)]
pub struct OutputAbi {
	pub type_name: String,
	pub variable_num: bool,
}

impl EndpointAbi {
	pub fn add_input<T: TypeAbi>(&mut self, arg_name: &'static str) {
		self.inputs.push(InputAbi {
			arg_name,
			type_name: T::type_name(),
			variable_num: false,
		});
	}

	pub fn add_output<T: TypeAbi>(&mut self) {
		self.outputs.extend_from_slice(T::output_abis().as_slice());
	}
}
