use colored::Colorize;
use multiversx_chain_scenario_format::serde_raw::StepRaw;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::cargo_toml::CargoTomlContents;

use super::scenario_loader::{self, scenario_to_function_name, ScenarioFile};

/// Main entry point for blackbox test generation
pub fn generate_blackbox_tests(overwrite: bool) {
    let contract_path = Path::new("..");
    let scenarios_dir = contract_path.join("scenarios");

    if !scenarios_dir.exists() {
        println!(
            "{}",
            format!(
                "No scenarios folder found at {}, skipping blackbox test generation",
                scenarios_dir.display()
            )
            .yellow()
        );
        return;
    }

    let scenario_files = scenario_loader::load_scenario_files(&scenarios_dir);

    if scenario_files.is_empty() {
        println!(
            "{}",
            format!("No .scen.json files found in {}", scenarios_dir.display()).yellow()
        );
        return;
    }

    println!(
        "{}",
        format!(
            "Found {} scenario file(s), generating blackbox tests...",
            scenario_files.len()
        )
        .green()
    );

    let tests_dir = contract_path.join("tests");
    if !tests_dir.exists() {
        fs::create_dir_all(&tests_dir).expect("Failed to create tests directory");
    }

    // Get crate name from Cargo.toml
    let cargo_toml_path = contract_path.join("Cargo.toml");
    let cargo_toml = CargoTomlContents::load_from_file(&cargo_toml_path);
    let crate_name = cargo_toml.package_name().replace('-', "_");

    generate_single_test_file(&tests_dir, &crate_name, &scenario_files, overwrite);

    println!("{}", "Blackbox test generation complete!".green());
}

/// Generates a single test file containing all scenarios
fn generate_single_test_file(
    tests_dir: &Path,
    crate_name: &str,
    scenario_files: &[ScenarioFile],
    overwrite: bool,
) {
    let test_file_name = format!("{}_blackbox_from_scenarios.rs", crate_name);
    let test_file_path = tests_dir.join(&test_file_name);

    // Check if file exists and overwrite is not enabled
    if test_file_path.exists() && !overwrite {
        println!(
            "  {} {}",
            "Skipping".yellow(),
            format!(
                "{} (already exists, use --overwrite to replace)",
                test_file_name
            )
            .dimmed()
        );
        return;
    }

    let mut file = File::create(&test_file_path)
        .unwrap_or_else(|_| panic!("Failed to create test file: {}", test_file_path.display()));

    generate_combined_test_content(&mut file, scenario_files);

    println!("  {} {}", "Generated:".green(), test_file_name);
}

/// Generates the content of the combined test file
fn generate_combined_test_content(file: &mut File, scenario_files: &[ScenarioFile]) {
    // Write file header
    writeln!(file, "// Auto-generated blackbox tests from scenarios").unwrap();
    writeln!(file).unwrap();

    // Write imports
    writeln!(file, "use multiversx_sc_scenario::imports::*;").unwrap();
    writeln!(file).unwrap();

    // Generate a test function and steps function for each scenario
    for scenario_file in scenario_files {
        generate_scenario_test(file, scenario_file);
        writeln!(file).unwrap();
    }
}

/// Generates test and steps functions for a single scenario
fn generate_scenario_test(file: &mut File, scenario_file: &ScenarioFile) {
    let scenario = &scenario_file.scenario;
    let test_name = scenario_to_function_name(&scenario_file.file_name);
    let steps_function_name = format!("{}_steps", test_name);

    // Write scenario comment if available
    if let Some(comment) = &scenario.comment {
        writeln!(file, "// {}", comment).unwrap();
    }

    // Write test function
    writeln!(file, "#[test]").unwrap();
    writeln!(file, "fn {}() {{", test_name).unwrap();
    writeln!(file, "    let mut world = ScenarioWorld::new();").unwrap();
    writeln!(file, "    {}(&mut world);", steps_function_name).unwrap();
    writeln!(file, "}}").unwrap();
    writeln!(file).unwrap();

    // Write steps function
    writeln!(
        file,
        "pub fn {}(world: &mut ScenarioWorld) {{",
        steps_function_name
    )
    .unwrap();

    // Generate comment for each step
    for (i, step) in scenario.steps.iter().enumerate() {
        write_step_comment(file, step, i);
    }

    writeln!(file, "}}").unwrap();
}

/// Writes a comment describing a step
fn write_step_comment(file: &mut File, step: &StepRaw, index: usize) {
    let (step_type, step_id, step_comment) = match step {
        StepRaw::ExternalSteps { comment, path } => {
            // For ExternalSteps, we'll generate actual code, not just a comment
            return write_external_steps(file, path, comment.as_deref());
        }
        StepRaw::SetState { comment, .. } => {
            ("SetState".to_string(), None, comment.as_deref())
        }
        StepRaw::ScCall { id, tx_id, comment, .. } => {
            ("ScCall".to_string(), id.as_ref().or(tx_id.as_ref()).map(|s| s.as_str()), comment.as_deref())
        }
        StepRaw::ScQuery { id, tx_id, comment, .. } => {
            ("ScQuery".to_string(), id.as_ref().or(tx_id.as_ref()).map(|s| s.as_str()), comment.as_deref())
        }
        StepRaw::ScDeploy { id, tx_id, comment, .. } => {
            ("ScDeploy".to_string(), id.as_ref().or(tx_id.as_ref()).map(|s| s.as_str()), comment.as_deref())
        }
        StepRaw::Transfer { id, tx_id, comment, .. } => {
            ("Transfer".to_string(), id.as_ref().or(tx_id.as_ref()).map(|s| s.as_str()), comment.as_deref())
        }
        StepRaw::ValidatorReward { id, comment, .. } => {
            ("ValidatorReward".to_string(), id.as_deref(), comment.as_deref())
        }
        StepRaw::CheckState { comment, .. } => {
            ("CheckState".to_string(), None, comment.as_deref())
        }
        StepRaw::DumpState { comment, .. } => {
            ("DumpState".to_string(), None, comment.as_deref())
        }
    };

    write!(file, "    // Step {}: {}", index, step_type).unwrap();
    if let Some(id) = step_id {
        write!(file, " (id: {})", id).unwrap();
    }
    if let Some(comment) = step_comment {
        write!(file, " - {}", comment).unwrap();
    }
    writeln!(file).unwrap();
}

/// Writes code for an ExternalSteps step (calls another scenario's steps function)
fn write_external_steps(file: &mut File, path: &str, comment: Option<&str>) {
    if let Some(comment_text) = comment {
        writeln!(file, "    // {}", comment_text).unwrap();
    }

    // Extract the scenario file name from the path and convert to function name
    let scenario_name = std::path::Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(path);
    
    let steps_function_name = format!("{}_steps", scenario_to_function_name(scenario_name));
    
    writeln!(file, "    {}(world);", steps_function_name).unwrap();
}
