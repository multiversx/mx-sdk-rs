mod builtin_func_exec;
mod change_owner_mock;
mod esdt_local_burn;
mod esdt_local_mint;
mod esdt_multi_transfer_mock;
mod esdt_nft_add_quantity_mock;
mod esdt_nft_burn_mock;
mod esdt_nft_create_mock;
mod esdt_nft_transfer_mock;
mod esdt_transfer_mock;
mod set_username_mock;
mod upgrade_contract;

pub use builtin_func_exec::execute_builtin_function_or_default;
