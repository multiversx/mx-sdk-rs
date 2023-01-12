mod facade;
mod scenario_go_runner;
mod scenario_rs_runner;
pub mod whitebox;

use std::path::Path;

/// Keeping this for backwards compatibility.
/// Unfortunately, the `deprecated` annotation doesn't function for reexports.
pub use whitebox as testing_framework;

pub use multiversx_chain_vm::{bech32, mandos_system, num_bigint, ContractInfo, DebugApi};

pub use multiversx_chain_vm::{self, multiversx_sc, scenario_format};

pub use facade::ScenarioWorld;
pub use scenario_go_runner::run_go;
pub use scenario_rs_runner::run_rs;

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
