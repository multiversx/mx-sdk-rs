use std::process::Command;

use colored::Colorize;

pub fn format_command(command: &Command) -> String {
    let mut result = String::new();
    for (key, opt_value) in command.get_envs() {
        if let Some(value) = opt_value {
            result +=
                format!("{}=\"{}\" ", key.to_string_lossy(), value.to_string_lossy()).as_str();
        }
    }
    result.push_str(command.get_program().to_string_lossy().as_ref());

    for arg in command.get_args() {
        result.push(' ');
        result.push_str(arg.to_string_lossy().as_ref());
    }

    result
}

pub fn print_build_command(contract_name: String, command: &Command) {
    let path = command
        .get_current_dir()
        .expect("missing command dir")
        .canonicalize()
        .expect("command dir canonicalization failed");
    println!(
        "{}\n{}",
        format!("Building {} in {} ...", contract_name, path.display()).green(),
        format_command(command).green(),
    );
}
