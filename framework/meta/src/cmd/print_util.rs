use colored::Colorize;
use std::path::Path;

pub fn print_all_count(num_contract_crates: usize) {
    println!(
        "\n{}",
        format!("Found {num_contract_crates} contract crates.").truecolor(128, 128, 128),
    );
}

pub fn print_all_index(contract_crates_index: usize, num_contract_crates: usize) {
    println!(
        "\n{}",
        format!("({contract_crates_index}/{num_contract_crates})").truecolor(128, 128, 128),
    );
}

pub fn print_all_command(meta_path: &Path, cargo_run_args: &[String]) {
    println!(
        "{} {}\n{} `cargo {}`",
        "In".green(),
        meta_path.display(),
        "Calling".green(),
        cargo_run_args.join(" "),
    );
}
