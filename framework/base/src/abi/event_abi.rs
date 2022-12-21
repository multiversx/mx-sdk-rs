use super::*;
use alloc::vec::Vec;

#[derive(Clone, Debug)]
pub struct EventInputAbi {
    pub arg_name: &'static str,
    pub type_name: TypeName,
    pub indexed: bool,
}

#[derive(Clone, Debug)]
pub struct EventAbi {
    pub docs: &'static [&'static str],
    pub identifier: &'static str,
    pub inputs: Vec<EventInputAbi>,
}

impl EventAbi {
    pub fn add_input<T: TypeAbi>(&mut self, arg_name: &'static str, indexed: bool) {
        self.inputs.push(EventInputAbi {
            arg_name,
            type_name: T::type_name(),
            indexed,
        });
    }
}
