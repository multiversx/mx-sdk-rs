use super::*;
use alloc::{string::String, vec::Vec};

#[derive(Debug, Default, Clone)]
pub struct ContractAbi {
    pub build_info: BuildInfoAbi,
    pub docs: &'static [&'static str],
    pub name: &'static str,
    pub constructors: Vec<EndpointAbi>,
    pub endpoints: Vec<EndpointAbi>,
    pub promise_callbacks: Vec<EndpointAbi>,
    pub events: Vec<EventAbi>,
    pub has_callback: bool,
    pub type_descriptions: TypeDescriptionContainerImpl,
}

impl ContractAbi {
    pub fn coalesce(&mut self, other: Self) {
        self.constructors
            .extend_from_slice(other.constructors.as_slice());
        self.endpoints.extend_from_slice(other.endpoints.as_slice());
        self.events.extend_from_slice(other.events.as_slice());
        self.promise_callbacks
            .extend_from_slice(other.promise_callbacks.as_slice());
        self.has_callback |= other.has_callback;
        self.type_descriptions.insert_all(&other.type_descriptions);
    }

    /// A type can provide more than 1 type descripions.
    /// For instance, a struct can also provide the descriptions of its fields.
    pub fn add_type_descriptions<T: TypeAbi>(&mut self) {
        T::provide_type_descriptions(&mut self.type_descriptions);
    }

    /// Contract main crate name.
    pub fn get_crate_name(&self) -> &str {
        self.build_info.contract_crate.name
    }

    /// Contract main crate name, but with underscores instead of dashes.
    pub fn get_crate_name_for_code(&self) -> String {
        self.get_crate_name().replace('-', "_").to_lowercase()
    }

    pub fn generate_with_endpoints(endpoints: Vec<EndpointAbi>) -> Self {
        ContractAbi {
            endpoints,
            ..Default::default()
        }
    }

    /// All exported functions: init, endpoints, promises callbacks.
    pub fn iter_all_exports(&self) -> impl Iterator<Item = &EndpointAbi> {
        self.constructors
            .iter()
            .chain(self.endpoints.iter())
            .chain(self.promise_callbacks.iter())
    }
}
