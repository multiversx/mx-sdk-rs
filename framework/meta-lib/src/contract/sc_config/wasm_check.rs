use super::ContractVariant;
use super::execute_command::{ExecuteCommandError, execute_spawn_command};
use crate::cli::CheckArgs;

impl ContractVariant {
    /// Runs `cargo check` in the corresponding wasm crate, for the wasm target.
    pub fn cargo_check(&self, check_args: &CheckArgs) -> Result<(), ExecuteCommandError> {
        let mut command = self.compose_wasm_cargo_command(
            "check",
            check_args.locked,
            &check_args.target_dir_wasm,
        );
        if check_args.frozen {
            command.arg("--frozen");
        }
        if check_args.offline {
            command.arg("--offline");
        }

        execute_spawn_command(&mut command)?;
        Ok(())
    }
}
