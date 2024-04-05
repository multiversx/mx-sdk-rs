mod install_scenario_go;
mod install_wasm32;
mod system_info;

use crate::cli_args::{InstallArgs, InstallCommand, InstallMxScenarioGoArgs, InstallWasm32Args};

use self::install_scenario_go::ScenarioGoInstaller;

pub fn install(args: &InstallArgs) {
    let command = args
        .command
        .as_ref()
        .expect("command expected after `install`");

    match command {
        InstallCommand::All => {
            install_scenario_go(&InstallMxScenarioGoArgs::default());
            install_wasm32(&InstallWasm32Args::default());
        },
        InstallCommand::MxScenarioGo(sg_args) => install_scenario_go(sg_args),
        InstallCommand::Wasm32(wam32_args) => install_wasm32(wam32_args),
    }
}

fn install_scenario_go(sg_args: &InstallMxScenarioGoArgs) {
    ScenarioGoInstaller::new(sg_args.tag.clone()).install();
}

fn install_wasm32(_wasm32_args: &InstallWasm32Args) {
    install_wasm32::install_wasm32_target();
}
