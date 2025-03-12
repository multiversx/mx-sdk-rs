pub mod builtin_function_mocks;
pub mod crypto_functions;
pub mod display_util;
pub mod host;
pub mod system_sc;
pub mod types;
pub mod vm_err_msg;
pub mod with_shared;
pub mod world_mock;

pub use world_mock::BlockchainMock;

#[cfg(feature = "wasmer")]
pub mod wasmer;

// Re-exporting the executor, for convenience.
pub use multiversx_chain_vm_executor as executor;

// Re-exporting the VM-core, for convenience.
pub use multiversx_chain_core as chain_core;

#[macro_use]
extern crate alloc;
