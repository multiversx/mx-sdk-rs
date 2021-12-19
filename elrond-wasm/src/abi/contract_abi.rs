use super::*;
use alloc::{string::String, vec::Vec};

#[derive(Debug, Default, Clone)]
pub struct ContractAbi {
    pub build_info: BuildInfoAbi,
    pub docs: &'static [&'static str],
    pub name: &'static str,
    pub constructors: Vec<EndpointAbi>,
    pub endpoints: Vec<EndpointAbi>,
    pub has_callback: bool,
    pub type_descriptions: TypeDescriptionContainerImpl,
}

impl ContractAbi {
    pub fn coalesce(&mut self, other: Self) {
        self.constructors
            .extend_from_slice(other.constructors.as_slice());
        self.endpoints.extend_from_slice(other.endpoints.as_slice());
        self.has_callback |= other.has_callback;
        self.type_descriptions.insert_all(&other.type_descriptions);
    }

    #[must_use]
    pub fn main_contract(&self) -> ContractAbi {
        ContractAbi {
            build_info: self.build_info.clone(),
            docs: self.docs,
            name: self.name,
            constructors: self.constructors.clone(),
            endpoints: self
                .endpoints
                .clone()
                .iter()
                .filter(|endpoint| endpoint.location == EndpointLocationAbi::MainContract)
                .cloned()
                .collect(),
            has_callback: self.has_callback,
            type_descriptions: self.type_descriptions.clone(),
        }
    }

    pub fn location_exists(&self, location: EndpointLocationAbi) -> bool {
        self.constructors
            .iter()
            .chain(self.endpoints.iter())
            .any(|endpoint| endpoint.location == location)
    }

    #[must_use]
    pub fn secondary_contract(&self, location: EndpointLocationAbi) -> ContractAbi {
        ContractAbi {
            build_info: self.build_info.clone(),
            docs: self.docs,
            name: self.name,
            constructors: Vec::new(),
            endpoints: self
                .endpoints
                .clone()
                .iter()
                .filter(|endpoint| endpoint.location == location)
                .cloned()
                .collect(),
            has_callback: false,
            type_descriptions: self.type_descriptions.clone(),
        }
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
}
