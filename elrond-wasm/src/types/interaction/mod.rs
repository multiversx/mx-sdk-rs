mod arg_buffer;
mod arg_buffer_managed;
mod async_call;
mod callback_closure;
mod callback_closure_unmanaged_args;
mod callback_selector_result;
mod contract_call;
mod contract_deploy;

pub use arg_buffer::ArgBuffer;
pub use arg_buffer_managed::ManagedArgBuffer;
pub use async_call::AsyncCall;
pub use callback_closure::{new_callback_call, CallbackClosure, CallbackClosureMatcher};
pub use callback_closure_unmanaged_args::CallbackClosureUnmanagedArgs;
pub use callback_selector_result::CallbackSelectorResult;
pub use contract_call::{new_contract_call, ContractCall};
pub use contract_deploy::{new_contract_deploy, ContractDeploy};

#[cfg(feature = "cb_closure_managed_deser")]
pub type CallbackClosureForDeser<M> = CallbackClosure<M>;

#[cfg(not(feature = "cb_closure_managed_deser"))]
pub type CallbackClosureForDeser<M> = CallbackClosureUnmanagedArgs<M>;
