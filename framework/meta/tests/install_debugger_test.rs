use multiversx_sc_meta::cmd::install::install_debugger;
use multiversx_sc_meta_lib::tools::find_current_workspace;

const INSTALL_DEBUGGER_TEMP_DIR_NAME: &str = "install-debugger-test";

#[test]
fn test_install_debugger() {
    let workspace_path = find_current_workspace().unwrap();
    let target_path = workspace_path.join(INSTALL_DEBUGGER_TEMP_DIR_NAME);
    install_debugger::install_debugger(Option::None);
}
