use super::*;
use alloc::vec::Vec;

#[derive(Clone, Debug)]
pub struct InputAbi {
	pub arg_name: &'static str,
	pub type_description: TypeDescription,
	pub variable_num: bool,
}

#[derive(Clone, Debug)]
pub struct OutputAbi {
	pub type_description: TypeDescription,
	pub variable_num: bool,
}

#[derive(Clone, Debug)]
pub struct EndpointAbi {
	pub docs: &'static [&'static str],
	pub name: &'static str,
	pub payable: bool,
	pub inputs: Vec<InputAbi>,
	pub outputs: Vec<OutputAbi>,
}

impl EndpointAbi {
	pub fn add_input<T: TypeAbi>(&mut self, arg_name: &'static str) {
		self.inputs.push(InputAbi {
			arg_name,
			type_description: T::type_description(),
			variable_num: false,
		});
	}

	pub fn add_output<T: TypeAbi>(&mut self) {
		self.outputs.extend_from_slice(T::output_abis().as_slice());
	}
}
