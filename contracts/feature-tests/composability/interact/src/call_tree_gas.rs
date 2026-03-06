use crate::call_tree_config::CallTreeConfig;

/// Gas consumed by the execution of any contract call, regardless of further calls.
const PER_EXECUTION_GAS: u64 = 7_000_000;

/// Additional gas reserved by the caller for each outgoing call it makes.
const PER_CALL_GAS: u64 = 3_000_000;

impl CallTreeConfig {
    /// Recursively computes the gas needed when calling contract `name`.
    ///
    /// Built bottom-up:
    /// - every node pays `PER_EXECUTION_GAS` for its own execution,
    /// - every outgoing call additionally costs `PER_CALL_GAS` for the caller.
    pub fn compute_gas_for(&self, name: &str) -> u64 {
        let contract = self
            .contracts
            .get(name)
            .unwrap_or_else(|| panic!("compute_gas_for: unknown contract '{name}'"));
        let children_gas: u64 = contract
            .calls
            .iter()
            .map(|call| PER_CALL_GAS + self.compute_gas_for(&call.to))
            .sum();
        PER_EXECUTION_GAS + children_gas
    }

    /// Fills all `gas_limit` fields (programmed calls + start calls) with
    /// bottom-up estimates and saves the result back to `path`.
    pub fn fill_gas_estimates(&mut self) {
        // Pre-compute values (immutable pass) before mutating.
        let updates: Vec<(String, Vec<(String, u64)>)> = self
            .contracts
            .iter()
            .map(|(name, contract)| {
                let child_updates: Vec<(String, u64)> = contract
                    .calls
                    .iter()
                    .map(|call| (call.to.clone(), self.compute_gas_for(&call.to)))
                    .collect();
                (name.clone(), child_updates)
            })
            .collect();

        for (name, child_updates) in updates {
            let contract = self.contracts.get_mut(&name).unwrap();
            for (call, (to, gas)) in contract.calls.iter_mut().zip(child_updates.iter()) {
                call.gas_limit = Some(*gas);
                println!("  {name} -> {to}: gas_limit = {gas}");
            }
        }

        let start_updates: Vec<(String, u64)> = self
            .start
            .iter()
            .map(|s| (s.to.clone(), self.compute_gas_for(&s.to)))
            .collect();

        for (start, (to, gas)) in self.start.iter_mut().zip(start_updates.iter()) {
            start.gas_limit = Some(*gas);
            println!("start '{to}': gas_limit = {gas}");
        }
    }
}
