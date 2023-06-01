#![allow(clippy::type_complexity)]
#![feature(exhaustive_patterns)]

pub mod api;
mod facade;
pub mod managed_test_util;
pub mod scenario;
pub mod standalone;
pub mod test_wallets;
mod vm_go_tool;
pub mod whitebox;

/// Keeping this for backwards compatibility.
/// Unfortunately, the `deprecated` annotation doesn't function for reexports.
pub use whitebox as testing_framework;

pub use multiversx_chain_vm::{self, bech32, num_bigint, DebugApi};

pub use multiversx_sc;

/// Exposing the scenario model. Might be moved in the future,
/// but the export will hopefully remain the same.
pub use crate::scenario::model as scenario_model;

/// For backwards compatibility, will be removed.
pub use crate::scenario as mandos_system;

// Re-exporting the whole mandos crate for easier use in tests.
pub use multiversx_chain_scenario_format as scenario_format;

pub use facade::{ContractInfo, ScenarioWorld};

use std::path::Path;

/// Legacy function for running a scenario test using the Go VM tool.
///
/// Use `sc-meta test-gen` to replace all calls to it automatically.
#[deprecated(
    since = "0.42.0",
    note = "Call `sc-meta test-gen` in the project folder to automatically upgrade all scenario tests."
)]
pub fn run_go<P: AsRef<Path>>(relative_path: P) {
    ScenarioWorld::vm_go().run(relative_path);
}

#[deprecated(
    since = "0.39.0",
    note = "Call `sc-meta test-gen` in the project folder to automatically upgrade all scenario tests."
)]
pub fn mandos_go<P: AsRef<Path>>(relative_path: P) {
    ScenarioWorld::vm_go().run(relative_path);
}

/// Legacy function for running a scenario test using the Go VM tool.
///
/// Use `sc-meta test-gen` to replace all calls to it automatically.
#[deprecated(
    since = "0.42.0",
    note = "Call `sc-meta test-gen` in the project folder to automatically upgrade all scenario tests."
)]
pub fn run_rs<P: AsRef<Path>>(relative_path: P, world: ScenarioWorld) {
    world.run(relative_path);
}

#[deprecated(
    since = "0.39.0",
    note = "Call `sc-meta test-gen` in the project folder to automatically upgrade all scenario tests."
)]
pub fn mandos_rs<P: AsRef<Path>>(relative_path: P, world: ScenarioWorld) {
    world.run(relative_path);
}

#[deprecated(
    since = "0.39.0",
    note = "Alias provided for backwards compatibility. Do replace `BlockchainMock` with `ScenarioWorld` after upgrading, though."
)]
pub type BlockchainMock = ScenarioWorld;
