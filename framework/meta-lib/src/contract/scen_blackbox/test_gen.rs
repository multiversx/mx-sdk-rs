use colored::Colorize;
use multiversx_sc::abi::ContractAbi;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::cargo_toml::CargoTomlContents;

use super::scenario_loader::{self, ScenarioFile, scenario_to_function_name};

/// Context for test generation, holding shared parameters and file reference
pub struct TestGenerator<'a> {
    pub crate_name: String,
    pub abi: ContractAbi,
    pub file: &'a mut File,
    /// Maps creator address to expected new address from setState.newAddresses
    pub new_address_map: HashMap<String, String>,
}

impl<'a> TestGenerator<'a> {
    pub fn new(crate_name: String, abi: ContractAbi, file: &'a mut File) -> Self {
        Self {
            crate_name,
            abi,
            file,
            new_address_map: HashMap::new(),
        }
    }

    /// Generates the combined test content to the file
    fn generate_combined_test_content(&mut self, scenario_files: &[ScenarioFile]) {
        // Write file header
        writeln!(self.file, "// Auto-generated blackbox tests from scenarios").unwrap();
        writeln!(self.file).unwrap();

        // Write imports
        writeln!(self.file, "use multiversx_sc_scenario::imports::*;").unwrap();
        writeln!(self.file).unwrap();
        writeln!(self.file, "use {}::*;", self.crate_name).unwrap();
        writeln!(self.file).unwrap();
        writeln!(
            self.file,
            "const CODE_PATH: MxscPath = MxscPath::new(\"output/{}.mxsc.json\");",
            self.crate_name
        )
        .unwrap();
        writeln!(self.file).unwrap();

        // Generate world() function
        writeln!(self.file, "fn world() -> ScenarioWorld {{").unwrap();
        writeln!(self.file, "    todo!()").unwrap();
        writeln!(self.file, "}}").unwrap();
        writeln!(self.file).unwrap();

        // Generate a test function and steps function for each scenario
        for scenario_file in scenario_files {
            self.generate_scenario_test(scenario_file);
            writeln!(self.file).unwrap();
        }
    }

    /// Generates test and steps functions for a single scenario
    fn generate_scenario_test(&mut self, scenario_file: &ScenarioFile) {
        let scenario = &scenario_file.scenario;
        let test_name = scenario_to_function_name(&scenario_file.file_name);
        let steps_function_name = format!("{}_steps", test_name);

        // Write scenario comment if available
        if let Some(comment) = &scenario.comment {
            writeln!(self.file, "// {}", comment).unwrap();
        }

        // Write test function
        writeln!(self.file, "#[test]").unwrap();
        writeln!(self.file, "fn {}() {{", test_name).unwrap();
        writeln!(self.file, "    let mut world = world();").unwrap();
        writeln!(self.file, "    {}(&mut world);", steps_function_name).unwrap();
        writeln!(self.file, "}}").unwrap();
        writeln!(self.file).unwrap();

        // Write steps function
        writeln!(
            self.file,
            "pub fn {}(world: &mut ScenarioWorld) {{",
            steps_function_name
        )
        .unwrap();

        // Generate code for each step
        for step in &scenario.steps {
            self.generate_step_code(step);
        }

        writeln!(self.file, "}}").unwrap();
    }
}

/// Main entry point for blackbox test generation
pub fn generate_scen_blackbox_tests(overwrite: bool, abi: &ContractAbi) {
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

    // Create test file
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
        println!("{}", "Blackbox test generation complete!".green());
        return;
    }

    let mut file = File::create(&test_file_path)
        .unwrap_or_else(|_| panic!("Failed to create test file: {}", test_file_path.display()));

    let mut generator = TestGenerator::new(crate_name, abi.clone(), &mut file);
    generator.generate_combined_test_content(&scenario_files);

    println!("  {} {}", "Generated:".green(), test_file_name);
    println!("{}", "Blackbox test generation complete!".green());
}
