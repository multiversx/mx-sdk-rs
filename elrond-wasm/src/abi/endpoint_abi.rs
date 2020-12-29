use super::*;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Clone, Debug)]
pub struct InputAbi {
	pub arg_name: &'static str,
	pub type_name: String,
	pub multi_arg: bool,
}

#[derive(Clone, Debug)]
pub struct OutputAbi {
	pub output_name: &'static str,
	pub type_name: String,
	pub multi_result: bool,
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
			type_name: T::type_name(),
			multi_arg: T::is_multi_arg_or_result(),
		});
	}

	pub fn add_output<T: TypeAbi>(&mut self, output_names: &[&'static str]) {
		self.outputs
			.extend_from_slice(T::output_abis(output_names).as_slice());
	}
}
