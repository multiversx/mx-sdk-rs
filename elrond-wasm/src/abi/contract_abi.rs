use alloc::vec::Vec;

#[derive(Debug)]
pub struct ContractAbi {
	pub docs: &'static [&'static str],
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
}
