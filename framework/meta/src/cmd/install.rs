mod install_scenario_go;
mod install_wasm_tools;
mod system_info;

use crate::cli::{
    InstallArgs, InstallCommand, InstallMxScenarioGoArgs, InstallWasm32Args, InstallWasmOptArgs,
};

use self::install_scenario_go::ScenarioGoInstaller;

pub async fn install(args: &InstallArgs) {
    let command = args
        .command
        .as_ref()
        .expect("command expected after `install`");

    match command {
        InstallCommand::All => {
            install_scenario_go(&InstallMxScenarioGoArgs::default()).await;
            install_wasm32(&InstallWasm32Args::default());
            install_wasm_opt(&InstallWasmOptArgs::default());
        },
        InstallCommand::MxScenarioGo(sg_args) => install_scenario_go(sg_args).await,
        InstallCommand::Wasm32(wam32_args) => install_wasm32(wam32_args),
        InstallCommand::WasmOpt(wasm_opt_args) => install_wasm_opt(wasm_opt_args),
    }
}

async fn install_scenario_go(sg_args: &InstallMxScenarioGoArgs) {
    ScenarioGoInstaller::new(sg_args.tag.clone())
        .install()
        .await;
}

fn install_wasm32(_wasm32_args: &InstallWasm32Args) {
    install_wasm_tools::install_wasm32_target();
}

fn install_wasm_opt(_wasm_opt_args: &InstallWasmOptArgs) {
    install_wasm_tools::install_wasm_opt();
}
