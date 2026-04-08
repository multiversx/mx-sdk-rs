mod check_state_gen;
mod const_state;
mod contract_scenarios;
mod format_values;
mod num_format;
mod parse_abi;
mod scenario_loader;
mod set_state_gen;
mod step_code_gen;
mod test_generator;
mod tx_gen;

use crate::cli::ScenBlackboxArgs;
use crate::folder_structure::{RelevantDirectories, RelevantDirectory, dir_pretty_print};
use colored::Colorize;
use multiversx_sc::abi::ContractAbi;
use std::fs;
use std::path::Path;

pub use contract_scenarios::ContractScenarios;
pub use test_generator::TestGenerator;

pub fn scen_blackbox_tool(args: &ScenBlackboxArgs) {
    let path = if let Some(some_path) = &args.path {
        Path::new(some_path.as_str())
    } else {
        Path::new("./")
    };

    let dirs = RelevantDirectories::find_all(path, &args.ignore);
    let num_dirs = dirs.len();

    if num_dirs == 0 {
        println!("No contracts found");
        return;
    }

    println!("Generating blackbox tests for {num_dirs} contract(s) ...\n");

    let output_path = args.output.as_deref().map(Path::new);
    for dir in dirs.iter() {
        generate_for_contract(dir, args.overwrite, output_path);
    }
}

fn generate_for_contract(dir: &RelevantDirectory, overwrite: bool, output_path: Option<&Path>) {
    dir_pretty_print(std::iter::once(dir), "", &|_| {});

    // Read the contract ABI
    let dir_name = dir.dir_name();
    let output_abi_path = dir
        .path
        .join("output")
        .join(format!("{}.abi.json", dir_name));

    if !output_abi_path.exists() {
        println!(
            "  ⚠️  No ABI found at {}, skipping",
            output_abi_path.display()
        );
        return;
    }

    let abi_json = std::fs::read_to_string(&output_abi_path)
        .unwrap_or_else(|_| panic!("Failed to read ABI file: {}", output_abi_path.display()));

    let abi_json: multiversx_sc_meta_lib::abi_json::ContractAbiJson =
        serde_json::from_str(&abi_json)
            .unwrap_or_else(|_| panic!("Failed to parse ABI file: {}", output_abi_path.display()));

    let abi: multiversx_sc::abi::ContractAbi = abi_json.into();

    generate_scen_blackbox_tests(&dir.path, overwrite, &abi, output_path);
}

/// Main entry point for blackbox test generation
/// Assumes the current directory is the contract root directory
/// If `output_path` is provided, the generated file is written there;
/// otherwise the default path inside the contract's `tests/` folder is used.
pub fn generate_scen_blackbox_tests(
    contract_path: &Path,
    overwrite: bool,
    abi: &ContractAbi,
    output_path: Option<&Path>,
) {
    let input = match ContractScenarios::load(contract_path) {
        Some(i) => i,
        None => {
            println!(
                "{}",
                format!(
                    "No scenarios found at {}, skipping blackbox test generation",
                    contract_path.join("scenarios").display()
                )
                .yellow()
            );
            return;
        }
    };

    println!(
        "{}",
        format!(
            "Found {} scenario file(s), generating blackbox tests...",
            input.scenario_files.len()
        )
        .green()
    );

    let crate_name = input.crate_name;

    // Determine the output file path
    let test_file_path = if let Some(explicit) = output_path {
        explicit.to_path_buf()
    } else {
        let tests_dir = contract_path.join("tests");
        if !tests_dir.exists() {
            fs::create_dir_all(&tests_dir).expect("Failed to create tests directory");
        }
        let test_file_name = format!("{}_blackbox_from_scenarios.rs", crate_name);
        tests_dir.join(test_file_name)
    };

    let test_file_display = test_file_path
        .file_name()
        .unwrap_or(test_file_path.as_os_str())
        .to_string_lossy();

    // Check if file exists and overwrite is not enabled
    if test_file_path.exists() && !overwrite {
        println!(
            "  {} {}",
            "Skipping".yellow(),
            format!(
                "{} (already exists, use --overwrite to replace)",
                test_file_display
            )
            .dimmed()
        );
        println!("{}", "Blackbox test generation complete!".green());
        return;
    }

    if let Some(parent) = test_file_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .unwrap_or_else(|_| panic!("Failed to create directory: {}", parent.display()));
        }
    }

    let mut generator = TestGenerator::new(crate_name, abi.clone());
    let test_content = generator.generate_combined_test_content(&input.scenario_files);
    fs::write(&test_file_path, &test_content)
        .unwrap_or_else(|_| panic!("Failed to write test file: {}", test_file_path.display()));

    println!("  {} {}", "Generated:".green(), test_file_display);
    println!("{}", "Blackbox test generation complete!".green());
}
