pub mod install_debugger;
mod install_scenario_go;
mod system_info;

use multiversx_sc_meta_lib::tools::{self, build_target::install_target};

use crate::cli::{
    InstallArgs, InstallCommand, InstallDebuggerArgs, InstallMxScenarioGoArgs, InstallWasm32Args,
    InstallWasmOptArgs,
};

use self::install_scenario_go::ScenarioGoInstaller;

pub async fn install(args: &InstallArgs) {
    // validated before, can unwrap directly
    let command = args.command.as_ref().unwrap();

    match command {
        InstallCommand::All => {
            install_scenario_go(&InstallMxScenarioGoArgs::default()).await;
            install_wasm32(&InstallWasm32Args::default());
            install_wasm_opt(&InstallWasmOptArgs::default());
            install_debugger(&InstallDebuggerArgs::default()).await;
        }
        InstallCommand::MxScenarioGo(sg_args) => install_scenario_go(sg_args).await,
        InstallCommand::Wasm32(wam32_args) => install_wasm32(wam32_args),
        InstallCommand::WasmOpt(wasm_opt_args) => install_wasm_opt(wasm_opt_args),
        InstallCommand::Debugger(debugger_args) => install_debugger(debugger_args).await,
    }
}

async fn install_scenario_go(sg_args: &InstallMxScenarioGoArgs) {
    ScenarioGoInstaller::new(sg_args.tag.clone())
        .install()
        .await;
}

fn install_wasm32(_wasm32_args: &InstallWasm32Args) {
    install_target(None, tools::build_target::WASM32_TARGET);
    if tools::build_target::is_wasm32v1_available() {
        install_target(None, tools::build_target::WASM32V1_TARGET);
    }
}

fn install_wasm_opt(_wasm_opt_args: &InstallWasmOptArgs) {
    tools::install_wasm_opt();
}

async fn install_debugger(_debugger_args: &InstallDebuggerArgs) {
    install_debugger::install_debugger(Option::None).await;
}
