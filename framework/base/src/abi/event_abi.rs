use super::*;
use alloc::{
    string::{String, ToString},
    vec::Vec,
};

#[derive(Clone, Debug)]
pub struct EventInputAbi {
    pub arg_name: String,
    pub type_name: TypeName,
    pub indexed: bool,
}

#[derive(Clone, Debug)]
pub struct EventAbi {
    pub docs: Vec<String>,
    pub identifier: String,
    pub inputs: Vec<EventInputAbi>,
}

impl EventAbi {
    /// Used in code generation.
    pub fn new(docs: &[&str], identifier: &str) -> Self {
        EventAbi {
            docs: docs.iter().map(|s| s.to_string()).collect(),
            identifier: identifier.to_string(),
            inputs: Vec::new(),
        }
    }

    /// Used in code generation.
    pub fn add_input<T: TypeAbi>(&mut self, arg_name: &str, indexed: bool) {
        self.inputs.push(EventInputAbi {
            arg_name: arg_name.to_string(),
            type_name: T::type_name(),
            indexed,
        });
    }
}
