mod executor_impl_err;

pub use executor_impl_err::ExecutorFileNotFoundError;

#[cfg(feature = "wasmer-prod")]
mod wasmer_prod;
#[cfg(feature = "wasmer-prod")]
pub use wasmer_prod::WasmerProdExecutor;

#[cfg(not(feature = "wasmer-prod"))]
mod wasmer_prod_executor_disabled;
#[cfg(not(feature = "wasmer-prod"))]
pub use wasmer_prod_executor_disabled::WasmerProdExecutor;

#[cfg(feature = "wasmer-experimental")]
mod we_executor;
#[cfg(feature = "wasmer-experimental")]
pub use we_executor::new_experimental_executor;

#[cfg(not(feature = "wasmer-experimental"))]
mod we_executor_disabled;
#[cfg(not(feature = "wasmer-experimental"))]
pub use we_executor_disabled::new_experimental_executor;
