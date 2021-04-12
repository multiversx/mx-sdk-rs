use super::*;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct ContractAbi {
	pub docs: &'static [&'static str],
	pub name: &'static str,
	pub constructor: Option<EndpointAbi>,
	pub endpoints: Vec<EndpointAbi>,
	pub type_descriptions: TypeDescriptionContainerImpl,
}

impl ContractAbi {
	pub fn coalesce(&mut self, other: Self) {
		self.endpoints.extend_from_slice(other.endpoints.as_slice());
		self.type_descriptions.insert_all(&other.type_descriptions);
	}

	/// A type can provide more than 1 type descripions.
	/// For instance, a struct can also provide the descriptions of its fields.
	pub fn add_type_descriptions<T: TypeAbi>(&mut self) {
		T::provide_type_descriptions(&mut self.type_descriptions);
	}
}
