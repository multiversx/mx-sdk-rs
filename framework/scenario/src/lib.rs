mod scenario_go_runner;
mod scenario_rs_runner;
mod scenario_world;
mod scenario_world_steps;
pub mod whitebox;

/// Keeping this for backwards compatibility.
/// Unfortunately, the `deprecated` annotation doesn't function for reexports.
pub use whitebox as testing_framework;

pub use multiversx_chain_vm::{bech32, mandos_system, num_bigint, ContractInfo, DebugApi};

pub use multiversx_chain_vm::{self, multiversx_sc, scenario_format};

pub use scenario_go_runner::scenario_go;
pub use scenario_rs_runner::scenario_rs;
pub use scenario_world::ScenarioWorld;

#[deprecated(
    since = "0.39.0",
    note = "Alias provided for backwards compatibility. Do replace `BlockchainMock` with `ScenarioWorld` after upgrading, though."
)]
pub type BlockchainMock = ScenarioWorld;
