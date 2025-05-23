use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct FunctionInfo {
    pub indexes: HashSet<usize>,
    pub forbidden_opcodes: HashSet<String>,
}

impl FunctionInfo {
    pub fn new() -> Self {
        Self {
            indexes: HashSet::new(),
            forbidden_opcodes: HashSet::new(),
        }
    }

    pub fn new_with_indexes(indexes: Vec<usize>) -> Self {
        Self {
            indexes: indexes.into_iter().collect(),
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

    pub fn add_function_index(&mut self, index: usize) {
        self.indexes.insert(index);
    }
}

#[derive(Default, Debug, Clone)]
pub struct EndpointInfo {
    pub index: usize,
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
