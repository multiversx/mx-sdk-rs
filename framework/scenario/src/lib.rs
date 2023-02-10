#![allow(clippy::type_complexity)]
#![feature(exhaustive_patterns)]

mod facade;
pub mod scenario;
mod scenario_go_runner;
mod scenario_rs_runner;
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
pub use scenario_go_runner::run_go;
pub use scenario_rs_runner::run_rs;

use std::path::Path;

#[deprecated(
    since = "0.39.0",
    note = "Alias provided for backwards compatibility. Do replace `BlockchainMock` with `ScenarioWorld` after upgrading, though."
)]
pub type BlockchainMock = ScenarioWorld;

#[deprecated(
    since = "0.39.0",
    note = "The old scenario testing method. Rename to `run_go`."
)]
pub fn mandos_go<P: AsRef<Path>>(relative_path: P) {
    run_go(relative_path);
}

#[deprecated(
    since = "0.39.0",
    note = "The old scenario testing method. Rename to `run_rs`."
)]
pub fn mandos_rs<P: AsRef<Path>>(relative_path: P, world: ScenarioWorld) {
    run_rs(relative_path, world);
}
