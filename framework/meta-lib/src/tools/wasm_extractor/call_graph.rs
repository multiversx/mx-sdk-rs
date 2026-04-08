use std::collections::{BTreeMap, HashMap, HashSet};

use super::{EndpointInfo, EndpointName, FunctionIndex, FunctionInfo};

#[derive(Default, Debug, Clone)]
pub struct CallGraph {
    pub function_map: BTreeMap<FunctionIndex, FunctionInfo>,
    pub endpoints: HashMap<EndpointName, EndpointInfo>,
    pub table_functions: Vec<FunctionIndex>,
    pub function_endpoints: HashMap<FunctionIndex, HashSet<EndpointName>>,
    pub call_indirect_accessible_from_endpoints: HashSet<EndpointName>,
}

impl CallGraph {
    pub fn next_function_index(&self) -> FunctionIndex {
        self.function_map.len()
    }

    pub fn insert_function(&mut self, function_index: FunctionIndex, function_info: FunctionInfo) {
        self.function_map.insert(function_index, function_info);
    }

    pub fn get_function_calls(&self) -> Vec<(FunctionIndex, Vec<FunctionIndex>)> {
        let mut result = Vec::new();
        for (&func_index, func_info) in &self.function_map {
            let called_indexes = func_info.get_called_function_indexes();
            result.push((func_index, called_indexes));
        }
        result
    }

    fn mark_accessible_from_function_index(
        &mut self,
        from_index: FunctionIndex,
        to_index: FunctionIndex,
    ) {
        let Some(to_func_info) = self.function_map.get_mut(&to_index) else {
            return;
        };

        if to_func_info
            .accessible_from_function_indexes
            .contains(&from_index)
        {
            return;
        }

        to_func_info
            .accessible_from_function_indexes
            .insert(from_index);

        let called_function_indexes = to_func_info.get_called_function_indexes();
        for called_index in called_function_indexes {
            self.mark_accessible_from_function_index(from_index, called_index);
        }
    }

    pub fn populate_accessible_from_function_indexes(&mut self) {
        let mut from_to_pairs = Vec::new();
        for (&func_index, func_info) in &self.function_map {
            for called_function_index in &func_info.called_function_indexes {
                from_to_pairs.push((func_index, *called_function_index));
            }
        }

        for (from_index, to_index) in from_to_pairs {
            self.mark_accessible_from_function_index(from_index, to_index);
        }
    }

    pub fn mark_accessible_from_call_indirect(&mut self, to_index: FunctionIndex) {
        let Some(to_func_info) = self.function_map.get_mut(&to_index) else {
            return;
        };

        if to_func_info.accessible_from_call_indirect {
            return;
        }

        to_func_info.accessible_from_call_indirect = true;

        let called_function_indexes = to_func_info.get_called_function_indexes();
        for called_index in called_function_indexes {
            self.mark_accessible_from_call_indirect(called_index);
        }
    }

    pub fn populate_accessible_from_call_indirect(&mut self) {
        for table_func_index in self.table_functions.clone() {
            self.mark_accessible_from_call_indirect(table_func_index);
        }
    }

    pub fn populate_function_endpoints(&mut self) {
        for (endpoint_name, endpoint) in &self.endpoints {
            self.function_endpoints
                .entry(endpoint.index)
                .or_default()
                .insert(endpoint_name.clone());
        }
    }

    pub fn function_accessible_from_endpoints(
        &self,
        function_index: FunctionIndex,
    ) -> HashSet<EndpointName> {
        let mut result = self
            .function_endpoints
            .get(&function_index)
            .cloned()
            .unwrap_or_default();
        if let Some(func_info) = self.function_map.get(&function_index) {
            for from_index in &func_info.accessible_from_function_indexes {
                if let Some(endpoints) = self.function_endpoints.get(from_index) {
                    for endpoint in endpoints {
                        result.insert(endpoint.clone());
                    }
                }
            }
        }
        result
    }

    pub fn populate_call_indirect_accessible_from_endpoints(&mut self) {
        for (func_index, func_info) in &self.function_map {
            if func_info.contains_call_indirect {
                let endpoints = self.function_accessible_from_endpoints(*func_index);
                for endpoint in endpoints {
                    self.call_indirect_accessible_from_endpoints
                        .insert(endpoint.clone());
                }
            }
        }
    }
}
