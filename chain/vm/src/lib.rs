pub mod blockchain;
pub mod builtin_functions;
pub mod crypto_functions;
pub mod display_util;
pub mod host;
pub mod system_sc;
pub mod types;
pub mod vm_err_msg;
pub mod wasmer;
pub mod with_shared;

pub use blockchain::BlockchainMock;

// Re-exporting the executor, for convenience.
pub use multiversx_chain_vm_executor as executor;

// Re-exporting the VM-core, for convenience.
pub use multiversx_chain_core as chain_core;

#[macro_use]
extern crate alloc;
