mod wasmer5_alt_executor;
mod wasmer_alt_executor_err;
mod wrapped_instance;

pub use wasmer_alt_executor_err::WasmerAltExecutorFileNotFoundError;
pub use wrapped_instance::WrappedInstance;

#[cfg(feature = "wasmer")]
mod wasmer_alt_executor;

#[cfg(feature = "wasmer")]
pub use wasmer_alt_executor::WasmerAltExecutor;

#[cfg(not(feature = "wasmer"))]
mod wasmer_alt_executor_disabled;

#[cfg(not(feature = "wasmer"))]
pub use wasmer_alt_executor_disabled::WasmerAltExecutor;

pub use wasmer5_alt_executor::Wasmer5Executor;
