use std::process::Command;

use super::OutputContract;

impl OutputContract {
    /// Runs `cargo clean` in the corresponding wasm crate.
    pub fn cargo_clean(&self) {
        let exit_status = Command::new("cargo")
            .args(["clean"])
            .current_dir(self.wasm_crate_path())
            .spawn()
            .expect("failed to spawn contract clean process")
            .wait()
            .expect("contract clean process was not running");

        assert!(exit_status.success(), "contract clean process failed");
    }
}
