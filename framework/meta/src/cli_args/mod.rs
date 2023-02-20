mod cli_args_build;
mod cli_args_contract;
mod cli_args_standalone;

pub use cli_args_build::*;
pub use cli_args_contract::*;
pub use cli_args_standalone::*;

pub trait CliArgsToRaw {
    /// Converts to a list of raw arguments, as they would be called in a command.
    fn to_raw(&self) -> Vec<String>;
}
