#[cfg(feature = "wasmer-experimental")]
mod we_executor;
#[cfg(feature = "wasmer-experimental")]
pub use we_executor::new_experimental_executor;

#[cfg(not(feature = "wasmer-experimental"))]
mod we_executor_disabled;
#[cfg(not(feature = "wasmer-experimental"))]
pub use we_executor_disabled::new_experimental_executor;
