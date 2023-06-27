mod builtin_func_exec;
mod builtin_func_map;
mod builtin_func_role_check_wrapper;
mod builtin_func_trait;
mod esdt_nft;
mod general;
mod transfer;
pub mod vm_builtin_function_names;

pub use builtin_func_exec::{execute_builtin_function_or_default, init_builtin_functions};
pub use builtin_func_map::BuiltinFunctionMap;
pub use builtin_func_trait::{BuiltinFunction, BuiltinFunctionEsdtTransferInfo};
pub use vm_builtin_function_names as builtin_function_names;
