mod build_info_abi;
mod endpoint_abi;
mod esdt_attribute_abi;
mod event_abi;

pub use build_info_abi::*;
pub use endpoint_abi::*;
pub use esdt_attribute_abi::EsdtAttributeAbi;
pub use event_abi::*;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::{TypeAbi, TypeDescriptionContainer, TypeDescriptionContainerImpl};

#[derive(Debug, Default, Clone)]
pub struct ContractAbi {
    pub build_info: BuildInfoAbi,
    pub docs: Vec<String>,
    pub name: String,
    pub constructors: Vec<EndpointAbi>,
    pub upgrade_constructors: Vec<EndpointAbi>,
    pub endpoints: Vec<EndpointAbi>,
    pub promise_callbacks: Vec<EndpointAbi>,
    pub events: Vec<EventAbi>,
    pub esdt_attributes: Vec<EsdtAttributeAbi>,
    pub has_callback: bool,
    pub type_descriptions: TypeDescriptionContainerImpl,
}

impl ContractAbi {
    /// Used in code generation.
    pub fn new(build_info: BuildInfoAbi, docs: &[&str], name: &str, has_callback: bool) -> Self {
        ContractAbi {
            build_info,
            docs: docs.iter().map(|s| s.to_string()).collect(),
            name: name.to_string(),
            constructors: Vec::new(),
            upgrade_constructors: Vec::new(),
            endpoints: Vec::new(),
            promise_callbacks: Vec::new(),
            events: Vec::new(),
            esdt_attributes: Vec::new(),
            has_callback,
            type_descriptions: TypeDescriptionContainerImpl::new(),
        }
    }

    pub fn coalesce(&mut self, other: Self) {
        self.constructors
            .extend_from_slice(other.constructors.as_slice());
        self.endpoints.extend_from_slice(other.endpoints.as_slice());
        self.upgrade_constructors
            .extend_from_slice(other.upgrade_constructors.as_slice());
        self.events.extend_from_slice(other.events.as_slice());
        self.promise_callbacks
            .extend_from_slice(other.promise_callbacks.as_slice());
        self.has_callback |= other.has_callback;
        self.type_descriptions.insert_all(&other.type_descriptions);
        self.esdt_attributes
            .extend_from_slice(other.esdt_attributes.as_slice());
    }

    /// A type can provide more than 1 type descripions.
    /// For instance, a struct can also provide the descriptions of its fields.
    pub fn add_type_descriptions<T: TypeAbi>(&mut self) {
        T::provide_type_descriptions(&mut self.type_descriptions);
    }

    /// Contract main crate name.
    pub fn get_crate_name(&self) -> &str {
        &self.build_info.contract_crate.name
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
            .chain(self.upgrade_constructors.iter())
            .chain(self.endpoints.iter())
            .chain(self.promise_callbacks.iter())
    }
}
