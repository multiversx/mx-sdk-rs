mod wasmer_alt_executor_err;
mod we_executor;

pub use wasmer_alt_executor_err::WasmerAltExecutorFileNotFoundError;

#[cfg(feature = "wasmer")]
mod wasmer_alt_executor;
#[cfg(feature = "wasmer")]
mod wasmer_alt_instance;
#[cfg(feature = "wasmer")]
mod wasmer_alt_instance_state;

#[cfg(feature = "wasmer")]
pub use wasmer_alt_executor::WasmerAltExecutor;
#[cfg(feature = "wasmer")]
pub use wasmer_alt_instance::WasmerAltInstance;
#[cfg(feature = "wasmer")]
pub use wasmer_alt_instance_state::WasmerAltInstanceState;

#[cfg(not(feature = "wasmer"))]
mod wasmer_alt_executor_disabled;

#[cfg(not(feature = "wasmer"))]
pub use wasmer_alt_executor_disabled::WasmerAltExecutor;

pub use we_executor::ExperimentalExecutor;
