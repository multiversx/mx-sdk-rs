use std::collections::{HashMap, HashSet};

pub type FunctionIndex = usize;

#[derive(Default, Debug, Clone)]
pub struct CallGraph {
    pub function_map: HashMap<FunctionIndex, FunctionInfo>,
    pub endpoints: HashMap<String, EndpointInfo>,
}

impl CallGraph {
    pub fn next_function_index(&self) -> FunctionIndex {
        self.function_map.len()
    }

    pub fn insert_function(&mut self, function_index: FunctionIndex, function_info: FunctionInfo) {
        self.function_map.insert(function_index, function_info);
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct FunctionInfo {
    pub called_function_indexes: HashSet<FunctionIndex>,
    pub forbidden_opcodes: HashSet<String>,
}

impl FunctionInfo {
    pub fn new() -> Self {
        Self {
            called_function_indexes: HashSet::new(),
            forbidden_opcodes: HashSet::new(),
        }
    }

    pub fn new_with_indexes(indexes: Vec<FunctionIndex>) -> Self {
        Self {
            called_function_indexes: indexes.into_iter().collect(),
            forbidden_opcodes: HashSet::new(),
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
