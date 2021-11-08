use super::*;
use alloc::{string::String, vec::Vec};

#[derive(Debug, Default)]
pub struct ContractAbi {
    pub build_info: BuildInfoAbi,
    pub docs: &'static [&'static str],
    pub name: &'static str,
    pub constructor: Option<EndpointAbi>,
    pub endpoints: Vec<EndpointAbi>,
    pub has_callback: bool,
    pub type_descriptions: TypeDescriptionContainerImpl,
}

impl ContractAbi {
    pub fn coalesce(&mut self, other: Self) {
        self.endpoints.extend_from_slice(other.endpoints.as_slice());
        self.has_callback |= other.has_callback;
        self.type_descriptions.insert_all(&other.type_descriptions);
    }

    /// A type can provide more than 1 type descripions.
    /// For instance, a struct can also provide the descriptions of its fields.
    pub fn add_type_descriptions<T: TypeAbi>(&mut self) {
        T::provide_type_descriptions(&mut self.type_descriptions);
    }

    /// Crate name, but with underscores instead of dashes.
    pub fn get_module_name(&self) -> String {
        self.build_info
            .contract_crate
            .name
            .replace('-', "_")
            .to_lowercase()
    }
}
