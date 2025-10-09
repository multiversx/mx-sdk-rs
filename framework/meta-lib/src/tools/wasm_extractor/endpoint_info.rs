use std::collections::{BTreeSet, HashSet};

pub type FunctionIndex = usize;

pub type EndpointName = String;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct FunctionInfo {
    pub called_function_indexes: BTreeSet<FunctionIndex>,
    pub contains_call_indirect: bool,
    pub forbidden_opcodes: HashSet<String>,
    pub accessible_from_function_indexes: HashSet<FunctionIndex>,
    pub accessible_from_call_indirect: bool,
}

impl FunctionInfo {
    pub fn new() -> Self {
        Self {
            called_function_indexes: BTreeSet::new(),
            contains_call_indirect: false,
            forbidden_opcodes: HashSet::new(),
            accessible_from_function_indexes: HashSet::new(),
            accessible_from_call_indirect: false,
        }
    }

    pub fn new_with_indexes(indexes: Vec<FunctionIndex>) -> Self {
        Self {
            called_function_indexes: indexes.into_iter().collect(),
            contains_call_indirect: false,
            forbidden_opcodes: HashSet::new(),
            accessible_from_function_indexes: HashSet::new(),
            accessible_from_call_indirect: false,
        }
    }

    pub fn add_forbidden_opcode(&mut self, opcode: String) {
        self.forbidden_opcodes.insert(opcode);
    }

    pub fn add_forbidden_opcodes(&mut self, opcodes: HashSet<String>) {
        for opcode in opcodes {
            self.forbidden_opcodes.insert(opcode.to_string());
        }
    }

    pub fn add_called_function(&mut self, called_function_index: FunctionIndex) {
        self.called_function_indexes.insert(called_function_index);
    }

    pub fn get_called_function_indexes(&self) -> Vec<FunctionIndex> {
        self.called_function_indexes.iter().cloned().collect()
    }
}

#[derive(Default, Debug, Clone)]
pub struct EndpointInfo {
    pub index: FunctionIndex,
    pub readonly: bool,
    pub forbidden_opcodes: HashSet<String>,
}

impl EndpointInfo {
    pub fn default(readonly: bool) -> Self {
        Self {
            index: 0usize,
            readonly,
            forbidden_opcodes: HashSet::new(),
        }
    }

    pub fn set_forbidden_opcodes(&mut self, opcodes: HashSet<String>) {
        self.forbidden_opcodes = opcodes;
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}
