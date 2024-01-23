mod install_scenario_go;
mod system_info;

use crate::cli_args::{InstallArgs, InstallCommand, InstallMxScenarioGoArgs};

use self::install_scenario_go::ScenarioGoInstaller;

pub fn install(args: &InstallArgs) {
    let command = args
        .command
        .as_ref()
        .expect("command expected after `install`");

    match command {
        InstallCommand::All => {
            install_scenario_go(&InstallMxScenarioGoArgs::default());
        },
        InstallCommand::MxScenarioGo(sg_args) => install_scenario_go(sg_args),
    }
}

fn install_scenario_go(sg_args: &InstallMxScenarioGoArgs) {
    ScenarioGoInstaller::new(sg_args.tag.clone()).install();
}
