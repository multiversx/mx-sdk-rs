mod install_scenario_go;

use crate::cli_args::InstallArgs;

pub fn install(args: &InstallArgs) {
    if args.scenario_go {
        install_scenario_go::ScenarioGoInstaller::new(args.tag.clone()).install();
    }
}
