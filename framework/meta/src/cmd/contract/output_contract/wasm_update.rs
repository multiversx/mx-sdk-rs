use std::process::Command;

use super::OutputContract;

impl OutputContract {
    /// Runs `cargo update` in the corresponding wasm crate.
    pub fn cargo_update(&self) {
        let exit_status = Command::new("cargo")
            .args(["update"])
            .current_dir(self.wasm_crate_path())
            .spawn()
            .expect("failed to spawn contract update process")
            .wait()
            .expect("contract update process was not running");

        assert!(exit_status.success(), "contract update process failed");
    }
}
