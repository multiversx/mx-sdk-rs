use crate::{multiversx_chain_vm::scenario::executor::parse_execute_mandos_steps, ScenarioWorld};
use std::path::Path;

/// Runs scenario test using the Rust infrastructure and the debug mode.
/// Uses a contract map to replace the references to the wasm bytecode
/// with the contracts running in debug mode.
pub fn run_rs<P: AsRef<Path>>(relative_path: P, mut world: ScenarioWorld) {
    let mut absolute_path = world.blockchain_mock.current_dir.clone();
    absolute_path.push(relative_path);
    parse_execute_mandos_steps(absolute_path.as_ref(), &mut world.blockchain_mock);
}
